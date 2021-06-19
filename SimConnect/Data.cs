using System;
using System.ComponentModel.DataAnnotations;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;

namespace FlyByWireless.SimConnect.Data
{
    [Obsolete]
    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct RaceResult
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
        /// <summary>
        /// degrees
        /// </summary>
        public readonly double Latitude, Longitude;

        /// <summary>
        /// meters
        /// </summary>
        public readonly double Altitude;

        public LatLonAlt(double latitude, double longitude, double altitude) =>
            (Latitude, Longitude, Altitude) = (latitude, longitude, altitude);

        public override string ToString() =>
            $"({(Latitude < 0 ? 'S' : 'N')}{Math.Abs(Latitude)}, {(Longitude < 0 ? 'W' : 'E')}{Math.Abs(Longitude)}, {Altitude}m)";
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct XYZ
    {
        public readonly double X, Y, Z;

        public XYZ(double x, double y, double z) =>
            (X, Y, Z) = (x, y, z);
    }

    [DebuggerDisplay("{Value}")]
    [StructLayout(LayoutKind.Sequential, Size = sizeof(int))]
    public readonly struct BOOL
    {
        [DebuggerBrowsable(DebuggerBrowsableState.Never)]
        public readonly bool Value;

        public BOOL(bool value) => Value = value;

        public static implicit operator bool(BOOL boolean) => boolean.Value;

        public static implicit operator BOOL(bool value) => new(value);

        public override string ToString() => ((bool)this).ToString();
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct BCO16
    {
        public const string UnitsName = nameof(BCO16);
        public readonly int Data;

        public BCO16(int value) => Data =
            (value & 0b111) |
            ((value & 0b111_000) << 1) |
            ((value & 0b111_000_000) << 2) |
            ((value & 0b111_000_000_000) << 3);

        public override string ToString() => Data.ToString("X4");

        public static implicit operator int(BCO16 bco16) =>
            (bco16.Data & 0x7) |
            ((bco16.Data & 0x70) >> 1) |
            ((bco16.Data & 0x700) >> 2) |
            ((bco16.Data & 0x7000) >> 3);

        public static implicit operator BCO16(int value) => new(value);
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public readonly struct FrequencyBCD16
    {
        public const string UnitsName = "Frequency BCD16";

        public readonly int Data;

        FrequencyBCD16(int kHz)
        {
            _ = Math.DivRem(Math.DivRem(Math.DivRem(Math.DivRem(kHz / 10, 10, out var d4), 10, out var d3), 10, out var d2), 10, out var d1);
            Data = (d1 << 12) | (d2 << 8) | (d3 << 4) | d4;
        }

        public override string ToString() => $"1{Data >> 8:X2}.{Data & 0xFF:X2} MHz";

        public int ToKHz()
        {
            var d4 = Data & 0xF;
            return (d4 is 2 or 7 ? 5 : 0) +
                d4 * 10 +
                ((Data & 0xF0) >> 4) * 100 |
                ((Data & 0xF00) >> 8) * 1000 |
                ((Data & 0xF000) >> 12) * 10000;
        }

        public int ToHz() => ToKHz() * 1000;

        public static FrequencyBCD16 FromKHz(int kHz) => new(kHz);

        public static FrequencyBCD16 FromHz(int Hz) => new(Hz / 1000);
    }
}