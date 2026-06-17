//! NNUE chess engine: alpha-beta search with a neural-network evaluation.
//!
//! Epic E. The network is trained offline in Python (`training/`, nnue-pytorch)
//! and exported as a quantized weights file; inference here is hand-written
//! integer arithmetic over an incrementally-updated accumulator (no ML
//! framework at runtime). Shares the search scaffolding with the classical
//! engine — only the evaluation function differs.

/// Identifies this engine to the UCI `id name` response and in arena results.
pub const ENGINE_NAME: &str = "mekchess-nnue";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_is_wired_up() {
        assert_eq!(ENGINE_NAME, "mekchess-nnue");
    }
}
