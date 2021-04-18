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

        internal int DatumId { get; set; }

        public DataDefinitionAttribute(string datumName, string unitsName, DataType datumType = DataType.Float64, float epsilon = 0) =>
            (DatumName, UnitsName, DatumType, Epsilon) = (datumName, unitsName, datumType, epsilon);
    }
}