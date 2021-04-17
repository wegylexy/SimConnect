using FlyByWireless.SimConnect;
using System;
using System.Runtime.InteropServices;
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

await client.DefineDataAsync<LatLon>();
Console.WriteLine("defined");

var ll = await client.RequestDataOnSimObjectAsync<LatLon>(0, Period.Once);
Console.WriteLine($"({ll.Latitude}, {ll.Longitude}, {ll.Altitude})");

await client.UndefineDataAsync<LatLon>();
Console.WriteLine("undefined");

await tcs.Task;

[StructLayout(LayoutKind.Sequential, Pack = 1)]
readonly struct LatLon
{
    [DataDefinition("PLANE LATITUDE", "degrees")]
    public readonly double Latitude;

    [DataDefinition("PLANE LONGITUDE", "degrees")]
    public readonly double Longitude;

    [DataDefinition("PLANE ALTITUDE", "meters")]
    public readonly double Altitude;
}