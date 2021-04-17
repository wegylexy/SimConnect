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
}