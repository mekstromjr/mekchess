//! UCI entry point for the classical engine.
//!
//! Kept deliberately thin: once `uci::run` and the `Engine` trait exist (A6),
//! this file should be little more than wiring an `engine_classical` instance
//! into the shared protocol driver.

fn main() {
    // Placeholder until the UCI loop lands (ticket A6).
    println!("{} (UCI binary scaffold)", engine_classical::ENGINE_NAME);
}
