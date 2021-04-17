using FlyByWireless.IO;
using FlyByWireless.SimConnect.Data;
using System;
using System.Buffers;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.IO;
using System.IO.Pipes;
using System.Linq;
using System.Reflection;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using System.Threading.Channels;
using System.Threading.Tasks;

namespace FlyByWireless.SimConnect
{
    record Protocol(
        uint ProtocolVersion,
        Version SimConnectVersion
    );

    unsafe delegate void RecvSimObjectDataHandler(RecvSimObjectData* data);

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

        Task? _run;

        uint _packetId = 0, _protocol, _defineId, _requestId;

        readonly ConcurrentDictionary<uint, Type> _defineTypes = new();

        readonly ConcurrentDictionary<Type, uint> _defineIds = new();

        readonly ConcurrentDictionary<uint, TaskCompletionSource<object>> _pending = new();

        readonly ConcurrentDictionary<uint, RecvSimObjectDataHandler> _requests = new();

        public event EventHandler<ExternalException>? OnRecvException;

        public event EventHandler<RecvQuit>? OnRecvQuit;

        void Disposed()
        {
            _pending.Clear();
            _defineTypes.Clear();
            _defineIds.Clear();
        }

        public async ValueTask DisposeAsync()
        {
            try
            {
                await using (ClientStream.ConfigureAwait(false))
                {
                    _quit?.Cancel();
                    if (_run is Task r)
                        await r;
                }
            }
            catch (OperationCanceledException) { }
            catch (EndOfStreamException) { }
            catch (ObjectDisposedException) { }
            finally
            {
                Disposed();
            }
        }

        public void Dispose()
        {
            try
            {
                using (ClientStream)
                {
                    _quit?.Cancel();
                    _run?.GetAwaiter().GetResult();
                }
            }
            catch (OperationCanceledException) { }
            catch (EndOfStreamException) { }
            catch (ObjectDisposedException) { }
            finally
            {
                Disposed();
            }
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

        async ValueTask ReadAsync(CancellationToken cancellationToken = default)
        {
            var buffer = ArrayPool<byte>.Shared.Rent(12);
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
                                if (_pending.TryRemove(_packetId, out var tcs))
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
                        case RecvId.SimObjectData:
                        case RecvId.SimObjectDataByType:
                        case RecvId.ClientData:
                            unsafe
                            {
                                fixed (void* p = s)
                                {
                                    var d = (RecvSimObjectData*)p;
                                    if (_requests.TryGetValue(d->RequestId, out var callback))
                                        callback(d);
                                }
                            }
                            break;
                        case RecvId.WeatherObservation: throw new NotImplementedException();
                        case RecvId.CloudState: throw new NotImplementedException();
                        case RecvId.AssignedObjectId: throw new NotImplementedException();
                        case RecvId.ReservedKey: throw new NotImplementedException();
                        case RecvId.CustomAction: throw new NotImplementedException();
                        case RecvId.SystemState: throw new NotImplementedException();
                        case RecvId.EventWeatherMode: throw new NotImplementedException();
                        case RecvId.AirportList: throw new NotImplementedException();
                        case RecvId.VORList: throw new NotImplementedException();
                        case RecvId.NDBList: throw new NotImplementedException();
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
        public async ValueTask<RecvOpen> OpenAsync(string applicationName, CancellationToken cancellationToken = default)
        {
            using (_quit) { }
            var lts = CancellationTokenSource.CreateLinkedTokenSource(cancellationToken, (_quit = new()).Token);
            await ((NamedPipeClientStream)ClientStream).ConnectAsync(lts.Token).ConfigureAwait(false);
            ExternalException ee = null!;
            int size = Unsafe.SizeOf<SendOpen>();
            foreach (var protocol in _protocols)
            {
                _protocol = protocol.ProtocolVersion;
                TaskCompletionSource<object> tcs = new();
                {
                    var a = ArrayPool<byte>.Shared.Rent(size);
                    try
                    {
                        MemoryMarshal.AsRef<SendOpen>(a) = new(protocol, applicationName);
                        var added = _pending.TryAdd(await WriteAsync(a.AsMemory(0, size), lts.Token).ConfigureAwait(false), tcs);
                        Debug.Assert(added);
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
                    _run = Task.Factory.StartNew(async () =>
                    {
                        using (lts)
                            while (!lts.IsCancellationRequested)
                                await ReadAsync(lts.Token).ConfigureAwait(false);
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

        public async ValueTask DefineDataAsync<T>(CancellationToken cancellationToken = default) where T : unmanaged
        {
            var id = Interlocked.Increment(ref _defineId);
            if (!(_defineTypes.TryAdd(id, typeof(T)) && _defineIds.TryAdd(typeof(T), id)))
                throw new InvalidOperationException();
            Debug.Assert(typeof(T) is
            {
                IsValueType: true,
                IsLayoutSequential: true,
                StructLayoutAttribute: { Pack: 1 }
            });
            var size = Unsafe.SizeOf<SendAddToDataDefinition>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                var m = a.AsMemory(0, size);
                foreach (var f in typeof(T).GetFields(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance))
                {
                    var d = f.GetCustomAttribute<DataDefinitionAttribute>();
                    Debug.Assert(d is not null && f.FieldType is var t && d.DatumType switch
                    {
                        DataType.Invalid => false,
                        DataType.Int32 => t == typeof(int),
                        DataType.Int64 => t == typeof(long),
                        DataType.Float32 => t == typeof(float),
                        DataType.Float64 => t == typeof(double),
                        DataType.String8 => t == typeof(String8),
                        DataType.String32 => t == typeof(String32),
                        DataType.String64 => t == typeof(String64),
                        DataType.String128 => t == typeof(String128),
                        DataType.String256 => t == typeof(String256),
                        DataType.String260 => t == typeof(String260),
                        DataType.StringV => false,
                        DataType.InitPosition => t == typeof(InitPosition),
                        DataType.MarkerState => t == typeof(MarkerState),
                        DataType.Waypoint => t == typeof(Waypoint),
                        DataType.LatLonAlt => t == typeof(LatLonAlt),
                        DataType.XYZ => t == typeof(XYZ),
                        _ => false,
                    });
                    MemoryMarshal.AsRef<SendAddToDataDefinition>(m.Span) = new(id, d.DatumName, d.UnitsName, d.DatumType, d.Epsilon, d.DatumId);
                    await WriteAsync(m, cancellationToken).ConfigureAwait(false);
                }
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask UndefineDataAsync<T>(CancellationToken cancellationToken = default) where T : unmanaged
        {
            if (_defineIds.TryRemove(typeof(T), out var id))
            {
                try
                {
                    var size = Unsafe.SizeOf<SendClearDataDefinition>();
                    var a = ArrayPool<byte>.Shared.Rent(size);
                    try
                    {
                        MemoryMarshal.AsRef<SendClearDataDefinition>(a) = new(id);
                        await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false);
                    }
                    finally
                    {
                        ArrayPool<byte>.Shared.Return(a);
                    }
                }
                finally
                {
                    _defineTypes.TryRemove(id, out _);
                }
            }
        }

        public async ValueTask<T> RequestDataOnSimObjectAsync<T>(uint objectId, Period period, DataRequestFlags flags = DataRequestFlags.Default, uint origin = 0, uint interval = 0, uint limit = 0, CancellationToken cancellationToken = default) where T : unmanaged
        {
            _defineIds.TryGetValue(typeof(T), out var defineId);
            var requestId = Interlocked.Increment(ref _requestId);
            TaskCompletionSource<T> tcs = new();
            unsafe
            {
                var buffer = new byte[sizeof(T)];
                uint offset = 0;
                _requests.TryAdd(requestId, data =>
                {
                    var length = (uint)data->Data.Length;
                    fixed (byte* source = data->Data)
                    fixed (byte* destination = &buffer[offset])
                        Unsafe.CopyBlock(destination, source, length);
                    offset += length;
                    if (data->EntryNumber == data->OutOf)
                    {
                        // TODO: period?
                        _requests.TryRemove(requestId, out _);
                        tcs.SetResult(MemoryMarshal.AsRef<T>(buffer));
                    }
                });
            }
            var size = Unsafe.SizeOf<SendRequestDataOnSimObject>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendRequestDataOnSimObject>(a) = new(requestId, defineId, objectId, period, flags, origin, interval, limit);
                await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
            return await tcs.Task;
        }
    }
}