using System;
using System.Threading.Tasks;

namespace FlyByWireless.SimConnect
{
    public sealed class ClientEvent : IAsyncDisposable
    {
        internal readonly uint EventId;

        internal readonly EventHandler<int> Received;

        public readonly Task Mapped;

        readonly Func<ValueTask> _remove;

        internal ClientEvent(uint eventId, EventHandler<int> received, Task mapped, Func<ValueTask> remove) =>
            (EventId, Received, Mapped, _remove) = (eventId, received, mapped, remove);

        public ValueTask DisposeAsync() => _remove();
    }

    public sealed class InputEvent : IAsyncDisposable
    {
        internal readonly string InputDefinition;

        public Task Mapped { get; }

        readonly Func<ValueTask> _remove;

        internal InputEvent(string inputDefinition, Task mapped, Func<ValueTask> remove) =>
            (InputDefinition, Mapped, _remove) = (inputDefinition, mapped, remove);

        public ValueTask DisposeAsync() => _remove();
    }

    public sealed class ReservedKey
    {
        public readonly string Chosen, Key;

        internal ReservedKey(RecvReservedKey recv) =>
            (Chosen, Key) = (recv.ChoiceReserved, recv.ReservedKey);

        public override string ToString() => Key;
    }
}