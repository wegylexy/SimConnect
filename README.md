# SimConnect
Purely managed SimConnect client

## Planned Features
- Emulate older versions on `VersionMismatch` exception.
- Use unmanaged `readonly ref struct` and `readonly struct`, `Span<>` and `Memory<>`, `ArrayPool<byte>.Shared`, etc. for high performance.
- Support AnyCPU, including x86, x64, and ARM64
- Rely on 0 native libraries other than .NET runtime.
- Attempt to route received exceptions to the sent source
- Connect from Android, iOS, Linux, macOS, etc. in addition to Windows (desktop).