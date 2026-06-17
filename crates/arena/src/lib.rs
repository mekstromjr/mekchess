//! In-process bot-battle arena (Epic D).
//!
//! For quick local iteration this runs engines as Rust libraries over the shared
//! [`chess_core`] board — no UCI text round-trip — and records W/D/L. For
//! statistically rigorous strength testing (SPRT, Elo, pentanomial scoring) and
//! large/distributed runs, we delegate to fast-chess driving the engines' UCI
//! binaries rather than reimplementing that here; the homelab-hosted version of
//! that workflow is Epic F (OpenBench).

/// Outcome of a single game from the first engine's perspective.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    Win,
    Draw,
    Loss,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_is_wired_up() {
        assert_ne!(GameResult::Win, GameResult::Loss);
    }
}
