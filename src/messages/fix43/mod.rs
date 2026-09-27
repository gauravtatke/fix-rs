//! FIX 4.3 typed messages — hand-written reference for the code generator (task 3.1).
//!
//! Per-version module: holds FIX 4.3's typed message facades. Version-neutral pieces live
//! outside this dir and are shared by every `crate::fixNN` module:
//! - field tag numbers → `crate::tags`
//! - the typed-conversion error → `crate::errors::TypedError`
//! - FIX value/format conversion (bool `Y`/`N`, datetimes, …) → `crate::convert`
//!
//! Shape per message (design log D1–D9): a newtype **facade over `Message`** with named
//! accessors taking each field's natural Rust type (D2), real enums for value-constrained
//! fields (D3), `Result` getters (D5), and `TryFrom<Message>` / `From<_> for Message`
//! seams.

pub mod fields;
pub mod logon;

// Public surface of this version module; consumed once the app migrates to typed
// messages (task 3.10), so unused for now.
#[allow(unused_imports)]
pub use self::{fields::EncryptMethod, logon::Logon};
