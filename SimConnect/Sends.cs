using System.Runtime.InteropServices;

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
    readonly struct SendAddToDataDefinition
    {
        readonly Send Send;
        readonly uint DefineId;
        readonly String256 DatumName, UnitsName;
        readonly DataType DatumType;
        readonly float Epsilon;
        readonly uint DatumId;

        public unsafe SendAddToDataDefinition(uint defineId, string datumName, string unitsName,
            DataType datumType = DataType.Float64, float epsilon = 0, uint datumId = uint.MaxValue) =>
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
}