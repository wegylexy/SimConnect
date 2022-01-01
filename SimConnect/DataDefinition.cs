using System.Reflection;

namespace FlyByWireless.SimConnect;

[AttributeUsage(AttributeTargets.Field)]
public sealed class DataDefinitionAttribute : Attribute
{
    public readonly string? DatumName, UnitsName;

    public readonly float Epsilon;

    internal int DatumId;

    public DataDefinitionAttribute(string? datumName = null, string? unitsName = null, float epsilon = 0) =>
        (DatumName, UnitsName, Epsilon) = (datumName, unitsName, epsilon);
}

[AttributeUsage(AttributeTargets.Field)]
public sealed class ClientDataDefinitionAttribute : Attribute
{
    public readonly float Epsilon;

    internal int DatumId;

    public ClientDataDefinitionAttribute(float epsilon = 0) =>
        Epsilon = epsilon;
}

public sealed class DataDefinition : IAsyncDisposable
{
    internal readonly uint DefineId;

    internal readonly (int Offset, int Size)[] Used;

    public readonly Task Added;

    readonly Func<ValueTask> _clear;

    internal DataDefinition(uint defineId, (int Offset, int Size)[] used, Task added, Func<ValueTask> undefine) =>
        (DefineId, Used, Added, _clear) = (defineId, used, added, undefine);

    public ValueTask DisposeAsync() => _clear();
}

public sealed class DataDefinitionException : System.Exception
{
    public FieldInfo Field;

    public string Definition;

    public Exception Exception;

    internal DataDefinitionException(FieldInfo field, string definition, Exception exception) :
        base($"{definition} of {field.DeclaringType?.FullName}.{field.Name} is {exception}") =>
        (Field, Definition, Exception) = (field, definition, exception);
}