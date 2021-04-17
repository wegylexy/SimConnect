using System;
using System.Runtime.InteropServices;
using System.Text;

namespace FlyByWireless.SimConnect
{
    [StructLayout(LayoutKind.Sequential, Pack = 1, Size = 8)]
    public unsafe readonly struct String8
    {
        readonly sbyte _first;

        public String8(string? value)
        {
            fixed (sbyte* b = &_first)
                b[string.IsNullOrEmpty(value) ? 0 : Encoding.Default.GetBytes(value, new Span<byte>(b, 7))] = 0;
        }

        public static implicit operator String8(string? value) => new(value);

        public static implicit operator string(in String8 string8) => string8.ToString();

        public unsafe override string ToString()
        {
            fixed (sbyte* b = &_first)
            {
                b[7] = 0;
                return new(b);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1, Size = 32)]
    public unsafe readonly struct String32
    {
        readonly sbyte _first;

        public String32(string? value)
        {
            fixed (sbyte* b = &_first)
                b[string.IsNullOrEmpty(value) ? 0 : Encoding.Default.GetBytes(value, new Span<byte>(b, 31))] = 0;
        }

        public static implicit operator String32(string? value) => new(value);

        public static implicit operator string(in String32 string32) => string32.ToString();

        public unsafe override string ToString()
        {
            fixed (sbyte* b = &_first)
            {
                b[31] = 0;
                return new(b);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1, Size = 64)]
    public unsafe readonly struct String64
    {
        readonly sbyte _first;

        public String64(string? value)
        {
            fixed (sbyte* b = &_first)
                b[string.IsNullOrEmpty(value) ? 0 : Encoding.Default.GetBytes(value, new Span<byte>(b, 63))] = 0;
        }

        public static implicit operator String64(string? value) => new(value);

        public static implicit operator string(in String64 string64) => string64.ToString();

        public unsafe override string ToString()
        {
            fixed (sbyte* b = &_first)
            {
                b[63] = 0;
                return new(b);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1, Size = 128)]
    public unsafe readonly struct String128
    {
        readonly sbyte _first;

        public String128(string? value)
        {
            fixed (sbyte* b = &_first)
                b[string.IsNullOrEmpty(value) ? 0 : Encoding.Default.GetBytes(value, new Span<byte>(b, 127))] = 0;
        }

        public static implicit operator String128(string? value) => new(value);

        public static implicit operator string(in String128 string128) => string128.ToString();

        public unsafe override string ToString()
        {
            fixed (sbyte* b = &_first)
            {
                b[127] = 0;
                return new(b);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1, Size = 256)]
    public unsafe readonly struct String256
    {
        readonly sbyte _first;

        public String256(string? value)
        {
            fixed (sbyte* b = &_first)
                b[string.IsNullOrEmpty(value) ? 0 : Encoding.Default.GetBytes(value, new Span<byte>(b, 255))] = 0;
        }

        public static implicit operator String256(string? value) => new(value);

        public static implicit operator string(in String256 string256) => string256.ToString();

        public unsafe override string ToString()
        {
            fixed (sbyte* b = &_first)
            {
                b[255] = 0;
                return new(b);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1, Size = 260)]
    public unsafe readonly struct String260
    {
        readonly sbyte _first;

        public String260(string? value)
        {
            fixed (sbyte* b = &_first)
                b[string.IsNullOrEmpty(value) ? 0 : Encoding.Default.GetBytes(value, new Span<byte>(b, 259))] = 0;
        }

        public static implicit operator String260(string? value) => new(value);

        public static implicit operator string(in String260 string260) => string260.ToString();

        public unsafe override string ToString()
        {
            fixed (sbyte* b = &_first)
            {
                b[259] = 0;
                return new(b);
            }
        }
    }

    [StructLayout(LayoutKind.Sequential, Pack = 1)]
    public unsafe readonly ref struct StringV
    {
        readonly sbyte _first;

        public static implicit operator string(in StringV stringV) => stringV.ToString();

        public override string ToString()
        {
            fixed (sbyte* b = &_first)
                return new(b);
        }
    }
}