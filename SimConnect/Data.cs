using System;
using System.ComponentModel.DataAnnotations;
using System.Runtime.InteropServices;
using System.Text;

namespace FlyByWireless.SimConnect.Data
{
    namespace Facility
    {
        [StructLayout(LayoutKind.Explicit, Pack = 1, Size = 33)]
        public unsafe readonly struct Airport
        {
            [FieldOffset(0)]
            readonly sbyte _Icao;

            /// <summary>
            /// degrees
            /// </summary>
            [FieldOffset(9)]
            public readonly double Latitude;

            /// <summary>
            /// degrees
            /// </summary>
            [FieldOffset(17)]
            public readonly double Longitude;

            /// <summary>
            /// meters
            /// </summary>
            [FieldOffset(25)]
            public readonly double Altitude;

            [MaxLength(8)]
            public string Icao
            {
                get
                {
                    fixed (sbyte* b = &_Icao)
                    {
                        b[8] = 0;
                        return new(b);
                    }
                }
            }

            public Airport(string icao, double latitude, double longitude, double altitude)
            {
                fixed (sbyte* b = &_Icao)
                    b[Encoding.Default.GetBytes(icao, new Span<byte>(b, 8))] = 0;
                (Latitude, Longitude, Altitude) = (latitude, longitude, altitude);
            }
        }

        [StructLayout(LayoutKind.Sequential, Pack = 1)]
        public readonly struct Waypoint
        {
            readonly Airport _Airport;

            /// <summary>
            /// degrees
            /// </summary>
            public readonly float MagVar;

            public string Icao => _Airport.Icao;

            /// <summary>
            /// degrees
            /// </summary>
            public double Latitude => _Airport.Latitude;

            /// <summary>
            /// degrees
            /// </summary>
            public double Longitude => _Airport.Longitude;

            /// <summary>
            /// meters
            /// </summary>
            public double Altitude => _Airport.Altitude;

            public Waypoint(string icao, double latitude, double longitude, double altitude, float magVar) =>
                (_Airport, MagVar) = (new(icao, latitude, longitude, altitude), magVar);
        }

        [StructLayout(LayoutKind.Sequential, Pack = 1)]
        public readonly struct NDB
        {
            readonly Waypoint _Waypoint;

            /// <summary>
            /// Hz
            /// </summary>
            public readonly uint Frequency;

            public string Icao => _Waypoint.Icao;

            /// <summary>
            /// degrees
            /// </summary>
            public double Latitude => _Waypoint.Latitude;

            /// <summary>
            /// degrees
            /// </summary>
            public double Longitude => _Waypoint.Longitude;

            /// <summary>
            /// meters
            /// </summary>
            public double Altitude => _Waypoint.Altitude;

            /// <summary>
            /// degrees
            /// </summary>
            public float MagVar => _Waypoint.MagVar;

            public NDB(string icao, double latitude, double longitude, double altitude, float magVar, uint frequency) =>
                (_Waypoint, Frequency) = (new(icao, latitude, longitude, altitude, magVar), frequency);
        }

        [StructLayout(LayoutKind.Sequential, Pack = 1)]
        public readonly struct VOR
        {
            readonly NDB _NDB;

            public readonly VORFlags Flags;

            /// <summary>
            /// degrees
            /// </summary>
            public readonly float Localizer;

            /// <summary>
            /// degrees
            /// </summary>
            public readonly double GlideLat, GlideLon;

            /// <summary>
            /// meters
            /// </summary>
            public readonly double GlideAlt;

            /// <summary>
            /// degrees
            /// </summary>
            public readonly float GlideSlopeAngle;

            public string Icao => _NDB.Icao;

            /// <summary>
            /// degrees
            /// </summary>
            public double Latitude => _NDB.Latitude;

            /// <summary>
            /// degrees
            /// </summary>
            public double Longitude => _NDB.Longitude;

            /// <summary>
            /// meters
            /// </summary>
            public double Altitude => _NDB.Altitude;

            /// <summary>
            /// degrees
            /// </summary>
            public float MagVar => _NDB.MagVar;

            /// <summary>
            /// Hz
            /// </summary>
            public uint Frequency => _NDB.Frequency;

            public VOR(string icao, double latitude, double longitude, double altitude, float magVar, uint frequency,
                VORFlags flags, float localizer, double glideLat, double glideLon, double glideAlt, float glideSlopeAngle) =>
                (_NDB, Flags, Localizer, GlideLat, GlideLon, GlideAlt, GlideSlopeAngle) = (
                    new(icao, latitude, longitude, altitude, magVar, frequency),
                    flags, localizer, glideLat, glideLon, glideAlt, glideSlopeAngle
                );
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct InitPosition
    {
        /// <summary>
        /// degrees
        /// </summary>
        public readonly double Latitude, Longitude;

        /// <summary>
        /// feet
        /// </summary>
        public readonly double Altitude;

        /// <summary>
        /// degrees
        /// </summary>
        public readonly double Pitch, Bank, Heading;

        readonly int _OnGround;

        public readonly uint Airspeed;

        public bool OnGround => _OnGround != 0;

        public InitPosition(double latitude, double longitude, double altitude, double pitch, double bank, double heading, bool onGround, uint airspeed) =>
            (Latitude, Longitude, Altitude, Pitch, Bank, Heading, _OnGround, Airspeed) = (latitude, longitude, altitude, pitch, bank, heading, onGround ? 1 : 0, airspeed);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public unsafe struct MarkerState
    {
        fixed sbyte _Name[64];

        readonly int _State;

        [MaxLength(63)]
        public string Name
        {
            get
            {
                fixed (sbyte* b = _Name)
                    return new(b);
            }
        }

        public bool State => _State != 0;

        public unsafe MarkerState(string name, bool state)
        {
            fixed (sbyte* b = _Name)
                b[Encoding.Default.GetBytes(name, new Span<byte>(b, 63))] = 0;
            _State = state ? 1 : 0;
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct Waypoint
    {
        /// <summary>
        /// degrees
        /// </summary>
        public readonly double Latitude, Longitude;

        /// <summary>
        /// feet
        /// </summary>
        public readonly double Altitude;

        public readonly WaypointFlags Flags;

        /// <summary>
        /// knots
        /// </summary>
        public readonly double Speed;

        /// <summary>
        /// percent
        /// </summary>
        public readonly double Throttle;

        public Waypoint(double latitude, double longitude, double altitude, WaypointFlags flags, double speed, double throttle) =>
            (Latitude, Longitude, Altitude, Flags, Speed, Throttle) = (latitude, longitude, altitude, flags, speed, throttle);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct LatLonAlt
    {
        public readonly double Latitude, Longitude, Altitude;

        public LatLonAlt(double latitude, double longitude, double altitude) =>
            (Latitude, Longitude, Altitude) = (latitude, longitude, altitude);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct XYZ
    {
        public readonly double X, Y, Z;

        public XYZ(double x, double y, double z) =>
            (X, Y, Z) = (x, y, z);
    }
}