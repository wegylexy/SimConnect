using FlyByWireless.SimConnect;
using FlyByWireless.SimConnect.Data;
using System.Runtime.InteropServices;

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
        await foreach (var ll in (await client.RequestDataOnSimObjectAsync<Info>(0, Period.VisualFrame, interval: 4)).WithCancellation(cts.Token))
            Console.WriteLine($"{ll.Position_deg_deg_m}");
    }
    catch (OperationCanceledException) { }
}
Console.WriteLine("undefined");

await tcs.Task;

[StructLayout(LayoutKind.Sequential, Pack = 1)]
readonly struct Info
{
    [DataDefinition("STRUCT LATLONALT")]
    public readonly LatLonAlt Position_deg_deg_m;

    [DataDefinition("PLANE PITCH DEGREES", "degrees")]
    public readonly float Pitch_deg;

    [DataDefinition("PLANE BANK DEGREES", "degrees")]
    public readonly float Roll_deg;

    [DataDefinition("PLANE HEADING DEGREES TRUE", "degrees")]
    public readonly float Yaw_deg;

    [DataDefinition("STRUCT WORLDVELOCITY")]
    public readonly XYZ LinearVelocities_ft_s;

    [DataDefinition("STRUCT WORLD ROTATION VELOCITY")]
    public readonly XYZ AngularVelocities_rad_s;

    [DataDefinition("PLANE ALT ABOVE GROUND")]
    public readonly float AbsoluteAltitude_m;

    [DataDefinition("PRESSURE ALTITUDE")]
    public readonly float PressureAltitude_m;

    [DataDefinition("TOTAL AIR TEMPERATURE", "Celsius")]
    public readonly float TAT_C;

    // TODO: Ambient_in_Cloud

    [DataDefinition("COM RECIEVE ALL")] // Microsoft spelled "receive" r-e-c-i-e-v-e
    public readonly BOOL ComRxAll;

    [DataDefinition("COM TRANSMIT:1")]
    public readonly BOOL Com1Tx;

    [DataDefinition("COM TRANSMIT:2")]
    public readonly BOOL Com2Tx;

    [DataDefinition("COM ACTIVE FREQUENCY:1", "kHz")]
    public readonly int Vhf1Active_kHz;

    [DataDefinition("COM STANDBY FREQUENCY:1", "kHz")]
    public readonly int Vhf1Standby_kHz;

    [DataDefinition("COM ACTIVE FREQUENCY:2", "kHz")]
    public readonly int Vhf2Active_kHz;

    [DataDefinition("COM STANDBY FREQUENCY:2", "kHz")]
    public readonly int Vhf2Standby_kHz;
}