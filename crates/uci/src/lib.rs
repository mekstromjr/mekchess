//! UCI protocol plumbing (ticket A6).
//!
//! UCI is the text protocol every chess GUI, match runner (fast-chess), and the
//! lichess bot bridge speak. The engine is *stateless* between commands: the GUI
//! sends a full `position` before every `go`, so the engine never has to
//! remember prior game state. That property is why we put this in its own crate
//! and have each engine implement a single trait — the protocol loop is shared,
//! the search is not.
//!
//! The intended shape: define an `Engine` trait (`new_game`, `set_position`,
//! `go`, `stop`) here, then `engine-classical` and `engine-nnue` each implement
//! it and hand an instance to a `run()` driver that owns the stdin/stdout loop.

/// Marker for the protocol version this crate targets.
pub const PROTOCOL: &str = "uci";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_is_wired_up() {
        assert_eq!(PROTOCOL, "uci");
        assert_eq!(chess_core::NAME, "chess-core");
    }
}
