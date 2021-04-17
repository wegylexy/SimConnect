using System;

namespace FlyByWireless.SimConnect
{
    [AttributeUsage(AttributeTargets.Field)]
    public sealed class DataDefinitionAttribute : Attribute
    {
        public string DatumName { get; }

        public string UnitsName { get; }

        public DataType DatumType { get; }

        public float Epsilon { get; }

        public uint DatumId { get; }

        public DataDefinitionAttribute(string datumName, string unitsName, DataType datumType = DataType.Float64, float epsilon = 0, uint datumId = uint.MaxValue) =>
            (DatumName, UnitsName, DatumType, Epsilon, DatumId) = (datumName, unitsName, datumType, epsilon, datumId);
    }
}