using System;
using System.ComponentModel.DataAnnotations;
using System.Runtime.InteropServices;

namespace FlyByWireless.SimConnect
{
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct Recv
    {
        public readonly uint Size, Version;
        public readonly RecvId Id;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvException
    {
        public const uint UnknownSendId = 0, UnknownIndex = uint.MaxValue;
        readonly Recv Recv;
        public readonly Exception Exception;
        public readonly uint SendId, Index;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvOpen
    {
        readonly Recv Recv;
        public readonly String256 ApplicationName;
        readonly int _ApplicationVersionMajor, _ApplicationVersionMinor, _ApplicationBuildMajor, _ApplicationBuildMinor,
            _SimConnectVersionMajor, _SimConnectVersionMinor, _SimConnectBuildMajor, _SimConnectBuildMinor,
            _Reserved1, _Reserved2;

        public Version ApplicationVersion => new(_ApplicationVersionMajor, _ApplicationVersionMinor, _ApplicationBuildMajor, _ApplicationBuildMinor);

        public Version SimConnectVersion => new(_SimConnectVersionMajor, _SimConnectVersionMinor, _SimConnectBuildMajor, _SimConnectBuildMinor);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvQuit
    {
        readonly Recv Recv;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEvent
    {
        public const uint UnknownGroup = uint.MaxValue;
        readonly Recv Recv;
        public readonly uint GroupId, EventId, Data;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventFileName
    {
        public readonly RecvEvent Event;
        public readonly String256 FileName;
        public readonly uint Flags;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventObjectAddRemove
    {
        public readonly RecvEvent Event;
        public readonly SimObjectType ObjType;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventFrame
    {
        public readonly RecvEvent Event;
        public readonly float FrameRate, SimSpeed;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventMultiplayerServerStarted
    {
        public readonly RecvEvent Event;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventMultiplayerClientStarted
    {
        public readonly RecvEvent Event;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventMultiplayerSessionEnded
    {
        public readonly RecvEvent Event;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct DataRaceResult
    {
        public readonly uint NumberOfRacers;
        public readonly Guid MissionGuid;
        public readonly String256 PlayerName, SessionType, Aircraft, PlayerRole;
        readonly double _TotalTime, _PenaltyTime;
        readonly int _IsDisqualified;

        public TimeSpan TotalTime => TimeSpan.FromSeconds(_TotalTime);

        public TimeSpan PenaltyTime => TimeSpan.FromSeconds(_PenaltyTime);

        public bool IsDisqualified => _IsDisqualified != 0;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventRaceEnd
    {
        public readonly RecvEvent Event;
        public readonly uint RacerNumber;
        public readonly DataRaceResult RacerData;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventRaceLap
    {
        public readonly RecvEvent Event;
        public readonly uint LapIndex;
        public readonly DataRaceResult RacerData;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public unsafe readonly ref struct RecvSimObjectData<T> where T : unmanaged
    {
        readonly Recv Recv;
        public readonly uint RequestId, ObjectId, DefineId;
        public readonly DataRequestFlags Flags;
        public readonly uint EntryNumber, OutOf;
        readonly int _DefineCount, _Data;

        public ReadOnlySpan<T> Data
        {
            get
            {
                // TODO: assert T against DefineId
                fixed (void* b = &_Data)
                    return new(b, _DefineCount);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly ref struct RecvSimObjectDataByType<T> where T : unmanaged
    {
        public readonly RecvSimObjectData<T> SimObjectData;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly ref struct RecvClientData<T> where T : unmanaged
    {
        public readonly RecvSimObjectData<T> SimObjectData;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly ref struct RecvWeatherObservation
    {
        readonly Recv Recv;
        public readonly uint RequestId;
        public readonly StringV Metar;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public unsafe readonly ref struct RecvCloudState
    {
        readonly Recv Recv;
        public readonly uint RequestId;
        readonly int _ArraySize;
        readonly byte _RgbData;

        public ReadOnlySpan<byte> RgbData
        {
            get
            {
                fixed (byte* b = &_RgbData)
                    return new(b, _ArraySize);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvAssignedObjectId
    {
        readonly Recv Recv;
        public readonly uint RequestId, ObjectId;
    }

    [StructLayout(LayoutKind.Explicit, Pack = 1, Size = 92)]
    public unsafe readonly struct RecvReservedKey
    {
        [FieldOffset(0)]
        readonly Recv Recv;

        [FieldOffset(12)]
        readonly sbyte _ChoiceReserved;

        [FieldOffset(42)]
        readonly sbyte _ReservedKey;

        [MaxLength(29)]
        public string ChoiceReserved
        {
            get
            {
                fixed (sbyte* b = &_ChoiceReserved)
                {
                    b[29] = 0;
                    return new(b);
                }
            }
        }

        [MaxLength(49)]
        public string ReservedKey
        {
            get
            {
                fixed (sbyte* b = &_ReservedKey)
                {
                    b[49] = 0;
                    return new(b);
                }
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvSystemState
    {
        readonly Recv Recv;
        public readonly uint RequestId, Integer;
        public readonly float Float;
        public readonly String256 String;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly ref struct RecvCustomAction
    {
        public readonly RecvEvent Event;
        public readonly Guid InstanceId;
        public readonly uint WaitForCompletion;
        public readonly StringV PayLoad;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RecvEventWeatherMode
    {
        public readonly RecvEvent Event;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public unsafe readonly ref struct RecvFacilitiesList<T> where T : unmanaged
    {
        readonly Recv Recv;
        public readonly uint RequestId;
        internal readonly int _ArraySize;
        public readonly uint EntryNumber, OutOf;
        readonly byte _Data;

        public ReadOnlySpan<T> Data
        {
            get
            {
                fixed (void* p = &_Data)
                    return new(p, _ArraySize);
            }
        }
    }
}