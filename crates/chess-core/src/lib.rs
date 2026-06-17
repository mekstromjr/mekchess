//! mekchess core domain.
//!
//! This crate owns the chess board representation and rules that every engine
//! and tool in the workspace shares. Keeping it engine-agnostic is deliberate:
//! the classical and NNUE engines, the arena, and the UCI binaries all build on
//! the *same* board type, which is what lets the in-process arena pit engines
//! against each other without serializing positions to text.
//!
//! Modules land here as Epic A (`mekchess: Core & Test Harness`) progresses:
//!
//! - `bitboard`  — the `u64`-backed set type and bit twiddling helpers.
//! - `board`     — the hybrid board (piece bitboards + occupancy + mailbox).
//! - `fen`       — FEN parse/serialize (ticket A2).
//! - `zobrist`   — incremental position hashing (ticket A3).
//! - `movegen`   — magic-bitboard legal move generation (ticket A4).
//! - `perft`     — leaf-node counting + divide, the correctness gate (A5).

/// Placeholder so the crate compiles and `cargo test` has something to run
/// before the real modules exist. Replace as Epic A fills in.
pub const NAME: &str = "chess-core";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_is_wired_up() {
        assert_eq!(NAME, "chess-core");
    }
}
