//! Thin library wrapper so integration tests (`tests/`) can `use backend::startgg::…`.

pub mod startgg;

// re‑export for convenience
pub use startgg::{StartggClient, StartggError};
