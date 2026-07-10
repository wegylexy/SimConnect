# simconnect-derive

The `#[derive(DataDefinition)]` proc macro for
[`flybywireless-simconnect`](https://crates.io/crates/flybywireless-simconnect) —
compile-time codegen for registering a struct's fields as a SimConnect
data definition and decoding/encoding `RECV_SIMOBJECT_DATA` payloads,
instead of the runtime reflection the prior C# client used.

Not usually depended on directly — `simconnect` re-exports this derive
macro, so `use simconnect::DataDefinition;` brings in both the trait and
the macro. See that crate's README for the full usage example and
project overview.

```toml
[dependencies]
simconnect-derive = { package = "flybywireless-simconnect-derive", version = "0.1" }
```

Repository: <https://github.com/wegylexy/SimConnect>
