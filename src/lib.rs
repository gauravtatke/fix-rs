#![allow(dead_code, unused_variables)]

//! fix-rs — a synchronous FIX protocol engine (FIX 4.3/4.4, v1).
//!
//! Layers: [`core`] (version-neutral wire model + data dictionary), [`session`] (the session
//! state machine), [`transport`] (synchronous networking today; an async sibling later),
//! [`messages`] (per-version typed message facades), plus the shared [`errors`], [`convert`]
//! (value⇄wire format) and [`tags`] (field→tag registry). Implement [`Application`] to plug in
//! your own logic.

pub mod application;
pub mod convert;
pub mod core;
pub mod errors;
pub mod messages;
pub mod session;
pub mod tags;
pub mod transport;

pub use application::Application;
