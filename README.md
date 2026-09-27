# fix-rs

A synchronous [FIX protocol](https://www.fixtrading.org/) engine in Rust — **acceptor and initiator**, FIX 4.3.

> **Status: work in progress toward a v1.0 release.** The engine runs real sessions end-to-end
> against QuickFIX/J today (see [What works](#what-works-today-v1)), but sequence-recovery
> (resend) and the typed message layer are still in progress.

**This is a personal learning project** (learning Rust), built with product merit as the primary
driver — see [ROADMAP.md](ROADMAP.md) for goals and scope. It draws heavy inspiration from
[QuickFIX/J](https://www.quickfixj.org/) ([source](https://github.com/quickfix-j/quickfixj)) —
the Java FIX engine — particularly its session model, `[Default]` + per-session config merge, and
`Application` callback design, adapted to idiomatic Rust.

## What works today (v1)

- **Sessions**: Logon / Heartbeat / TestRequest / Logout, sequence-number and CompID checks,
  and session scheduling (start/end day + time, timezone).
- **Both roles**: **acceptor** and **initiator** over synchronous `std::net` TCP + threads. The
  initiator **auto-reconnects** and re-logs-on after a drop.
- **Message layer**: parsing & serialization with correct field ordering, body-length/checksum
  verification, and dictionary-driven validation (required fields, tag validity, enum values,
  data formats, repeating-group counts). Malformed / non-compliant input is rejected, not
  silently accepted.
- **Automatic error responses**: the engine emits `Reject (35=3)` / `Logout (35=5)` /
  silent-drop per FIX 4.3 rules for the errors it must handle itself; recoverable errors keep
  the connection alive.
- **Application messages** via the `Application` trait — verified live against a QuickFIX/J
  counterparty: `NewOrderSingle (35=D)` → `ExecutionReport (35=8)` ack + fill, and
  `MarketDataRequest (35=V)` → `MarketDataSnapshotFullRefresh (35=W)`.
- **Config**: `toml` + `serde` — one `[Default]` block plus N `[[Session]]` overrides.

## In progress / not yet

- **Typed message codegen (v1.1, WIP)** — today you build and read messages with the raw
  `Message` + tag API. A generated typed layer (`Logon`, `NewOrderSingle`, … with typed
  accessors, value enums, and `Decimal` money fields) is being built. See [ROADMAP.md](ROADMAP.md).
- **Resend / sequence recovery (v1.2)** — `ResendRequest` / gap fill / sequence-number
  persistence are **not** implemented; a too-high inbound sequence number currently only logs a
  warning. Do not rely on gap recovery yet.
- **FIX 4.4 and async (Tokio)** — post-1.0 (v2). Today is FIX 4.3 and synchronous by design.

## Architecture

A version-neutral core with per-version messages layered on top:

| Module | Responsibility |
|---|---|
| `core/` | Wire model (`Message` / `FieldMap` / `Group`) + runtime `DataDictionary` (parses `resources/FIX43.xml`) |
| `session/` | Session state machine (`Session`, `SessionState`, identity, schedule, settings) |
| `transport/` | Synchronous networking (`sync/` acceptor + initiator), `SessionMap` registry, heartbeat timer (an async sibling is planned here) |
| `messages/` | Per-version typed message layer (`fix43/`; WIP) |
| `application`, `errors`, `convert`, `tags` | The `Application` trait (your integration point), error types, FIX value⇄Rust conversion, and the field→tag registry |

The package builds a **library crate** (`fix_rs`, the engine) and a **demo binary**
(`src/main.rs` + `SampleApp`).

## Build & run

```sh
cargo build     # build the library + demo binary
cargo test      # run the test suite
cargo run       # run the demo, reading src/FixConfig.toml
```

Config lives in `src/FixConfig.toml`; every supported setting is documented in
**[`src/FixConfig.example.toml`](src/FixConfig.example.toml)**.

## Using it as a library

The integration point is the [`Application`](src/application.rs) trait: implement it, hand it to
your sessions, and run the transport. Building/reading messages currently uses the raw `Message`
API (the typed accessors are the WIP v1.1 layer).

### 1. Implement `Application`

```rust
use fix_rs::Application;
use fix_rs::core::message::Message;
use fix_rs::errors::{BusinessMsgReject, DonotSend, RejectLogon};
use fix_rs::session::SessionId;

struct MyApp;

impl Application for MyApp {
    fn on_create(&mut self, _sid: &SessionId) {}
    fn on_logon(&mut self, sid: &SessionId) { log::info!("logged on: {sid}"); }
    fn on_logout(&mut self, _sid: &SessionId) {}

    // Adjust outbound admin messages before they leave (e.g. add Logon credentials).
    fn on_admin_msg_sending(&mut self, _sid: &SessionId, _msg: &mut Message) {}

    // Inbound admin message; return Err(RejectLogon) to reject a Logon.
    fn on_admin_msg_received(&mut self, _sid: &SessionId, _msg: &Message)
        -> Result<(), RejectLogon> { Ok(()) }

    // Outbound app message; return Err(DonotSend) to cancel the send.
    fn on_app_msg_sending(&mut self, _sid: &SessionId, _msg: &mut Message)
        -> Result<(), DonotSend> { Ok(()) }

    // Inbound app message → return the messages to send back. The engine stamps the
    // header (CompIDs, MsgSeqNum, SendingTime) and serializes each one; you build the body.
    fn on_app_msg_received(&mut self, _sid: &SessionId, msg: &Message)
        -> Result<Vec<Message>, BusinessMsgReject>
    {
        match msg.get_header_field::<String>(35).as_deref() {
            // NewOrderSingle (35=D) → ExecutionReport (35=8) "New" ack
            Ok("D") => {
                let mut ack = Message::new();
                ack.set_header_field(35, "8");
                ack.set_body_field(39, "0"); // OrdStatus = New
                ack.set_body_field(150, "0"); // ExecType = New
                // … remaining required ExecutionReport fields …
                Ok(vec![ack])
            }
            _ => Ok(vec![]),
        }
    }
}
```

### 2. Wire up and run

The engine runs from a `SessionMap` (the shared registry of all sessions), a heartbeat timer,
and one transport thread per bind/connect address. A complete, runnable version of the snippet
below is in [`examples/minimal_acceptor.rs`](examples/minimal_acceptor.rs)
(`cargo run --example minimal_acceptor`); [`src/main.rs`](src/main.rs) is the canonical runner
that also handles the initiator role. A minimal acceptor:

```rust
use fix_rs::session::{ConnectionType, SessionProperties};
use fix_rs::transport::{SessionMap, SOCKET_ACCEPT_HOST_IP, start_timer};
use fix_rs::transport::sync::acceptor::IoAcceptor;
use std::net::{IpAddr, SocketAddr};

let cfg = std::fs::read_to_string("src/FixConfig.toml")?;
let props = SessionProperties::from_str(&cfg)?;

// One registry, shared by the timer and every transport.
let sessions: SessionMap = props
    .sessions()
    .iter()
    .map(|(id, c)| (id.clone(), c.to_session(Box::new(MyApp))))
    .collect();

start_timer(sessions.clone()); // heartbeats & timeouts, on a background thread

// Bind each acceptor session. IoAcceptor::start() blocks; main.rs spawns a
// thread per address (and also handles the initiator role) — see it for the full runner.
for c in props.sessions().values()
    .filter(|c| c.connection_type() == ConnectionType::Acceptor)
{
    let addr = SocketAddr::new(
        SOCKET_ACCEPT_HOST_IP.parse::<IpAddr>().unwrap(),
        c.socket_accept_port().unwrap(),
    );
    IoAcceptor::new(addr, sessions.clone()).start()?;
}
```

## Project docs

- **[ROADMAP.md](ROADMAP.md)** — goals, scope, and the v1 → v1.1 → v1.2 → v1.0 plan.

## License

See [LICENSE](LICENSE).
