//! Networking / transport layer.
//!
//! The synchronous implementation (`std::net` + `std::thread`) lives under `sync`. A future
//! async (Tokio) implementation would be a feature-gated sibling here, reusing the same
//! session/model stack unchanged (nothing below this layer knows about the runtime).
//! `registry` holds the session lookup (`SessionMap`); `timer` is the per-second heartbeat tick.

pub mod registry;
pub mod sync;
pub mod timer;

pub use registry::SessionMap;
pub use timer::start_timer;

/// Default bind IP for acceptor sockets.
pub(crate) const SOCKET_ACCEPT_HOST_IP: &str = "127.0.0.1";
