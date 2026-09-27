#![allow(dead_code, unused_variables)]

//! fix-rs — a synchronous FIX protocol engine (FIX 4.3/4.4, v1).
//!
//! Layers: [`core`] (version-neutral wire model + data dictionary), [`session`] (the session
//! state machine), [`transport`] (synchronous networking today; an async sibling later),
//! [`messages`] (per-version typed message facades), plus the shared [`errors`], [`convert`]
//! (value⇄wire format) and [`tags`] (field→tag registry). Implement [`Application`] to plug in
//! your own logic.

// Old build-time per-field codegen (build/main.rs -> $OUT_DIR/fields.rs); consumed by
// `core::message`. Removed in task 3.9 when the standalone generator replaces it.
include!(concat!(env!("OUT_DIR"), "/mod.rs"));

pub mod application;
pub mod convert;
pub mod core;
pub mod errors;
pub mod messages;
pub mod session;
pub mod tags;
pub mod transport;

pub use application::Application;
