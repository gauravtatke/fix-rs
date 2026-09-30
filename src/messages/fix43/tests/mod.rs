//! Hand-written tests for the generated FIX 4.3 typed messages (`super::generated`).
//!
//! Kept out of `generated/` so the generator's output stays pure code the freshness test can
//! reproduce byte-for-byte, and so these remain an *independent* oracle: a bug in the emitter
//! can't bake itself into the tests that are supposed to catch it. One module per generated file.

mod fields;
mod logon;
