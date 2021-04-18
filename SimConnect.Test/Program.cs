using FlyByWireless.SimConnect;
using System;
using System.Runtime.InteropServices;
using System.Threading;
using System.Threading.Tasks;

TaskCompletionSource tcs = new();
using SimConnect client = new();
client.OnRecvQuit += (_, _) =>
{
    Console.WriteLine("Quit received.");
    tcs.SetResult();
};
var open = await client.OpenAsync("Test");
Console.WriteLine($"Application Name:\t{open.ApplicationName}");
Console.WriteLine($"Application Version:\t{open.ApplicationVersion}");
Console.WriteLine($"SimConnect Version:\t{open.SimConnectVersion}");

{
    await using var defined = await client.DefineDataAsync<Pose>();
    Console.WriteLine("defined");

    CancellationTokenSource cts = new(10000);
    try
    {
        await foreach (var ll in (await client.RequestDataOnSimObjectAsync<Pose>(0, Period.Once, DataRequestFlags.Tagged, limit: 5)).WithCancellation(cts.Token))
            Console.WriteLine($"({ll.Latitude}, {ll.Longitude}, {ll.Altitude}), {ll.TerrainElevationFt}ft = {ll.TerrainElevationM}m");
    }
    catch (OperationCanceledException) { }
}
Console.WriteLine("undefined");

await tcs.Task;

[StructLayout(LayoutKind.Sequential, Pack = 1)]
readonly struct Pose
{
    [DataDefinition("GROUND ALTITUDE", "feet", DataType.Int32)]
    public readonly int TerrainElevationFt;

    [DataDefinition("PLANE LATITUDE", "degrees")]
    public readonly double Latitude;

    [DataDefinition("GROUND ALTITUDE", "meters", DataType.Int32)]
    public readonly int TerrainElevationM;

    [DataDefinition("PLANE LONGITUDE", "degrees")]
    public readonly double Longitude;

    [DataDefinition("PLANE ALTITUDE", "meters")]
    public readonly double Altitude;
}