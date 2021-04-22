using System;
using System.Runtime.InteropServices;
using System.Text;

namespace FlyByWireless.SimConnect
{
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct Send
    {
        readonly int Size;
        readonly uint Version, Type, SendId;

        public Send(int size, uint type) =>
            (Size, Version, Type, SendId) = (size, 0, type, 0);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendOpen
    {
        readonly Send Send;
        readonly String256 ApplicationName;
        readonly ulong FSX;
        readonly int _VerionMajor, _VersionMinor, _BuildMajor, _BuildMinor;

        public unsafe SendOpen(Protocol protocol, string applicationName)
        {
            var v = protocol.SimConnectVersion;
            (Send, ApplicationName, FSX, _VerionMajor, _VersionMinor, _BuildMajor, _BuildMinor) =
                (new(sizeof(SendOpen), 0xF0000001), applicationName, 0x4653580000000000, v.Major, v.Minor, v.Build, v.Revision);
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendMapClientEventToSimEvent
    {
        readonly Send Send;
        readonly uint EventId;
        readonly String256 EventName;

        public unsafe SendMapClientEventToSimEvent(uint eventId, string? eventName) =>
            (Send, EventId, EventName) =
                (new(sizeof(SendMapClientEventToSimEvent), 0xF0000004), eventId, eventName);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendTransmitClientEvent
    {
        readonly Send Send;
        readonly uint ObjectId, EventId;
        readonly int Data;
        readonly uint GroupId;
        readonly EventFlags Flags;

        public unsafe SendTransmitClientEvent(uint objectId, uint eventId, int data, uint groupId, EventFlags flags) =>
            (Send, ObjectId, EventId, Data, GroupId, Flags) =
                (new(sizeof(SendTransmitClientEvent), 0xF0000005), objectId, eventId, data, groupId, flags);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendSetSystemEventState
    {
        readonly Send Send;
        readonly uint EventId;
        readonly State State;

        public unsafe SendSetSystemEventState(uint eventId, State state) =>
            (Send, EventId, State) =
                (new(sizeof(SendSetSystemEventState), 0xF0000006), eventId, state);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendAddClientEventToNotificationGroupAsync
    {
        readonly Send Send;
        readonly uint GroupId;
        readonly uint EventId;
        readonly int Maskable;

        public unsafe SendAddClientEventToNotificationGroupAsync(uint groupId, uint eventId, bool maskable) =>
            (Send, GroupId, EventId, Maskable) =
                (new(sizeof(SendAddClientEventToNotificationGroupAsync), 0xF0000007), groupId, eventId, maskable ? 1 : 0);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendRemoveClientEventAsync
    {
        readonly Send Send;
        readonly uint GroupId;
        readonly uint EventId;

        public unsafe SendRemoveClientEventAsync(uint groupId, uint eventId) =>
            (Send, GroupId, EventId) =
                (new(sizeof(SendRemoveClientEventAsync), 0xF0000008), groupId, eventId);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendSetNotificationGroupPriority
    {
        readonly Send Send;
        readonly uint GroupId;
        readonly GroupPriority Priority;

        public unsafe SendSetNotificationGroupPriority(uint groupId, GroupPriority priority) =>
            (Send, GroupId, Priority) =
                (new(sizeof(SendRemoveClientEventAsync), 0xF0000009), groupId, priority);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendClearNotificationGroup
    {
        readonly Send Send;
        readonly uint GroupId;

        public unsafe SendClearNotificationGroup(uint groupId) =>
            (Send, GroupId) =
                (new(sizeof(SendClearNotificationGroup), 0xF000000A), groupId);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendRequestNotificationGroup
    {
        readonly Send Send;
        readonly uint GroupId, Reserved, Flags;

        public unsafe SendRequestNotificationGroup(uint groupId, uint reserved, uint flags) =>
            (Send, GroupId, Reserved, Flags) =
                (new(sizeof(SendRequestNotificationGroup), 0xF000000B), groupId, reserved, flags);
    }

    #region SimObject Data
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendAddToDataDefinition
    {
        readonly Send Send;
        readonly uint DefineId;
        readonly String256 DatumName, UnitsName;
        readonly DataType DatumType;
        readonly float Epsilon;
        readonly int DatumId;

        public unsafe SendAddToDataDefinition(uint defineId, string datumName, string? unitsName,
            DataType datumType = DataType.Float64, float epsilon = 0, int datumId = -1) =>
            (Send, DefineId, DatumName, UnitsName, DatumType, Epsilon, DatumId) =
                (new(sizeof(SendAddToDataDefinition), 0xF000000C), defineId, datumName, unitsName, datumType, epsilon, datumId);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendClearDataDefinition
    {
        readonly Send Send;
        readonly uint DefineId;

        public unsafe SendClearDataDefinition(uint defineId) =>
            (Send, DefineId) = (new(sizeof(SendClearDataDefinition), 0xF000000D), defineId);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendRequestDataOnSimObject
    {
        readonly Send Send;
        readonly uint RequestId, DefineId, ObjectId;
        readonly Period Period;
        readonly DataRequestFlags Flags;
        readonly uint Origin, Interval, Limit;

        public unsafe SendRequestDataOnSimObject(uint requestId, uint defineId, uint objectId, Period period, DataRequestFlags flags = DataRequestFlags.Default, uint origin = 0, uint interval = 0, uint limit = 0) =>
            (Send, RequestId, DefineId, ObjectId, Period, Flags, Origin, Interval, Limit) =
                (new(sizeof(SendRequestDataOnSimObject), 0xF000000E), requestId, defineId, objectId, period, flags, origin, interval, limit);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendRequestDataOnSimObjectType
    {
        readonly Send Send;
        readonly uint RequestId, DefineId, RadiusMeters;
        readonly SimObjectType Type;

        public unsafe SendRequestDataOnSimObjectType(uint requestId, uint defineId, uint radiusMeters, SimObjectType type) =>
            (Send, RequestId, DefineId, RadiusMeters, Type) =
                (new(sizeof(SendRequestDataOnSimObjectType), 0xF000000F), requestId, defineId, radiusMeters, type);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendSetDataOnSimObject<T> where T : unmanaged
    {
        readonly Send Send;
        readonly uint DefineId, ObjectId;
        readonly DataSetFlags Flags;
        readonly int ArrayCount, UnitSize;
        readonly T Data;

        public unsafe SendSetDataOnSimObject(uint defineId, uint objectId, ReadOnlySpan<T> data)
        {
            (Send, DefineId, ObjectId, Flags, ArrayCount, UnitSize) =
                (new(sizeof(SendSetDataOnSimObject<T>) + sizeof(T) * (data.Length - 1), 0xF0000010), defineId, objectId, DataSetFlags.Default, data.Length, sizeof(T));
            fixed (T* p = &Data)
                data.CopyTo(new(p, data.Length));
        }
    }
    #endregion

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendMapInputEventToClientEvent
    {
        readonly Send Send;
        readonly uint GroupId;
        readonly String256 InputDefinition;
        readonly uint DownEventId;
        readonly int DownValue;
        readonly uint UpEventId;
        readonly int UpValue;
        readonly int Maskable;

        public unsafe SendMapInputEventToClientEvent(uint groupId, string inputDefinition, uint downEventId, int downValue = 0, uint upEventId = uint.MaxValue, int upValue = 0, bool maskable = false) =>
            (Send, GroupId, InputDefinition, DownEventId, DownValue, UpEventId, UpValue, Maskable) =
                (new(sizeof(SendMapInputEventToClientEvent), 0xF0000011), groupId, inputDefinition, downEventId, downValue, upEventId, upValue, maskable ? 1 : 0);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendSetInputGroupPriority
    {
        readonly Send Send;
        readonly uint GroupId;
        readonly GroupPriority Priority;

        public unsafe SendSetInputGroupPriority(uint groupId, GroupPriority priority) =>
            (Send, GroupId, Priority) =
                (new(sizeof(SendSetInputGroupPriority), 0xF0000012), groupId, priority);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendRemoveInputEvent
    {
        readonly Send Send;
        readonly uint GroupId;
        readonly String256 InputDefinition;

        public unsafe SendRemoveInputEvent(uint groupId, string inputDefinition) =>
            (Send, GroupId, InputDefinition) =
                (new(sizeof(SendRemoveInputEvent), 0xF0000013), groupId, inputDefinition);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendClearInputGroup
    {
        readonly Send Send;
        readonly uint GroupId;

        public unsafe SendClearInputGroup(uint groupId) =>
            (Send, GroupId) =
                (new(sizeof(SendClearInputGroup), 0xF0000014), groupId);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendSetInputGroupState
    {
        readonly Send Send;
        readonly uint GroupId;
        readonly State State;

        public unsafe SendSetInputGroupState(uint groupId, State state) =>
            (Send, GroupId, State) =
                (new(sizeof(SendSetInputGroupState), 0xF0000015), groupId, state);
    }

    [StructLayout(LayoutKind.Explicit, Pack = 1, Size = 106)]
    readonly struct SendRequestReservedKey
    {
        [FieldOffset(0)]
        readonly Send Send;

        [FieldOffset(12)]
        readonly uint EventId;

        [FieldOffset(16)]
        readonly sbyte KeyChoice1;

        [FieldOffset(46)]
        readonly sbyte KeyChoice2;

        [FieldOffset(76)]
        readonly sbyte KeyChoice3;

        public unsafe SendRequestReservedKey(uint eventId, string? keyChoice1, string? keyChoice2, string? keyChoice3)
        {
            (Send, EventId) =
                (new(sizeof(SendRequestReservedKey), 0xF0000016), eventId);
            fixed (sbyte* b = &KeyChoice1)
                b[string.IsNullOrEmpty(keyChoice1) ? 0 : Encoding.ASCII.GetBytes(keyChoice1, new(b, 29))] = 0;
            fixed (sbyte* b = &KeyChoice2)
                b[string.IsNullOrEmpty(keyChoice2) ? 0 : Encoding.ASCII.GetBytes(keyChoice2, new(b, 29))] = 0;
            fixed (sbyte* b = &KeyChoice3)
                b[string.IsNullOrEmpty(keyChoice3) ? 0 : Encoding.ASCII.GetBytes(keyChoice3, new(b, 29))] = 0;
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendSubscribeToSystemEvent
    {
        readonly Send Send;
        readonly uint EventId;
        readonly String256 EventName;

        public unsafe SendSubscribeToSystemEvent(uint eventId, string eventName) =>
            (Send, EventId, EventName) =
                (new(sizeof(SendSubscribeToSystemEvent), 0xF0000017), eventId, eventName);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendUnsubscribeToSystemEvent
    {
        readonly Send Send;
        readonly uint EventId;

        public unsafe SendUnsubscribeToSystemEvent(uint eventId) =>
            (Send, EventId) =
                (new(sizeof(SendSubscribeToSystemEvent), 0xF0000018), eventId);
    }

    #region Weather
    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendWeatherRequestInterpolatedObservation
    {
        readonly Send Send;
        readonly uint RequestId;
        readonly float Lat, Lon, Alt;

        public unsafe SendWeatherRequestInterpolatedObservation(uint requestId, float lat, float lon, float alt) =>
            (Send, RequestId, Lat, Lon, Alt) =
                (new(sizeof(SendWeatherRequestInterpolatedObservation), 0xF0000019), requestId, lat, lon, alt);
    }

    [Obsolete]
    [StructLayout(LayoutKind.Explicit, Pack = 1, Size = 21)]
    readonly struct SendWeatherRequestObservationAtStation
    {
        [FieldOffset(0)]
        readonly Send Send;
        [FieldOffset(12)]
        readonly uint RequestId;
        [FieldOffset(16)]
        readonly sbyte Icao;

        public unsafe SendWeatherRequestObservationAtStation(uint requestId, string icao, out int size)
        {
            (Send, RequestId) =
                (new(sizeof(SendWeatherRequestInterpolatedObservation), 0xF000001A), requestId);
            fixed (sbyte* b = &Icao)
                b[size = Encoding.ASCII.GetBytes(icao, new(b, 4))] = 0;
            size += 17;
        }
    }

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendWeatherRequestObservationAtNearestStation
    {
        readonly Send Send;
        readonly uint RequestId;
        readonly float Lat, Lon;

        public unsafe SendWeatherRequestObservationAtNearestStation(uint requestId, float lat, float lon) =>
            (Send, RequestId, Lat, Lon) =
                (new(sizeof(SendWeatherRequestObservationAtNearestStation), 0xF000001B), requestId, lat, lon);
    }
    #endregion

    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    readonly struct SendCameraSetRelative6DOF
    {
        readonly Send Send;
        readonly float DeltaX, DeltaY, DeltaZ,
            PitchDeg, BankDeg, HeadingDeg;

        public unsafe SendCameraSetRelative6DOF(float deltaX, float deltaY, float deltaZ, float pitchDeg, float bankDeg, float headingDeg) =>
            (Send, DeltaX, DeltaY, DeltaZ, PitchDeg, BankDeg, HeadingDeg) =
                (new(sizeof(SendCameraSetRelative6DOF), 0xF0000030), deltaX, deltaY, deltaZ, pitchDeg, bankDeg, headingDeg);
    }
}