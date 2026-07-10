# simconnect-cli

Manual smoke-test binary for
[`flybywireless-simconnect`](https://crates.io/crates/flybywireless-simconnect) —
connects to a running Microsoft Flight Simulator instance (local named
pipe on Windows, or TCP) and prints the negotiated protocol version plus
the first few inbound packets it receives.

```sh
cargo run -p simconnect-cli            # Windows named pipe
cargo run -p simconnect-cli -- 500      # TCP, port 500
```

There's no way to exercise the real named pipe/TCP transport in CI, so
this is meant to be run by hand against a live sim.

Repository: <https://github.com/wegylexy/SimConnect>
