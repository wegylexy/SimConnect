using FlyByWireless.SimConnect;
using FlyByWireless.SimConnect.Data;
using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;

TaskCompletionSource tcs = new();
using SimConnect client = new();
client.UncaughtException += (_, e) => Console.Error.WriteLine(e.Message);
client.Quit += (_) =>
{
    Console.WriteLine("Quit received.");
    tcs.SetResult();
};
var open = await client.OpenAsync("Test");
Console.WriteLine($"Application Name:\t{open.ApplicationName}");
Console.WriteLine($"Application Version:\t{open.ApplicationVersion}");
Console.WriteLine($"SimConnect Version:\t{open.SimConnectVersion}");

for (var i = 0u; i < 20; ++i)
    _ = (await client.MapClientEventToSimEventAsync(i, (_, e) => { }, "Custom.Test")).Mapped
        .ContinueWith(task => Console.Error.WriteLine(task.Exception), TaskContinuationOptions.OnlyOnFaulted);

_ = (await client.SubscribeToSystemEventAsync((_, e) =>
{
    Console.WriteLine("1sec");
}, "1sec")).Mapped.ContinueWith(task => Console.Error.WriteLine(task.Exception), TaskContinuationOptions.OnlyOnFaulted);

{
    await using var defined = await client.DefineDataAsync<Info>();
    _ = defined.Added.ContinueWith(task =>
    {
        if (task.IsFaulted)
            Console.Error.WriteLine(task.Exception);
        else
            Console.WriteLine("defined");
    });

    CancellationTokenSource cts = new(10000);
    try
    {
        await foreach (var ll in (await client.RequestDataOnSimObjectAsync<Info>(0, Period.Once, DataRequestFlags.Tagged, limit: 5)).WithCancellation(cts.Token))
            Console.WriteLine($"{ll.Title}: {ll.Struct_LatLonAlt}, {ll.AltitudeFt}ft = {ll.Plane_Altitude}m, ground={ll.Ground_Altitude}m");
    }
    catch (OperationCanceledException) { }
}
Console.WriteLine("undefined");

await tcs.Task;

[StructLayout(LayoutKind.Sequential, Pack = 1)]
struct Info
{
    public String260 Title;

    [DataDefinition("PLANE ALTITUDE", "feet")]
    public double AltitudeFt;

    public LatLonAlt Struct_LatLonAlt;

    public double Plane_Altitude;

    [DataDefinition(unitsName: "meters")]
    public double Ground_Altitude;

    public readonly BOOL Com_Recieve_All;

    [DataDefinition("COM TRANSMIT:1")]
    public readonly BOOL Com1Tx;

    [DataDefinition("COM TRANSMIT:2")]
    public readonly BOOL Com2Tx;

    [DataDefinition("COM ACTIVE FREQUENCY:1", "kHz")]
    public int ComActiveFrequency1_kHz;

    [DataDefinition("COM ACTIVE FREQUENCY:1", FrequencyBCD16.UnitsName)]
    public FrequencyBCD16 ComActiveFrequency1_bcd16;

    [DataDefinition("TRANSPONDER CODE:1")]
    public int TransponderCode1_dco;

    [DataDefinition("TRANSPONDER CODE:1", BCO16.UnitsName)]
    public BCO16 TransponderCode1_bco16;
}