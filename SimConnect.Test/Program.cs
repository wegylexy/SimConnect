using FlyByWireless.SimConnect;
using System;
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
await tcs.Task;