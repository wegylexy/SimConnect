using FlyByWireless.SimConnect.Data;
using System;
using System.Threading.Tasks;

namespace FlyByWireless.SimConnect
{
    #region EventArgs
    public class DataEventArgs : EventArgs
    {
        public readonly int Data;

        internal DataEventArgs(in RecvEvent recv) => Data = recv.Data;
    }

    public class FileNameEventArgs : DataEventArgs
    {
        public readonly string FileName;

        public readonly uint Flags;

        internal FileNameEventArgs(in RecvEventFileName recv) : base(recv.Event) =>
            (FileName, Flags) = (recv.FileName, recv.Flags);
    }

    public class ObjectAddRemoveEventArgs : DataEventArgs
    {
        public readonly SimObjectType ObjType;

        internal ObjectAddRemoveEventArgs(in RecvEventObjectAddRemove recv) : base(recv.Event) =>
            ObjType = recv.ObjType;
    }

    public class FrameEventArgs : DataEventArgs
    {
        public readonly float FrameRate, SimSpeed;

        internal FrameEventArgs(in RecvEventFrame recv) : base(recv.Event) =>
            (FrameRate, SimSpeed) = (recv.FrameRate, recv.SimSpeed);
    }

    [Obsolete]
    public class MultiplayerServerStartedEventArgs : DataEventArgs
    {
        internal MultiplayerServerStartedEventArgs(in RecvEventMultiplayerServerStarted recv) : base(recv.Event) { }
    }

    [Obsolete]
    public class MultiplayerClientStartedEventArgs : DataEventArgs
    {
        internal MultiplayerClientStartedEventArgs(in RecvEventMultiplayerClientStarted recv) : base(recv.Event) { }
    }

    [Obsolete]
    public class MultiplayerSessionEndedEventArgs : DataEventArgs
    {
        internal MultiplayerSessionEndedEventArgs(in RecvEventMultiplayerSessionEnded recv) : base(recv.Event) { }
    }

    [Obsolete]
    public class RaceEndEventArgs : DataEventArgs
    {
        public readonly uint RacerNumber;

        public readonly RaceResult RacerData;

        internal RaceEndEventArgs(in RecvEventRaceEnd recv) : base(recv.Event) =>
            (RacerNumber, RacerData) = (recv.RacerNumber, recv.RacerData);
    }

    [Obsolete]
    public class RaceLapEventArgs : DataEventArgs
    {
        public readonly uint LapIndex;

        public readonly RaceResult RacerData;

        internal RaceLapEventArgs(in RecvEventRaceLap recv) : base(recv.Event) =>
            (LapIndex, RacerData) = (recv.LapIndex, recv.RacerData);
    }

    [Obsolete]
    public class WeatherModeEventArgs : DataEventArgs
    {
        public new WeatherMode Data => (WeatherMode)base.Data;

        internal WeatherModeEventArgs(in RecvEventWeatherMode recv) : base(recv.Event) { }
    }
    #endregion

    public sealed class DataEvent : IAsyncDisposable
    {
        internal readonly uint EventId;

        internal readonly EventHandler<DataEventArgs> Received;

        public readonly Task Mapped;

        readonly Func<ValueTask> _remove;

        internal DataEvent(uint eventId, EventHandler<DataEventArgs> received, Task mapped, Func<ValueTask> remove) =>
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