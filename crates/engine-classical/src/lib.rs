//! Classical chess engine: alpha-beta search over a handcrafted evaluation.
//!
//! Epic C builds this out: material + piece-square eval, negamax + alpha-beta,
//! quiescence, iterative deepening, a transposition table, move ordering, and
//! the usual search reductions. All of it reads/writes the shared
//! [`chess_core`] board, so the arena can drive it as a plain Rust call.

/// Identifies this engine to the UCI `id name` response and in arena results.
pub const ENGINE_NAME: &str = "mekchess-classical";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_is_wired_up() {
        assert_eq!(ENGINE_NAME, "mekchess-classical");
    }
}
