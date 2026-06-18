//! Pieces: the data types for a chess piece.
//!
//! A chess piece is a [`Color`] paired with a [`PieceType`]. These are pure
//! data; converting a piece to or from its FEN character is a FEN-format
//! concern (used only by FEN) and lives in [`crate::fen`].
//!
//! ## Rust concepts in play here
//! - `enum` as a *sum type*: `Color` and `PieceType` each enumerate a fixed set
//!   of variants; a `match` over them is checked for exhaustiveness.
//! - `#[derive(...)]`: auto-implements common traits — `Debug` (`{:?}`,
//!   used by `assert_eq!` failure messages), `PartialEq`/`Eq` (`==`), and
//!   `Clone`/`Copy` (these types are tiny, so copying is cheap and avoids
//!   ownership-move friction).

/// Which side a piece belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

/// The six kinds of chess piece (no color attached).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// A piece on the board: a color together with a kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceType,
}
