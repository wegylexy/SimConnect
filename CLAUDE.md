# CLAUDE.md

Guidance for working on this repo (Claude Code or otherwise).

## What this is

A from-scratch, pure-Rust port of the prior C# SimConnect client (see
`main`/`dev` for that history) — no `SimConnect.dll` dependency, meant to
be statically linked into other Rust binaries. Targets the SimConnect
wire protocol across FSX through MSFS 2020 ("KittyHawk")/2024 ("SunRise"),
with the FSX-era protocol range as the baseline and MSFS2020/2024 as
additional feature-gated capability (`kittyhawk`/`sunrise` Cargo
features, on by default).

## Layout

Directory names and Cargo dependency keys throughout this workspace stay
`simconnect`/`simconnect-proto`/`simconnect-derive`/`simconnect-cli` — the
crates.io *published* package names are prefixed `flybywireless-` (the
plain `simconnect` name is taken) via `[package] name` in each `Cargo.toml`,
aliased back with `package = "flybywireless-..."` on every dependency edge
so no code changes. `simconnect/Cargo.toml` additionally sets `[lib] name
= "simconnect"` (without it, the compiled crate name would default to
`flybywireless_simconnect`, breaking every `::simconnect::...` path the
derive macro's generated code hardcodes, plus this crate's own
examples/tests). If you rename any crate again, keep the actual compiled
lib name unchanged unless you also update `simconnect-derive/src/lib.rs`'s
generated paths.

- `simconnect-proto/` — no I/O. Wire encoding/decoding only: `codec.rs`
  (byte cursor), `send.rs`/`recv.rs` (packet builders/parsers), `enums.rs`,
  `protocol.rs` (version negotiation table), `strings.rs` (fixed-width
  Latin-1 strings, incl. `String8`..`String260`), `bcd.rs` (packed BCD/octal
  digit types), `data.rs` (`Waypoint`/`LatLonAlt`/`Xyz`/`MarkerState`),
  `data_definition.rs` (the `DataDefinition` trait + `FieldSpec`, shared by
  the derive macro and the runtime client).
- `simconnect-derive/` — the `#[derive(DataDefinition)]` proc macro. Must
  stay a separate crate (`proc-macro = true`); re-exported from
  `simconnect` rather than depended on directly by consumers.
- `simconnect/` — the client, `async fn` end to end (tokio is an
  unconditional dependency; there's no sync API and no `async` feature
  flag): `transport.rs` (named pipe / TCP, real non-blocking I/O —
  `tokio::net::TcpStream`, or tokio's own IOCP-backed `NamedPipeClient` on
  Windows), `cfg.rs` (`SimConnect.cfg` discovery), `connection.rs`
  (negotiation + framing; read and write sides are split via
  `tokio::io::split` into independently-`tokio::sync::Mutex`-locked
  halves, so a `recv()`/`recv_ref()` guard held open while parsing never
  blocks a concurrent `send()` — see
  `connection::tests::recv_guard_does_not_block_concurrent_send`.
  `Connection::recv` reuses one `read_buf: Vec<u8>` across calls instead
  of allocating fresh per packet — the read side of a zero-copy pass; the
  write side (`PacketWriter::new` in `simconnect_proto::send`) still
  allocates fresh per call, left alone since pooling it would mean
  threading a reusable buffer through all ~30 send builder signatures for
  a much lower-frequency code path), `client.rs` (public API —
  `SimConnect` holds a plain `Arc<Connection>`, no outer mutex, and its
  methods take `&self` not `&mut self`, since all the locking is internal
  to `Connection`; `recv_ref` returns `connection::RecvGuard` borrowing
  `Connection`'s read buffer directly, `recv` is the allocating
  `recv_ref().await?.to_vec()` convenience), `data_definition.rs`
  (runtime side of the derive macro — note `DataDefinitionGuard::drop`
  can't `await`, so its `ClearDataDefinition` cleanup is a
  `tokio::runtime::Handle::try_current()` + `handle.spawn(...)`
  fire-and-forget task, silently skipped if no runtime is current at drop
  time).
- `simconnect-cli/` — manual smoke-test binary; there's no way to exercise
  the real named pipe/TCP transport in CI, so this is run by hand against a
  live sim.
- `simconnect/examples/data_definition.rs` — runnable derive-macro demo.
- `simconnect/tests/data_definition.rs` — encode/decode/schema round-trip
  tests for the derive macro that don't need a live sim.

## Wire-format gotchas

0. **The header is asymmetric: 12 bytes inbound, 16 bytes outbound — don't
   assume they're the same.** Server-to-client packets are `size`(i32) +
   `version`(u32) + `id`/opcode(u32), 12 bytes — this is `PacketReader::header`,
   the one every real `parse_*` call site uses. Client-to-server packets add a
   4th field, `send_id`(u32), 16 bytes total — this is what `PacketWriter::new`/
   `finish` build, and it's what the sim echoes back in
   `RECV_EXCEPTION.dwSendID` for correlation (`recv::parse_exception`, not the
   generic header). This crate originally had `PacketReader::header` consume
   all 4 fields on the inbound side too, which is wrong but easy to not
   notice: it doesn't error, it just shifts every subsequent field by 4 bytes
   and produces a plausible-looking wrong value instead of an error. Caught
   only by decoding a live MSFS2024 capture: `RECV_SIMOBJECT_DATA.define_id`
   and `RECV_EXCEPTION.exception` (`NameUnrecognized` for a bogus datum name,
   `CreateObjectFailed` for an invalid AI object title) only matched what was
   actually sent/expected once the 4th field was dropped from
   `PacketReader::header`. All of `recv.rs`'s own tests were self-referential
   (`PacketWriter` build → `PacketReader` read, sharing the same wrong
   assumption on both sides) and couldn't have caught this — they now build
   fake inbound packets with `PacketWriter::new_inbound`/`finish_inbound`
   (the 12-byte shape) instead of `new`/`finish` (the 16-byte outbound
   shape), and outbound-packet tests that need the 4th field use
   `PacketReader::outbound_header` instead of `header`. If a newly-added
   `parse_*` function's fields come back offset by exactly 4 bytes, or a
   decoded value is a plausible-but-wrong enum/id, check which header shape
   the test/code is assuming.
1. **Strings are Latin-1, not ASCII or UTF-8.** Fixed-width `SIMCONNECT_STRINGnn`
   fields can contain byte values above 0x7F (titles, liveries, ICAO codes).
   ASCII mangles them; Latin-1 round-trips every byte losslessly. See
   `simconnect-proto/src/strings.rs`.
2. **Don't reuse a transport across a rejected `Open`.** Once the sim has
   replied `VersionMismatch` on a connection, it will not accept a second
   `Open` on that same pipe/socket — reconnect fresh for each entry in the
   negotiation table. `connection::Connection::open` does this by taking a
   `connect` closure it calls once per attempt.
3. **No `#[repr(packed)]` field references.** The wire format is a flat
   byte layout with no alignment padding. `#[repr(packed)]` + `&field` is
   undefined behavior in Rust when a multi-byte field ends up at an
   unaligned offset. This crate uses an explicit byte cursor
   (`codec::PacketWriter`/`PacketReader`) instead — slower to write but
   never UB, and identical on every target.
4. **Protocol version build numbers are provenance-sensitive.** The
   `ProtocolVersion` table in `simconnect-proto/src/protocol.rs` (RTM, SP1,
   SP2, Msfs) is cross-checked against the official SDK's documented
   major-version cumulative numbering (FSX=10, MSFS2020=11, MSFS2024=12)
   plus independently-observed exact build numbers
   (`protocol::tests::msfs_build_number` pins the announced one down). The
   table has one entry per accepted packet format and never identifies a
   product: MSFS 2020 accepts the 2024 quadruple, so the 2020 entry it used
   to carry was unreachable and, worse, invited consumers to read a
   negotiated value as the sim's release. `SimProduct` answers that from the
   sim's own `szApplicationName`. This project is MIT and
   deliberately doesn't cite or attribute any LGPL-licensed project as a
   source in docs or code, even for facts (only copied *code* creates a
   licensing obligation, but the project's policy is to keep the citation
   trail clean of LGPL sources regardless). If you add or correct an
   entry, cite where the build number came from in the commit message
   without naming an LGPL source; these are the kind of numbers that are
   easy to silently get wrong and hard to notice until a real sim rejects
   the handshake.

## What's implemented vs. deferred

FSX-era coverage (protocol negotiation, the core opcode/message set) plus
MSFS2020 additions (facility data, jetway data, controllers list,
input-event enumeration, extended events — behind the `kittyhawk`
feature), the `#[derive(DataDefinition)]` macro, and a fully async client
(`tokio::net::TcpStream`/`NamedPipeClient`, no blocking I/O, no separate
sync API) are done. Still open: `FlowEvent`, `GetInputEvent`/
`EnumerateInputEventParams`, camera API, CommBus, legacy facility lists,
and more — see `GAPS.md`. Don't claim support for any unimplemented item
without adding the corresponding wire structs and a real-sim smoke test —
verifying past FSX against a real sim, for every opcode/struct in this
crate, is still outstanding.

## Feature-flag gotcha: unify path dependencies with `default-features = false`

Every internal path dependency between this workspace's own crates
(`simconnect` → `simconnect-proto`, `simconnect-cli` → `simconnect`) must
set `default-features = false` and forward `kittyhawk`/`sunrise` through
the dependent crate's *own* features (`kittyhawk = ["simconnect-proto/
kittyhawk"]`, and `sunrise` implying `kittyhawk` at every level, not just
inside `simconnect-proto`). Without this, Cargo's feature unification
re-enables the dependency's default features (`kittyhawk`+`sunrise`)
regardless of `--no-default-features` on an outer `cargo build/test
--workspace` invocation, because the *other* workspace member's
unqualified dependency edge still requests them — verify with `cargo test
-p simconnect-proto --no-default-features` vs. `cargo test --workspace
--no-default-features` and compare test counts; they should match. If you
add a new workspace member with a path dependency on another member that
has feature flags, apply the same pattern immediately — don't wait to
notice the leak.

## Verification

- `cargo build --workspace` and `cargo build --workspace --no-default-features --features kittyhawk` for the feature matrix.
- `cargo test -p simconnect-proto` covers wire encode/decode; no simulator needed.
- `cargo run -p simconnect-cli` (Windows, sim running) or
  `cargo run -p simconnect-cli -- <port>` (TCP) is the only way to verify
  against a real `Open` handshake — do this before claiming a
  protocol-version or opcode change works.
