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

    unsafe delegate void PointerAction(void* pointer);

    public sealed class DataDefinition : IAsyncDisposable
    {
        internal uint DefineId { get; }

        internal (int Offset, int Size)[] Used { get; }

        readonly Func<ValueTask> _undefine;

        internal DataDefinition(uint defineId, (int Offset, int Size)[] used, Func<ValueTask> undefine) =>
            (DefineId, Used, _undefine) = (defineId, used, undefine);

        public ValueTask DisposeAsync() => _undefine();
    }

    public sealed class SimConnect : IAsyncDisposable, IDisposable
    {
        static readonly UnboundedChannelOptions _channelOptions = new()
        {
            AllowSynchronousContinuations = true,
            SingleReader = true,
            SingleWriter = true
        };

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

        readonly ConcurrentDictionary<Type, DataDefinition> _defines = new();

        readonly ConcurrentDictionary<uint, TaskCompletionSource<object>> _pending = new();

        readonly ConcurrentDictionary<uint, PointerAction> _requests = new();

        public event EventHandler<ExternalException>? OnRecvException;

        public event EventHandler<RecvQuit>? OnRecvQuit;

        void Disposed()
        {
            _pending.Clear();
            _defineTypes.Clear();
            _defines.Clear();
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
                        case RecvId.WeatherObservation:
                        case RecvId.CloudState:
                        case RecvId.SystemState:
                        case RecvId.ClientData:
                        case RecvId.AirportList:
                        case RecvId.VORList:
                        case RecvId.NDBList:
                        case RecvId.WaypointList:
                            unsafe
                            {
                                fixed (void* p = s)
                                {
                                    if (_requests.TryGetValue(((uint*)p)[3], out var callback))
                                        callback(p);
                                }
                            }
                            break;
                        case RecvId.AssignedObjectId: throw new NotImplementedException();
                        case RecvId.ReservedKey: throw new NotImplementedException();
                        case RecvId.CustomAction: throw new NotImplementedException();
                        case RecvId.EventWeatherMode: throw new NotImplementedException();
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
                        // TODO: clean up
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

        public async ValueTask<DataDefinition> DefineDataAsync<T>(CancellationToken cancellationToken = default) where T : unmanaged
        {
            if (typeof(T) is not
                {
                    IsValueType: true,
                    IsLayoutSequential: true,
                    StructLayoutAttribute: { Pack: 1 }
                })
                throw new NotSupportedException(); // TODO: support when requested in tagged format
            var size = Unsafe.SizeOf<SendAddToDataDefinition>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                var m = a.AsMemory(0, size);
                var fields = typeof(T).GetFields(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance);
                var used = new (int Offset, int Size)[fields.Length];
                var id = Interlocked.Increment(ref _defineId);
                DataDefinition define = new(id, used, async () =>
                {
                    if (_defines.TryRemove(typeof(T), out var d))
                    {
                        try
                        {
                            var size = Unsafe.SizeOf<SendClearDataDefinition>();
                            var a = ArrayPool<byte>.Shared.Rent(size);
                            try
                            {
                                MemoryMarshal.AsRef<SendClearDataDefinition>(a) = new(d.DefineId);
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
                });
                if (!(_defineTypes.TryAdd(id, typeof(T)) && _defines.TryAdd(typeof(T), define)))
                    throw new InvalidOperationException();
                var datumId = 0;
                var offset = 0;
                foreach (var f in fields)
                {
                    var d = f.GetCustomAttribute<DataDefinitionAttribute>();
                    var t = f.FieldType;
                    unsafe
                    {
                        var s = d!.DatumType switch
                        {
                            DataType.Int32 when t == typeof(int) => sizeof(int),
                            DataType.Int64 when t == typeof(long) => sizeof(long),
                            DataType.Float32 when t == typeof(float) => sizeof(float),
                            DataType.Float64 when t == typeof(double) => sizeof(double),
                            DataType.String8 when t == typeof(String8) => sizeof(String8),
                            DataType.String32 when t == typeof(String32) => sizeof(String32),
                            DataType.String64 when t == typeof(String64) => sizeof(String64),
                            DataType.String128 when t == typeof(String128) => sizeof(String128),
                            DataType.String256 when t == typeof(String256) => sizeof(String256),
                            DataType.String260 when t == typeof(String260) => sizeof(String260),
                            DataType.InitPosition when t == typeof(InitPosition) => sizeof(InitPosition),
                            DataType.MarkerState when t == typeof(MarkerState) => sizeof(MarkerState),
                            DataType.Waypoint when t == typeof(Waypoint) => sizeof(Waypoint),
                            DataType.LatLonAlt when t == typeof(LatLonAlt) => sizeof(LatLonAlt),
                            DataType.XYZ when t == typeof(XYZ) => sizeof(XYZ),
                            _ => throw new NotSupportedException("Datum type mismatch."),
                        };
                        used[d.DatumId = datumId++] = (offset, s);
                        offset += s;
                    }
                    MemoryMarshal.AsRef<SendAddToDataDefinition>(m.Span) = new(id, d.DatumName, d.UnitsName, d.DatumType, d.Epsilon, d.DatumId);
                    await WriteAsync(m, cancellationToken).ConfigureAwait(false);
                }
                return define;
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        sealed class DataAsyncEnumerable<T> : IAsyncEnumerable<T> where T : unmanaged
        {
            ChannelReader<T>? _reader;

            readonly Func<OperationCanceledException, ValueTask> _cancel;

            public DataAsyncEnumerable(ChannelReader<T> reader, Func<OperationCanceledException, ValueTask> cancel) => (_reader, _cancel) = (reader, cancel);

            public async IAsyncEnumerator<T> GetAsyncEnumerator(CancellationToken cancellationToken = default)
            {
                ChannelReader<T>? r;
                (r, _reader) = (_reader, null);
                if (r is null)
                    throw new InvalidOperationException();
                for (; ; )
                {
                    try
                    {
                        if (!await r.WaitToReadAsync(cancellationToken).ConfigureAwait(false))
                            break;
                    }
                    catch (OperationCanceledException ex) when (cancellationToken.IsCancellationRequested)
                    {
                        await _cancel.Invoke(ex).ConfigureAwait(false);
                    }
                    var e = r.ReadAllAsync(cancellationToken).GetAsyncEnumerator(cancellationToken);
                    for (; ; )
                    {
                        try
                        {
                            if (!await e.MoveNextAsync())
                                break;
                        }
                        catch (OperationCanceledException ex) when (cancellationToken.IsCancellationRequested)
                        {
                            await _cancel.Invoke(ex).ConfigureAwait(false);
                            goto Complete;
                        }
                        yield return e.Current;
                    }
                }
            Complete: { }
            }
        }

        sealed class EmptyAsyncEnumerable<T> : IAsyncEnumerable<T> where T : unmanaged
        {
            public static EmptyAsyncEnumerable<T> Instance { get; } = new();

            public IAsyncEnumerator<T> GetAsyncEnumerator(CancellationToken cancellationToken = default) => E.Instance;

            class E : IAsyncEnumerator<T>
            {
                public static E Instance => new();

                public T Current => default;

                public ValueTask DisposeAsync() => default;

                public ValueTask<bool> MoveNextAsync() => ValueTask.FromResult(false);
            }
        }

        public async ValueTask<T> RequestDataOnSimObjectOnceAsync<T>(uint objectId, DataRequestFlags flags = DataRequestFlags.Default, CancellationToken cancellationToken = default) where T : unmanaged
        {
            // TODO: optimize
            var e = (await RequestDataOnSimObjectAsync<T>(objectId, Period.Once, flags, cancellationToken: cancellationToken).ConfigureAwait(false)).GetAsyncEnumerator(cancellationToken);
            await using var _e = e.ConfigureAwait(false);
            await e.MoveNextAsync().ConfigureAwait(false);
            return e.Current;
        }

        public async ValueTask<IAsyncEnumerable<T>> RequestDataOnSimObjectAsync<T>(uint objectId, Period period, DataRequestFlags flags = DataRequestFlags.Default, uint origin = 0, uint interval = 0, uint limit = 0, CancellationToken cancellationToken = default) where T : unmanaged
        {
            var define = _defines[typeof(T)];
            if (period == Period.Never)
                return EmptyAsyncEnumerable<T>.Instance;
            var size = Unsafe.SizeOf<SendRequestDataOnSimObject>();
            var requestId = Interlocked.Increment(ref _requestId);
            DataAsyncEnumerable<T> E(uint take)
            {
                var channel = Channel.CreateUnbounded<T>(_channelOptions);
                var writter = channel.Writer;
                uint written = 0;
                unsafe
                {
                    var buffer = flags == DataRequestFlags.Tagged ? new byte[sizeof(T)] : null;
                    var added = _requests.TryAdd(requestId, pointer =>
                    {
                        var data = ((RecvData*)pointer)->Data;
                        if (buffer != null)
                        {
                            for (var i = 0; i < data.Length;)
                            {
                                var (offset, next) = define.Used[MemoryMarshal.AsRef<uint>(data[i..])];
                                next += i += sizeof(uint);
                                data[i..next].CopyTo(buffer.AsSpan(offset));
                                i = next;
                            }
                            data = buffer;
                        }
                        if (writter.TryWrite(MemoryMarshal.AsRef<T>(data)) && (take == 0 || ++written < take))
                            return;
                        _requests.TryRemove(requestId, out _);
                        writter.TryComplete();
                    });
                    Debug.Assert(added);
                }
                return new(channel.Reader, async e =>
                {
                    if (_requests.TryRemove(requestId, out _) && writter.TryComplete(e))
                    {
                        var a = ArrayPool<byte>.Shared.Rent(size);
                        try
                        {
                            MemoryMarshal.AsRef<SendRequestDataOnSimObject>(a) = new(requestId, 0, 0, Period.Never);
                            await WriteAsync(a.AsMemory(0, size), default).ConfigureAwait(false);
                        }
                        catch (ObjectDisposedException) { }
                        finally
                        {
                            ArrayPool<byte>.Shared.Return(a);
                        }
                    }
                });
            }
            var e = E(period == Period.Once ? 1 : limit);
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendRequestDataOnSimObject>(a) = new(requestId, define.DefineId, objectId, period, flags, origin, interval, limit);
                await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
            // TODO: support modifying parameters (except tagged) during enumeration? even from Never to another period?
            return e;
        }

        public async ValueTask<IAsyncEnumerable<T>> RequestDataOnSimObjectTypeAsync<T>(uint radiusMeters, SimObjectType type, CancellationToken cancellationToken = default) where T : unmanaged
        {
            var define = _defines[typeof(T)];
            var size = Unsafe.SizeOf<SendRequestDataOnSimObjectType>();
            var requestId = Interlocked.Increment(ref _requestId);
            DataAsyncEnumerable<T> E()
            {
                var channel = Channel.CreateUnbounded<T>(_channelOptions);
                var writter = channel.Writer;
                unsafe
                {
                    var added = _requests.TryAdd(requestId, pointer =>
                    {
                        var recv = (RecvData*)pointer;
                        if (recv->EntryNumber != 0)
                            writter.TryWrite(MemoryMarshal.AsRef<T>(recv->Data));
                        if (recv->EntryNumber == recv->OutOf)
                        {
                            _requests.TryRemove(requestId, out _);
                            writter.TryComplete();
                        }
                    });
                    Debug.Assert(added);
                }
                return new(channel.Reader, async e =>
                {
                    if (_requests.TryRemove(requestId, out _) && writter.TryComplete(e))
                    {
                        var a = ArrayPool<byte>.Shared.Rent(size);
                        try
                        {
                            MemoryMarshal.AsRef<SendRequestDataOnSimObjectType>(a) = new(requestId, 0, 0, default);
                            await WriteAsync(a.AsMemory(0, size), default).ConfigureAwait(false);
                        }
                        catch (ObjectDisposedException) { }
                        finally
                        {
                            ArrayPool<byte>.Shared.Return(a);
                        }
                    }
                });
            }
            var e = E();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendRequestDataOnSimObjectType>(a) = new(requestId, 0, 0, default);
                await WriteAsync(a.AsMemory(0, size), default).ConfigureAwait(false);
            }
            catch (ObjectDisposedException) { }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
            // TODO: support modifying radius and/or type during enumeration?
            return e;
        }
    }
}