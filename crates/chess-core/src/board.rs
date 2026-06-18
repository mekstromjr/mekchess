//! The board: the in-memory representation of a chess position.
//!
//! This module owns the *data* — how a position is stored internally (a 64-entry
//! mailbox plus the side to move, castling rights, en passant target, and move
//! clocks). Converting a `Board` to or from the external FEN text format is a
//! separate concern and lives in [`crate::fen`].

use crate::piece::{Color, Piece};
use crate::square::Square;

/// Castling availability for both sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CastlingRights {
    /// White may castle kingside (the h1 rook).
    pub white_kingside: bool,
    /// White may castle queenside (the a1 rook).
    pub white_queenside: bool,
    /// Black may castle kingside (the h8 rook).
    pub black_kingside: bool,
    /// Black may castle queenside (the a8 rook).
    pub black_queenside: bool,
}

/// A complete chess position.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// Mailbox: index 0 = a1 .. 63 = h8 (LERF). `None` is an empty square.
    pub squares: [Option<Piece>; 64],
    /// The side whose turn it is to move.
    pub side_to_move: Color,
    /// Which castles remain legal for each side.
    pub castling: CastlingRights,
    /// The en passant target square, if any.
    pub en_passant: Option<Square>,
    /// Halfmove clock: plies since the last capture or pawn move (50-move rule).
    pub halfmove: u32,
    /// Fullmove number; starts at 1 and increments after Black moves.
    pub fullmove: u32,
}
