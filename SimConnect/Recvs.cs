using FlyByWireless.SimConnect.Data;
using System;
using System.ComponentModel.DataAnnotations;
using System.Runtime.InteropServices;
using System.Text;

namespace FlyByWireless.SimConnect
{
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct Recv
    {
        public readonly int Size, Version;
        public readonly RecvId Id;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvException
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
    readonly struct RecvQuit
    {
        readonly Recv Recv;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEvent
    {
        internal const uint UnknownGroup = uint.MaxValue;
        readonly Recv Recv;
        internal readonly uint GroupId, EventId;
        public readonly int Data;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventFileName
    {
        internal readonly RecvEvent Event;
        public readonly String256 FileName;
        public readonly uint Flags;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventObjectAddRemove
    {
        internal readonly RecvEvent Event;
        public readonly SimObjectType ObjType;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventFrame
    {
        internal readonly RecvEvent Event;
        public readonly float FrameRate, SimSpeed;
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventMultiplayerServerStarted
    {
        internal readonly RecvEvent Event;
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventMultiplayerClientStarted
    {
        internal readonly RecvEvent Event;
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventMultiplayerSessionEnded
    {
        internal readonly RecvEvent Event;
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventRaceEnd
    {
        internal readonly RecvEvent Event;
        public readonly uint RacerNumber;
        public readonly RaceResult RacerData;
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventRaceLap
    {
        internal readonly RecvEvent Event;
        public readonly uint LapIndex;
        public readonly RaceResult RacerData;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    unsafe readonly ref struct RecvData
    {
        readonly Recv Recv;
        public readonly uint RequestId, ObjectId, DefineId;
        public readonly DataRequestFlags Flags;
        public readonly uint EntryNumber, OutOf;
        readonly int _DefineCount, _Data;

        public unsafe ReadOnlySpan<byte> Data
        {
            get
            {
                fixed (void* p = &_Data)
                    return new(p, Recv.Size - 40);
            }
        }
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly ref struct RecvWeatherObservation
    {
        readonly Recv Recv;
        public readonly uint RequestId;
        public readonly StringV Metar;
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    unsafe readonly ref struct RecvCloudState
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
    readonly struct RecvAssignedObjectId
    {
        readonly Recv Recv;
        public readonly uint RequestId, ObjectId;
    }

    [StructLayout(LayoutKind.Explicit, Pack = 1, Size = 92)]
    unsafe readonly struct RecvReservedKey
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
                    var length = 0;
                    while (length < 30)
                        if (b[length++] == 0)
                            break;
                    return new(b, 0, length, Encoding.ASCII);
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
                    var length = 0;
                    while (length < 50 && b[length] != 0)
                        ++length;
                    return new(b, 0, length, Encoding.ASCII);
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

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly ref struct RecvCustomAction
    {
        public readonly RecvEvent Event;
        public readonly Guid InstanceId;
        public readonly uint WaitForCompletion;
        public readonly StringV PayLoad;
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct RecvEventWeatherMode
    {
        public readonly RecvEvent Event;
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    unsafe readonly ref struct RecvFacilitiesList<T> where T : unmanaged
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