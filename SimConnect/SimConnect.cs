using FlyByWireless.IO;
using System;
using System.Buffers;
using System.Collections.Concurrent;
using System.Collections.ObjectModel;
using System.IO;
using System.IO.Pipes;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

namespace FlyByWireless.SimConnect
{
    record Protocol(
        uint ProtocolVersion,
        Version SimConnectVersion
    );

    public sealed class SimConnect : IAsyncDisposable, IDisposable
    {
        static readonly ReadOnlyCollection<Protocol> _protocols = new(new Protocol[]
        {
            // new(4, new(1, 0, 20, 0)), // ESP
            new(4, new(10, 0, 62615, 0)), // SE
            new(4, new(10, 0, 61637, 0)), // SP2
            new(4, new(10, 0, 61472, 0)), // SP2 1A
            new(4, new(10, 0, 61259, 0)), // X-PACK
            new(3, new(10, 0, 61355, 0)), // SP1 1A
            new(3, new(10, 0, 61242, 0)), // SP1
            new(2, new(0, 0, 60905, 0)) // RTM
        });

        static void WriteFixedString(MemoryStream stream, string value, int width)
        {
            var buffer = stream.GetBuffer().AsSpan((int)stream.Position, width);
            buffer[(string.IsNullOrEmpty(value) ? 0 : Encoding.UTF8.GetBytes(value, buffer))..].Clear();
            stream.Position += width;
        }

        Stream ClientStream { get; } =
            new NamedPipeClientStream(".", @"Microsoft Flight Simulator\SimConnect", PipeDirection.InOut, PipeOptions.Asynchronous);

        CancellationTokenSource? _quit;

        uint _packetId = 0, _openPacketId, _protocol;

        readonly ConcurrentDictionary<uint, TaskCompletionSource<object>> _pending = new();

        public event EventHandler<ExternalException>? OnRecvException;

        public event EventHandler<RecvQuit>? OnRecvQuit;

        public async ValueTask DisposeAsync()
        {
            await using (ClientStream.ConfigureAwait(false))
                _quit?.Cancel();
        }

        public void Dispose()
        {
            using (ClientStream)
                _quit?.Cancel();
        }

        async ValueTask<uint> WriteAsync(Memory<byte> buffer, CancellationToken cancellationToken = default)
        {
            uint Id()
            {
                var send = MemoryMarshal.Cast<byte, uint>(buffer.Span);
                send[1] = _protocol;
                return send[3] = Interlocked.Increment(ref _packetId);
            }
            var id = Id();
            await ClientStream.WriteAsync(buffer, cancellationToken).ConfigureAwait(false);
            return id;
        }

        async Task ReadAsync(CancellationToken cancellationToken = default)
        {
            var buffer = ArrayPool<byte>.Shared.Rent(4);
            try
            {
                await ClientStream.ReadFullyAsync(buffer.AsMemory(0, 12), cancellationToken).ConfigureAwait(false);
                var size = BitConverter.ToInt32(buffer);
                if (buffer.Length < size)
                {
                    var old = buffer;
                    buffer = ArrayPool<byte>.Shared.Rent(size);
                    old.AsSpan(0, 12).CopyTo(buffer);
                    ArrayPool<byte>.Shared.Return(old);
                }
                await ClientStream.ReadFullyAsync(buffer.AsMemory(12, size - 12), cancellationToken).ConfigureAwait(false);
                void Parse()
                {
                    var s = buffer.AsSpan(0, size);
                    switch (MemoryMarshal.AsRef<Recv>(s).Id)
                    {
                        case RecvId.Null: throw new NotImplementedException();
                        case RecvId.Exception:
                            {
                                ref var r = ref MemoryMarshal.AsRef<RecvException>(s);
                                var ex = new ExternalException(r.Exception.ToString(), (int)r.Exception);
                                ex.Data.Add(nameof(r.Index), r.Index);
                                if (!(_pending.TryRemove(r.SendId, out var tcs) && tcs.TrySetException(ex)))
                                    OnRecvException?.Invoke(this, ex);
                            }
                            break;
                        case RecvId.Open:
                            {
                                if (_pending.TryRemove(_openPacketId, out var tcs))
                                    tcs.SetResult(MemoryMarshal.AsRef<RecvOpen>(s));
                            }
                            break;
                        case RecvId.Quit:
                            _quit?.Cancel();
                            OnRecvQuit?.Invoke(this, MemoryMarshal.AsRef<RecvQuit>(s));
                            break;
                        case RecvId.Event: throw new NotImplementedException();
                        case RecvId.EventObjectAddRemove: throw new NotImplementedException();
                        case RecvId.EventFileName: throw new NotImplementedException();
                        case RecvId.EventFrame: throw new NotImplementedException();
                        case RecvId.SimObjectData: throw new NotImplementedException();
                        case RecvId.SimObjectDataByType: throw new NotImplementedException();
                        case RecvId.WeatherObservation: throw new NotImplementedException();
                        case RecvId.CloudState: throw new NotImplementedException();
                        case RecvId.AssignedObjectId: throw new NotImplementedException();
                        case RecvId.ReservedKey: throw new NotImplementedException();
                        case RecvId.CustomAction: throw new NotImplementedException();
                        case RecvId.SystemState: throw new NotImplementedException();
                        case RecvId.ClientData: throw new NotImplementedException();
                        case RecvId.EventWeatherMode: throw new NotImplementedException();
                        case RecvId.AirportList: throw new NotImplementedException();
                        case RecvId.VorList: throw new NotImplementedException();
                        case RecvId.NdbList: throw new NotImplementedException();
                        case RecvId.WaypointList: throw new NotImplementedException();
                        case RecvId.EventMultiplayerServerStarted: throw new NotImplementedException();
                        case RecvId.EventMultiplayerClientStarted: throw new NotImplementedException();
                        case RecvId.EventMultiplayerSessionEnded: throw new NotImplementedException();
                        case RecvId.EventRaceEnd: throw new NotImplementedException();
                        case RecvId.EnentRaceLap: throw new NotImplementedException();
                    }
                }
                Parse();
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(buffer);
            }
        }

        // TODO: support TCP, remote server, and config
        public async Task<RecvOpen> OpenAsync(string applicationName, CancellationToken cancellationToken = default)
        {
            using (_quit) { }
            var lts = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken, (_quit = new()).Token);
            await ((NamedPipeClientStream)ClientStream).ConnectAsync(lts.Token).ConfigureAwait(false);
            int size;
            unsafe
            {
                size = sizeof(SendOpen);
            }
            ExternalException ee = null!;
            foreach (var protocol in _protocols)
            {
                _protocol = protocol.ProtocolVersion;
                TaskCompletionSource<object> tcs = new();
                {
                    var a = ArrayPool<byte>.Shared.Rent(size);
                    try
                    {
                        MemoryMarshal.AsRef<SendOpen>(a) = new(protocol, applicationName);
                        _pending.TryAdd(_openPacketId = await WriteAsync(a.AsMemory(0, size), lts.Token).ConfigureAwait(false), tcs);
                    }
                    finally
                    {
                        ArrayPool<byte>.Shared.Return(a);
                    }
                }
                await ReadAsync(lts.Token).ConfigureAwait(false);
                try
                {
                    var open = (RecvOpen)await tcs.Task.ConfigureAwait(false);
                    _ = Task.Factory.StartNew(async () =>
                    {
                        using (lts)
                        {
                            while (!lts.IsCancellationRequested)
                            {
                                try
                                {
                                    await ReadAsync(lts.Token).ConfigureAwait(false);
                                }
                                catch
                                {
                                    break;
                                }
                            }
                        }
                    }, TaskCreationOptions.LongRunning);
                    return open;
                }
                catch (ExternalException ex) when (ex.ErrorCode == (int)Exception.VersionMismatch)
                {
                    ee = ex;
                }
            }
            throw ee;
        }
    }
}