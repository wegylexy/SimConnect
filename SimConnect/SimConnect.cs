using FlyByWireless.IO;
using FlyByWireless.SimConnect.Data;
using Microsoft.Win32.SafeHandles;
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

    public sealed class AsyncException : System.Exception
    {
        public uint Index { get; }

        internal AsyncException(RecvException exception) : base(exception.Exception.ToString()) =>
            (HResult, Index) = ((int)exception.Exception, exception.Index);
    }

    public sealed class SimConnect : IAsyncDisposable, IDisposable
    {
        [DllImport("kernel32", SetLastError = true)]
        [return: MarshalAs(UnmanagedType.Bool)]
        static extern bool PeekNamedPipe(SafePipeHandle hNamedPipe, nint lpBuffer, uint nBufferSize, out uint lpBytesRead, out uint lpTotalBytesAvail, out uint lpBytesLeftThisMessage);

        static readonly UnboundedChannelOptions _channelOptions = new()
        {
            AllowSynchronousContinuations = true,
            SingleReader = true,
            SingleWriter = true
        };

        static readonly ReadOnlyCollection<Protocol> _protocols = new(new Protocol[]
        {
            new(4, new(10, 0, 63003, 0)), // SE beta
            new(4, new(10, 0, 62615, 0)), // SE
            new(4, new(10, 0, 61637, 0)), // SP2
            new(4, new(10, 0, 61472, 0)), // SP2 1A
            new(4, new(10, 0, 61259, 0)), // X-PACK
            new(3, new(10, 0, 61355, 0)), // SP1 1A
            new(3, new(10, 0, 61242, 0)), // SP1
            new(2, new(0, 0, 60905, 0)) // RTM
        });

        Stream ClientStream { get; } =
            new NamedPipeClientStream(".", @"Microsoft Flight Simulator\SimConnect", PipeDirection.InOut, PipeOptions.Asynchronous);

        CancellationTokenSource? _quit;

        Task? _run;

        uint _packetId = 0, _protocol, _defineId, _requestId, _eventId;

        readonly ConcurrentDictionary<uint, Type> _defineTypes = new();

        readonly ConcurrentDictionary<Type, DataDefinition> _defines = new();

        TaskCompletionSource<RecvOpen>? _open;

        readonly ConcurrentDictionary<uint, TaskCompletionSource> _sents = new();

        readonly ConcurrentDictionary<uint, PointerAction> _requests = new();

        readonly ConcurrentDictionary<uint, EventHandler<DataEventArgs>> _eventHandlers = new();

        readonly ConcurrentDictionary<string, (TaskCompletionSource<ReservedKey> Reserved, string Choice1, string? Choice2, string? Choice3)> _reservedKeys = new();

        public event EventHandler<AsyncException>? UncaughtException;

        public event Action<object>? Quit;

        void Disposed()
        {
            _defineTypes.Clear();
            _defines.Clear();
            _open = null;
            _sents.Clear();
            _requests.Clear();
            _eventHandlers.Clear();
            _reservedKeys.Clear();
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

        async ValueTask<Task> WriteAsync(Memory<byte> buffer, CancellationToken cancellationToken = default)
        {
            uint Id()
            {
                var send = MemoryMarshal.Cast<byte, uint>(buffer.Span);
                send[1] = _protocol;
                return send[3] = Interlocked.Increment(ref _packetId);
            }
            var id = Id();
            TaskCompletionSource tcs = new();
            var added = _sents.TryAdd(id, tcs);
            Debug.Assert(added);
            await ClientStream.WriteAsync(buffer, cancellationToken).ConfigureAwait(false);
            return tcs.Task;
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
                        case RecvId.Null:
                            break;
                        case RecvId.Exception:
                            {
                                ref var r = ref MemoryMarshal.AsRef<RecvException>(s);
                                var ex = new AsyncException(r);
                                if (!(_sents.TryRemove(r.SendId, out var tcs) && tcs.TrySetException(ex)))
                                    UncaughtException?.Invoke(this, ex);
                            }
                            break;
                        case RecvId.Open:
                            {
                                var open = _open;
                                open?.TrySetResult(MemoryMarshal.AsRef<RecvOpen>(s));
                                _open = null;
                            }
                            break;
                        case RecvId.Quit:
                            _quit?.Cancel();
                            Quit?.Invoke(this);
                            break;
                        case RecvId.Event:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEvent>(s);
                                if (_eventHandlers.TryGetValue(e.EventId, out var h))
                                    h?.Invoke(this, new(in e));
                            }
                            break;
                        case RecvId.EventObjectAddRemove:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventObjectAddRemove>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new ObjectAddRemoveEventArgs(in e));
                            }
                            break;
                        case RecvId.EventFileName:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventFileName>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new FileNameEventArgs(in e));
                            }
                            break;
                        case RecvId.EventFrame:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventFrame>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new FrameEventArgs(in e));
                            }
                            break;
                        case RecvId.SimObjectData:
                        case RecvId.SimObjectDataByType:
                        case RecvId.SystemState:
                        case RecvId.ClientData:
                        case RecvId.AirportList:
                        case RecvId.VORList:
                        case RecvId.NDBList:
                        case RecvId.WaypointList:
#pragma warning disable CS0612 // Type or member is obsolete
                        case RecvId.WeatherObservation:
                        case RecvId.CloudState:
#pragma warning restore CS0612 // Type or member is obsolete
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
                        case RecvId.ReservedKey:
                            {
                                ReservedKey k = new(MemoryMarshal.AsRef<RecvReservedKey>(s));
                                if (_reservedKeys.TryRemove(k.Chosen, out var t))
                                    t.Reserved.TrySetResult(k);
                            }
                            break;
#pragma warning disable CS0612 // Type or member is obsolete
                        case RecvId.CustomAction: throw new NotImplementedException();
                        case RecvId.EventWeatherMode:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventWeatherMode>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new WeatherModeEventArgs(in e));
                            }
                            break;
                        case RecvId.EventMultiplayerServerStarted:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventMultiplayerServerStarted>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new MultiplayerServerStartedEventArgs(in e));
                            }
                            break;
                        case RecvId.EventMultiplayerClientStarted:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventMultiplayerClientStarted>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new MultiplayerClientStartedEventArgs(in e));
                            }
                            break;
                        case RecvId.EventMultiplayerSessionEnded:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventMultiplayerSessionEnded>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new MultiplayerSessionEndedEventArgs(in e));
                            }
                            break;
                        case RecvId.EventRaceEnd:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventRaceEnd>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new RaceEndEventArgs(in e));
                            }
                            break;
                        case RecvId.EventRaceLap:
                            {
                                ref var e = ref MemoryMarshal.AsRef<RecvEventRaceLap>(s);
                                if (_eventHandlers.TryGetValue(e.Event.EventId, out var h))
                                    h?.Invoke(this, new RaceLapEventArgs(in e));
                            }
                            break;
#pragma warning restore CS0612 // Type or member is obsolete
                        default:
                            throw new NotSupportedException();
                    }
                }
                Parse();
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(buffer);
            }
            try
            {
                if (ClientStream is NamedPipeClientStream s && PeekNamedPipe(s.SafePipeHandle, 0, 0, out _, out var a, out _) && a > 0)
                    return;
            }
            catch (DllNotFoundException) { }
            foreach (var id in _sents.Keys)
                if (_sents.TryRemove(id, out var t))
                    t.TrySetResult();
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
                _open = new();
                var opening = _open.Task;
                Task fail;
                {
                    var a = ArrayPool<byte>.Shared.Rent(size);
                    try
                    {
                        MemoryMarshal.AsRef<SendOpen>(a) = new(protocol, applicationName);
                        fail = await WriteAsync(a.AsMemory(0, size), lts.Token).ConfigureAwait(false);
                    }
                    finally
                    {
                        ArrayPool<byte>.Shared.Return(a);
                    }
                }
                await ReadAsync(lts.Token).ConfigureAwait(false);
                try
                {
                    _ = await Task.WhenAny(opening, fail).ConfigureAwait(false);
                    var open = opening.Result;
                    _open = null;
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

        public async ValueTask<DataEvent> MapClientEventToSimEventAsync(uint groupId, EventHandler<DataEventArgs> handler, string? eventName = null, bool maskable = false, CancellationToken cancellationToken = default)
        {
            var id = Interlocked.Increment(ref _eventId);
            var added = _eventHandlers.TryAdd(id, handler);
            Debug.Assert(added);
            var mapSize = Unsafe.SizeOf<SendMapClientEventToSimEvent>();
            var addSize = Unsafe.SizeOf<SendAddClientEventToNotificationGroupAsync>();
            var a = ArrayPool<byte>.Shared.Rent(Math.Max(mapSize, addSize));
            try
            {
                MemoryMarshal.AsRef<SendMapClientEventToSimEvent>(a) = new(id, eventName);
                var mapping = await WriteAsync(a.AsMemory(0, mapSize), cancellationToken).ConfigureAwait(false);
                MemoryMarshal.AsRef<SendAddClientEventToNotificationGroupAsync>(a) = new(groupId, id, maskable);
                var adding = await WriteAsync(a.AsMemory(0, addSize), cancellationToken).ConfigureAwait(false);
                return new(id, handler, Task.WhenAll(
                    mapping.ContinueWith(task =>
                    {
                        if (task.Exception?.InnerException is AsyncException ee)
                            throw new ArgumentException(ee.Message, ee.Index switch
                            {
                                2 => nameof(eventName),
                                _ => throw ee
                            });
                    }, TaskContinuationOptions.OnlyOnFaulted),
                    adding.ContinueWith(task =>
                    {
                        if (task.Exception?.InnerException is AsyncException ee)
                            throw new ArgumentException(ee.Message, ee.Index switch
                            {
                                1 => nameof(groupId),
                                3 => nameof(maskable),
                                _ => throw ee
                            });
                    }, TaskContinuationOptions.OnlyOnFaulted)
                ), async () =>
                {
                    var size = Unsafe.SizeOf<SendRemoveClientEventAsync>();
                    var a = ArrayPool<byte>.Shared.Rent(size);
                    try
                    {
                        MemoryMarshal.AsRef<SendRemoveClientEventAsync>(a) = new(groupId, id);
                        _ = await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false);
                    }
                    finally
                    {
                        ArrayPool<byte>.Shared.Return(a);
                    }
                });
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<Task> TransmitClientEventAsync(DataEvent mappedEvent, uint objectId, int data, uint groupId, EventFlags flags, CancellationToken cancellationToken = default)
        {
            var size = Unsafe.SizeOf<SendTransmitClientEvent>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendTransmitClientEvent>(a) = new(objectId, mappedEvent.EventId, data, groupId, flags);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(objectId),
                            3 => nameof(data),
                            4 => nameof(groupId),
                            5 => nameof(flags),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<Task> SetSystemEventStateAsync(DataEvent subscribedEvent, State state, CancellationToken cancellationToken = default)
        {
            var size = Unsafe.SizeOf<SendSetSystemEventState>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendSetSystemEventState>(a) = new(subscribedEvent.EventId, state);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(subscribedEvent),
                            2 => nameof(state),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<Task> SetNotificationGroupPriorityAsync(uint groupId, GroupPriority priority, CancellationToken cancellationToken = default)
        {
            var size = Unsafe.SizeOf<SendSetNotificationGroupPriority>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendSetNotificationGroupPriority>(a) = new(groupId, priority);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(groupId),
                            2 => nameof(priority),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<Task> ClearNotificationGroupAsync(uint groupId, CancellationToken cancellationToken = default)
        {
            var size = Unsafe.SizeOf<SendClearNotificationGroup>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendClearNotificationGroup>(a) = new(groupId);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(groupId),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<Task> RequestNotificationGroupAsync(uint groupId, uint reserved = 0, uint flags = 0, CancellationToken cancellationToken = default)
        {
            var size = Unsafe.SizeOf<SendRequestNotificationGroup>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendRequestNotificationGroup>(a) = new(groupId, reserved, flags);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(groupId),
                            2 => nameof(reserved),
                            3 => nameof(flags),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        #region SimObject Data
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
                if (!(_defineTypes.TryAdd(id, typeof(T)) && _defines.TryAdd(typeof(T), null!)))
                    throw new InvalidOperationException();
                var datumId = 0;
                var offset = 0;
                var adding = new Task[fields.Length];
                foreach (var f in fields)
                {
                    var d = f.GetCustomAttribute<DataDefinitionAttribute>() ?? new();
                    unsafe
                    {
                        var (t, s) = f.FieldType.Name switch
                        {
                            nameof(Int32) => (DataType.Int32, sizeof(int)),
                            nameof(Int64) => (DataType.Int64, sizeof(long)),
                            nameof(Single) => (DataType.Float32, sizeof(float)),
                            nameof(Double) => (DataType.Float64, sizeof(double)),
                            nameof(String8) when d.UnitsName is null => (DataType.String8, sizeof(String8)),
                            nameof(String32) when d.UnitsName is null => (DataType.String32, sizeof(String32)),
                            nameof(String64) when d.UnitsName is null => (DataType.String64, sizeof(String64)),
                            nameof(String128) when d.UnitsName is null => (DataType.String128, sizeof(String128)),
                            nameof(String256) when d.UnitsName is null => (DataType.String256, sizeof(String256)),
                            nameof(String260) when d.UnitsName is null => (DataType.String260, sizeof(String260)),
                            nameof(MarkerState) when d.UnitsName is null => (DataType.MarkerState, sizeof(MarkerState)),
                            nameof(Waypoint) when d.UnitsName is null => (DataType.Waypoint, sizeof(Waypoint)),
                            nameof(LatLonAlt) when d.UnitsName is null => (DataType.LatLonAlt, sizeof(LatLonAlt)),
                            nameof(XYZ) when d.UnitsName is null => (DataType.XYZ, sizeof(XYZ)),
                            _ => throw new NotSupportedException("Datum type and/or units name mismatch."),
                        };
                        used[d.DatumId = datumId] = (offset, s);
                        offset += s;
                        MemoryMarshal.AsRef<SendAddToDataDefinition>(m.Span) =
                            new(id, d.DatumName ?? f.Name.ToUpperInvariant().Replace('_', ' '), d.UnitsName, t, d.Epsilon, d.DatumId);
                    }
                    adding[datumId] = (await WriteAsync(m, cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                    {
                        if (task.Exception?.InnerException is AsyncException ee)
                            throw new DataDefinitionException(f, ee.Index switch
                            {
                                2 => nameof(d.DatumName),
                                3 => nameof(d.UnitsName),
                                5 => nameof(d.Epsilon),
                                _ => throw ee
                            }, (Exception)ee.HResult);
                    }, TaskContinuationOptions.OnlyOnFaulted);
                    ++datumId;
                }
                // TODO: use CollectionsMarshal in .NET 6
                return _defines[typeof(T)] = new(id, used, Task.WhenAll(adding), async () =>
                {
                    if (_defines.TryRemove(typeof(T), out var d))
                    {
                        // TODO: change period to never
                        try
                        {
                            var size = Unsafe.SizeOf<SendClearDataDefinition>();
                            var a = ArrayPool<byte>.Shared.Rent(size);
                            try
                            {
                                MemoryMarshal.AsRef<SendClearDataDefinition>(a) = new(d.DefineId);
                                _ = await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false);
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
            var channel = Channel.CreateUnbounded<T>(_channelOptions);
            var writter = channel.Writer;
            DataAsyncEnumerable<T> E(uint take)
            {
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
                            _ = await WriteAsync(a.AsMemory(0, size), default).ConfigureAwait(false);
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
                _ = (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        writter.TryComplete(new ArgumentException(ee.Message, ee.Index switch
                        {
                            3 => nameof(objectId),
                            4 => nameof(period),
                            5 => nameof(flags),
                            6 => nameof(origin),
                            7 => nameof(interval),
                            8 => nameof(limit),
                            _ => throw ee
                        }));
                }, TaskContinuationOptions.OnlyOnFaulted);
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
            var channel = Channel.CreateUnbounded<T>(_channelOptions);
            var writter = channel.Writer;
            DataAsyncEnumerable<T> E()
            {
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
                            _ = await WriteAsync(a.AsMemory(0, size), default).ConfigureAwait(false);
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
                _ = (await WriteAsync(a.AsMemory(0, size), default).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        writter.TryComplete(new ArgumentException(ee.Message, ee.Index switch
                        {
                            3 => nameof(radiusMeters),
                            4 => nameof(type),
                            _ => throw ee
                        }));
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            catch (ObjectDisposedException) { }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
            return e;
        }

        public ValueTask<Task> SetDataOnSimObjectAsync<T>(uint objectId, ref T data, CancellationToken cancellationToken = default) where T : unmanaged =>
            SetDataOnSimObjectAsync(objectId, MemoryMarshal.CreateReadOnlySpan(ref data, 1), cancellationToken);

        public ValueTask<Task> SetDataOnSimObjectAsync<T>(uint objectId, ReadOnlySpan<T> data, CancellationToken cancellationToken = default) where T : unmanaged
        {
            Debug.Assert(!data.IsEmpty);
            var define = _defines[typeof(T)];
            var size = Unsafe.SizeOf<SendSetDataOnSimObject<T>>() + Unsafe.SizeOf<T>() * (data.Length - 1);
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendSetDataOnSimObject<T>>(a) = new(define.DefineId, objectId, data);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
            async ValueTask<Task> Async()
            {
                try
                {
                    return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                    {
                        if (task.Exception?.InnerException is AsyncException ee)
                            throw new ArgumentException(ee.Message, ee.Index switch
                            {
                                2 => nameof(objectId),
                                6 => nameof(data),
                                _ => throw ee
                            });
                    }, TaskContinuationOptions.OnlyOnFaulted);
                }
                finally
                {
                    ArrayPool<byte>.Shared.Return(a);
                }
            }
            return Async();
        }
        #endregion

        public async ValueTask<InputEvent> MapInputEventToClientEventAsync(uint groupId, string inputDefinition, DataEvent? downHandler = null, int downValue = 0, DataEvent? upHandler = null, int upValue = 0, bool maskable = false, CancellationToken cancellationToken = default)
        {
            uint downId = downHandler?.EventId ?? uint.MaxValue, upId = upHandler?.EventId ?? uint.MaxValue;
            var size = Unsafe.SizeOf<SendMapInputEventToClientEvent>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendMapInputEventToClientEvent>(a) = new(groupId, inputDefinition, downId, downValue, upId, upValue, maskable);
                return new(inputDefinition, (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(groupId),
                            2 => nameof(inputDefinition),
                            4 => nameof(downValue),
                            6 => nameof(upValue),
                            7 => nameof(maskable),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted), async () =>
                {
                    var size = Unsafe.SizeOf<SendRemoveInputEvent>();
                    var a = ArrayPool<byte>.Shared.Rent(size);
                    try
                    {
                        MemoryMarshal.AsRef<SendRemoveInputEvent>(a) = new(groupId, inputDefinition);
                        _ = await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false);
                    }
                    finally
                    {
                        ArrayPool<byte>.Shared.Return(a);
                    }
                });
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<Task> SetInputGroupPriorityAsync(uint groupId, GroupPriority priority, CancellationToken cancellationToken)
        {
            var size = Unsafe.SizeOf<SendSetInputGroupPriority>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendSetInputGroupPriority>(a) = new(groupId, priority);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            2 => nameof(priority),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<Task> ClearInputGroupAsync(uint groupId, CancellationToken cancellationToken = default)
        {
            var size = Unsafe.SizeOf<SendClearInputGroup>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendClearInputGroup>(a) = new(groupId);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(groupId),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<Task> SetInputGroupStateAsync(uint groupId, State state, CancellationToken cancellationToken = default)
        {
            var size = Unsafe.SizeOf<SendSetInputGroupState>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendSetInputGroupState>(a) = new(groupId, state);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(groupId),
                            2 => nameof(state),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        public async ValueTask<ReservedKey> RequestReservedKeyAsync(DataEvent mappedEvent, string keyChoice1, string? keyChoice2, string? keyChoice3, CancellationToken cancellationToken = default)
        {
            TaskCompletionSource<ReservedKey> tcs = new();
            var t = (tcs, keyChoice1, keyChoice2, keyChoice3);
            var (added1, added2, added3) = (false, false, false);
            string? chosen = null;
            try
            {
                added1 = _reservedKeys.TryAdd(keyChoice1, t);
                if (string.IsNullOrEmpty(keyChoice2))
                    keyChoice2 = null;
                else
                    added2 = _reservedKeys.TryAdd(keyChoice2, t);
                if (string.IsNullOrEmpty(keyChoice3))
                    keyChoice3 = null;
                else
                    added3 = _reservedKeys.TryAdd(keyChoice3, t);
                var size = Unsafe.SizeOf<SendRequestReservedKey>();
                var a = ArrayPool<byte>.Shared.Rent(size);
                try
                {
                    MemoryMarshal.AsRef<SendRequestReservedKey>(a) = new(mappedEvent.EventId, keyChoice1, keyChoice2, keyChoice3);
                    _ = (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                    {
                        if (task.Exception?.InnerException is AsyncException ee)
                        {
                            tcs.TrySetException(ee.Index switch
                            {
                                1 => nameof(mappedEvent),
                                2 => nameof(keyChoice1),
                                3 => nameof(keyChoice2),
                                4 => nameof(keyChoice3),
                                _ => null
                            } is string n ? new ArgumentException(ee.Message, n) : ee);
                        }
                    }, TaskContinuationOptions.OnlyOnFaulted);
                }
                finally
                {
                    ArrayPool<byte>.Shared.Return(a);
                }
                var r = await tcs.Task.ConfigureAwait(false);
                chosen = r.Chosen;
                return r;
            }
            finally
            {
                if (added1 && keyChoice1 != chosen)
                    _reservedKeys.TryRemove(keyChoice1, out _);
                if (added2 && keyChoice2 != chosen)
                    _reservedKeys.TryRemove(keyChoice2!, out _);
                if (added3 && keyChoice3 != chosen)
                    _reservedKeys.TryRemove(keyChoice3!, out _);
            }
        }

        public async ValueTask<DataEvent> SubscribeToSystemEventAsync(EventHandler<DataEventArgs> handler, string eventName, CancellationToken cancellationToken = default)
        {
            var id = Interlocked.Increment(ref _eventId);
            var added = _eventHandlers.TryAdd(id, handler);
            Debug.Assert(added);
            var size = Unsafe.SizeOf<SendSubscribeToSystemEvent>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendSubscribeToSystemEvent>(a) = new(id, eventName);
                return new(id, handler, (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            2 => nameof(eventName),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted), async () =>
                {
                    var size = Unsafe.SizeOf<SendUnsubscribeToSystemEvent>();
                    var a = ArrayPool<byte>.Shared.Rent(size);
                    try
                    {
                        MemoryMarshal.AsRef<SendUnsubscribeToSystemEvent>(a) = new(id);
                        _ = await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false);
                    }
                    finally
                    {
                        ArrayPool<byte>.Shared.Return(a);
                    }
                });
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        #region Weather
        [Obsolete]
        public async ValueTask<string> WeatherRequestInterpolatedObservationAsync(float lat, float lon, float alt /* feet */, CancellationToken cancellationToken)
        {
            var id = Interlocked.Increment(ref _requestId);
            try
            {
                TaskCompletionSource<string> tcs = new();
                unsafe void R() => _requests.TryAdd(id, data => tcs.TrySetResult(((RecvWeatherObservation*)data)->Metar));
                R();
                var size = Unsafe.SizeOf<SendWeatherRequestInterpolatedObservation>();
                var a = ArrayPool<byte>.Shared.Rent(size);
                try
                {
                    MemoryMarshal.AsRef<SendWeatherRequestInterpolatedObservation>(a) = new(id, lat, lon, alt);
                    _ = (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                    {
                        if (task.Exception?.InnerException is AsyncException ee)
                        {
                            tcs.TrySetException(ee.Index switch
                            {
                                2 => nameof(lat),
                                3 => nameof(lon),
                                4 => nameof(alt),
                                _ => null
                            } is string n ? new ArgumentException(ee.Message, n) : ee);
                        }
                    }, TaskContinuationOptions.OnlyOnFaulted);
                }
                finally
                {
                    ArrayPool<byte>.Shared.Return(a);
                }
                return await tcs.Task.ConfigureAwait(false);
            }
            finally
            {
                _requests.TryRemove(id, out _);
            }
        }

        [Obsolete]
        public async ValueTask<string> WeatherRequestObservationAtStationAsync(string icao, CancellationToken cancellationToken)
        {
            var id = Interlocked.Increment(ref _requestId);
            try
            {
                TaskCompletionSource<string> tcs = new();
                unsafe void R() => _requests.TryAdd(id, data => tcs.TrySetResult(((RecvWeatherObservation*)data)->Metar));
                R();
                var size = Unsafe.SizeOf<SendWeatherRequestObservationAtStation>();
                var a = ArrayPool<byte>.Shared.Rent(size);
                try
                {
                    MemoryMarshal.AsRef<SendWeatherRequestObservationAtStation>(a) = new(id, icao, out size);
                    _ = (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                    {
                        if (task.Exception?.InnerException is AsyncException ee)
                        {
                            tcs.TrySetException(ee.Index switch
                            {
                                2 => nameof(icao),
                                _ => null
                            } is string n ? new ArgumentException(ee.Message, n) : ee);
                        }
                    }, TaskContinuationOptions.OnlyOnFaulted);
                }
                finally
                {
                    ArrayPool<byte>.Shared.Return(a);
                }
                return await tcs.Task.ConfigureAwait(false);
            }
            finally
            {
                _requests.TryRemove(id, out _);
            }
        }

        [Obsolete]
        public async ValueTask<string> WeatherRequestObservationAtNearestStationAsync(float lat, float lon, CancellationToken cancellationToken)
        {
            var id = Interlocked.Increment(ref _requestId);
            try
            {
                TaskCompletionSource<string> tcs = new();
                unsafe void R() => _requests.TryAdd(id, data => tcs.TrySetResult(((RecvWeatherObservation*)data)->Metar));
                R();
                var size = Unsafe.SizeOf<SendWeatherRequestObservationAtNearestStation>();
                var a = ArrayPool<byte>.Shared.Rent(size);
                try
                {
                    MemoryMarshal.AsRef<SendWeatherRequestObservationAtNearestStation>(a) = new(id, lat, lon);
                    _ = (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                    {
                        if (task.Exception?.InnerException is AsyncException ee)
                        {
                            tcs.TrySetException(ee.Index switch
                            {
                                2 => nameof(lat),
                                3 => nameof(lon),
                                _ => null
                            } is string n ? new ArgumentException(ee.Message, n) : ee);
                        }
                    }, TaskContinuationOptions.OnlyOnFaulted);
                }
                finally
                {
                    ArrayPool<byte>.Shared.Return(a);
                }
                return await tcs.Task.ConfigureAwait(false);
            }
            finally
            {
                _requests.TryRemove(id, out _);
            }
        }

        // TODO: other weather methods
        #endregion

        // TODO: AI methods

        // TODO: mission action methods

        [Obsolete]
        public async ValueTask<Task> CameraSetRelative6DOFAsync(float deltaX, float deltaY, float deltaZ, float pitchDeg, float bankDeg, float headingDeg, CancellationToken cancellationToken = default)
        {
            var size = Unsafe.SizeOf<SendCameraSetRelative6DOF>();
            var a = ArrayPool<byte>.Shared.Rent(size);
            try
            {
                MemoryMarshal.AsRef<SendCameraSetRelative6DOF>(a) = new(deltaX, deltaY, deltaZ, pitchDeg, bankDeg, headingDeg);
                return (await WriteAsync(a.AsMemory(0, size), cancellationToken).ConfigureAwait(false)).ContinueWith(task =>
                {
                    if (task.Exception?.InnerException is AsyncException ee)
                        throw new ArgumentException(ee.Message, ee.Index switch
                        {
                            1 => nameof(deltaX),
                            2 => nameof(deltaY),
                            3 => nameof(deltaZ),
                            4 => nameof(pitchDeg),
                            5 => nameof(bankDeg),
                            6 => nameof(headingDeg),
                            _ => throw ee
                        });
                }, TaskContinuationOptions.OnlyOnFaulted);
            }
            finally
            {
                ArrayPool<byte>.Shared.Return(a);
            }
        }

        // TODO: menu methods

        // TODO: system state methods

        // TODO: client data methods

        // TODO: flight plan methods

        // TODO: text method

        // TODO: facilities methods
    }
}