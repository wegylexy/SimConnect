//! Transport abstraction: local named pipe or remote TCP. Genuinely
//! non-blocking — `tokio::net::TcpStream` (already real async), and on
//! Windows, tokio's own IOCP-backed
//! `tokio::net::windows::named_pipe::NamedPipeClient` (no unsafe/FFI
//! needed) — not a blocking transport moved onto a worker thread.

use std::io;
use std::time::Duration;

/// A duplex byte stream to the sim.
pub trait Transport: tokio::io::AsyncRead + tokio::io::AsyncWrite + Send + Unpin {}

impl Transport for tokio::net::TcpStream {}

#[cfg(windows)]
impl Transport for tokio::net::windows::named_pipe::NamedPipeClient {}

/// Test-only: lets `connection.rs`'s tests drive a `Connection` over an
/// in-memory `tokio::io::duplex` pair instead of a real pipe/socket.
#[cfg(test)]
impl Transport for tokio::io::DuplexStream {}

/// Default local pipe name used by the official SimConnect client/server
/// (`SimConnect.cs`'s hardcoded path in the prior C# client). Sim-version-agnostic: MSFS 2020 and
/// 2024 both use this exact well-known pipe (the official SimConnect DLL's own convention, not a
/// per-year name), so there is no separate "2024" pipe name anywhere in this module.
pub const DEFAULT_PIPE_NAME: &str = r"\\.\pipe\Microsoft Flight Simulator\SimConnect";

/// Prepar3D's own local pipe names, one per major version — confirmed live against a running
/// Prepar3D v4 install via `[System.IO.Directory]::GetFiles("\\.\pipe\")`, which showed
/// `Lockheed Martin Prepar3D v4\SimConnect` open *alongside* the unversioned/v2/v3 pipes too (P3D
/// appears to also keep every older-version pipe name open for backward compatibility with
/// clients written against them) — ordered newest-first regardless, so a real v6/v5 install's own
/// pipe wins over a stale compatibility alias rather than relying on that behavior.
#[cfg(windows)]
pub const PREPAR3D_PIPE_NAMES: &[&str] = &[
    r"\\.\pipe\Lockheed Martin Prepar3D v6\SimConnect",
    r"\\.\pipe\Lockheed Martin Prepar3D v5\SimConnect",
    r"\\.\pipe\Lockheed Martin Prepar3D v4\SimConnect",
    r"\\.\pipe\Lockheed Martin Prepar3D v3\SimConnect",
    r"\\.\pipe\Lockheed Martin Prepar3D v2\SimConnect",
    r"\\.\pipe\Lockheed Martin Prepar3D\SimConnect",
];

/// FSX/FSX:SE's own local pipe name — a wholly different naming convention from every other sim
/// here, predating the "<Sim Name>\SimConnect" pattern P3D and MSFS both use (confirmed against
/// the original SimConnect SDK's own reference; not a guess). Not caught by
/// [`scan_for_pipe_names`]'s substring match against `"SimConnect"` below, since that string
/// doesn't appear in this name at all — it needs its own explicit entry regardless of whether
/// that scan exists.
#[cfg(windows)]
pub const FSX_PIPE_NAME: &str = r"\\.\pipe\FS98MAIN";

/// Last-resort local-pipe discovery: lists every currently-open named pipe and returns the ones
/// whose name contains `needle` (case-insensitive) — e.g. `"simconnect"`, to catch a sim/version
/// this module has no hardcoded name for yet. `\\.\pipe\` is a real, listable directory on
/// Windows (confirmed the same way `PowerShell`'s `[System.IO.Directory]::
/// GetFiles("\\.\pipe\")` lists it, which is how [`PREPAR3D_PIPE_NAMES`] was itself confirmed) —
/// `std::fs::read_dir` walks it like any other directory. Order among matches is whatever
/// `read_dir` yields (not sorted/prioritized) — callers should try every sim/version they have a
/// named constant for first, and use this only to catch an installation none of those cover.
#[cfg(windows)]
pub fn scan_for_pipe_names(needle: &str) -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(r"\\.\pipe\") else {
        return Vec::new();
    };
    let needle = needle.to_ascii_lowercase();
    entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.to_ascii_lowercase().contains(&needle))
        .map(|name| format!(r"\\.\pipe\{name}"))
        .collect()
}

/// Connects a TCP transport to a remote (or loopback) SimConnect server.
pub async fn connect_tcp(host: &str, port: u16) -> io::Result<tokio::net::TcpStream> {
    let stream = tokio::net::TcpStream::connect((host, port)).await?;
    stream.set_nodelay(true)?;
    Ok(stream)
}

/// Connects a named-pipe transport (Windows only — the sim itself only
/// runs on Windows, so this is the common local-connection path),
/// retrying briefly if the pipe exists but its listener backlog is
/// momentarily full (`ERROR_PIPE_BUSY`).
#[cfg(windows)]
pub async fn connect_pipe(
    pipe_name: &str,
) -> io::Result<tokio::net::windows::named_pipe::NamedPipeClient> {
    use tokio::net::windows::named_pipe::ClientOptions;

    let mut last_err = None;
    for _ in 0..20 {
        match ClientOptions::new().open(pipe_name) {
            Ok(client) => return Ok(client),
            Err(e) if e.raw_os_error() == Some(231) /* ERROR_PIPE_BUSY */ => {
                last_err = Some(e);
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
            Err(e) => return Err(e),
        }
    }
    Err(last_err.unwrap_or_else(|| io::Error::new(io::ErrorKind::NotFound, pipe_name)))
}
