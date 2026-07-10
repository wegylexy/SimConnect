# simconnect-proto

Wire encoding/decoding for Microsoft Flight Simulator's SimConnect
protocol — packet builders/parsers, enums, and the FSX/MSFS2020/MSFS2024
protocol-version negotiation table. No I/O; this crate only deals in
bytes in, bytes out.

Not usually depended on directly — the
[`flybywireless-simconnect`](https://crates.io/crates/flybywireless-simconnect)
client re-exports it as `simconnect::proto`. See that crate's README for
usage and the full project overview.

```toml
[dependencies]
simconnect-proto = { package = "flybywireless-simconnect-proto", version = "0.1" }
```

Repository: <https://github.com/wegylexy/SimConnect>
