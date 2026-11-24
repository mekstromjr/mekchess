//! Board squares and the indexing convention.
//!
//! A [`Square`] is a board location 0..=63. We use **Little-Endian Rank-File
//! (LERF)** mapping, the most common engine convention:
//!
//! ```text
//!   index = rank * 8 + file
//!
//!   file: a=0, b=1, ... h=7   (the column)
//!   rank: 1=0, 2=1, ... 8=7   (the row, 0-based internally)
//!
//!   a1 = 0,  h1 = 7,  a8 = 56,  h8 = 63
//! ```
//!
//! Note: this is purely index <-> (file, rank) math. The FEN "rank 8 comes
//! first" reversal is NOT here — that lives in `fen::placement_from_fen`. Keep
//! this type honest and simple.
//!
//! ## Rust concept: the newtype pattern
//! `Square` wraps a single `u8` in a tuple struct: `struct Square(u8)`. Why not
//! just use `u8` directly?
//! - **Type safety**: a function taking `Square` can't be handed any random
//!   `u8` by mistake — the compiler stops you. (A `type Square = u8;` *alias*
//!   gives you a nicer name but zero safety; the newtype gives real safety.)
//! - **Invariant**: by making the inner field private and only building squares
//!   through constructors, you guarantee every `Square` is in 0..=63.
//! - **Methods**: you can hang `file()`, `rank()`, etc. off it.
//!
//! It compiles to the same machine code as a bare `u8` — zero runtime cost.
//!
//! Access the wrapped value with `self.0` (tuple-struct field syntax).

/// A board square, 0..=63, using a1=0 LERF indexing. The inner `u8` is private
/// so every `Square` is guaranteed valid.
///
// a0 => (0,0) is in the bottom left of the chess board.
// That means rank 1 is 0-7, rank 2 is 8-15, rank 3 is 16-23, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    /// Build a square from a raw 0..=63 index, or `None` if out of range.
    pub fn from_index(index: u8) -> Option<Square> {
        if index < 64 {
            Some(Square(index))
        } else {
            None
        }
    }

    /// Build a square from a `file` (0..=7) and `rank` (0..=7).
    ///
    /// Assume both are already in range (this is internal construction).
    pub fn from_file_rank(file: u8, rank: u8) -> Square {
        Square(rank * 8 + file)
    }

    /// The raw 0..=63 index.
    pub fn index(self) -> u8 {
        self.0
    }

    /// The file (column), 0..=7 (a=0 .. h=7).
    pub fn file(self) -> u8 {
        self.0 % 8
    }

    /// The rank (row), 0..=7 (rank 1 = 0 .. rank 8 = 7).
    pub fn rank(self) -> u8 {
        self.0 / 8
    }

    /// Checks that the string is a valid FEN square location
    pub fn is_valid_algebraic(s: &str) -> bool {
        // Check string length
        let bytes = s.as_bytes();
        if bytes.len() != 2 {
            return false;
        }

        // Check file is valid
        let file = bytes[0].to_ascii_lowercase();
        if !(b'a'..=b'h').contains(&file) {
            return false;
        }

        // Check rank is Valid
        let rank = bytes[1];
        if !(b'1'..=b'8').contains(&rank) {
            return false;
        }

        true
    }

    /// Parse algebraic coordinates like `"e4"` into a square, or `None` if the
    /// input isn't exactly a valid file letter (`a`..`h`) followed by a rank
    /// digit (`1`..`8`).
    pub fn from_algebraic(s: &str) -> Option<Square> {
        if Self::is_valid_algebraic(s) {
            let bytes = s.as_bytes();
            let file = bytes[0] - b'a';
            let rank = bytes[1] - b'1';
            Some(Self::from_file_rank(file, rank))
        } else {
            None
        }
    }

    /// The algebraic coordinates for this square, e.g. `"e4"`.
    pub fn to_algebraic(self) -> String {
        let file_char = (b'a' + self.file()) as char;
        let rank_char = (b'1' + self.rank()) as char;
        format!("{file_char}{rank_char}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corner_indices() {
        assert_eq!(Square::from_file_rank(0, 0).index(), 0); // a1
        assert_eq!(Square::from_file_rank(7, 0).index(), 7); // h1
        assert_eq!(Square::from_file_rank(0, 7).index(), 56); // a8
        assert_eq!(Square::from_file_rank(7, 7).index(), 63); // h8
    }

    #[test]
    fn e4_is_index_28() {
        // e = file 4, rank "4" = rank index 3 -> 3*8 + 4 = 28
        let e4 = Square::from_file_rank(4, 3);
        assert_eq!(e4.index(), 28);
        assert_eq!(e4.file(), 4);
        assert_eq!(e4.rank(), 3);
    }

    #[test]
    fn from_index_validates_range() {
        assert_eq!(Square::from_index(0).map(|s| s.index()), Some(0));
        assert_eq!(Square::from_index(63).map(|s| s.index()), Some(63));
        assert_eq!(Square::from_index(64), None);
        assert_eq!(Square::from_index(200), None);
    }

    #[test]
    fn parses_algebraic() {
        assert_eq!(
            Square::from_algebraic("a1"),
            Some(Square::from_file_rank(0, 0))
        );
        assert_eq!(
            Square::from_algebraic("e4"),
            Some(Square::from_file_rank(4, 3))
        );
        assert_eq!(
            Square::from_algebraic("h8"),
            Some(Square::from_file_rank(7, 7))
        );
    }

    #[test]
    fn rejects_bad_algebraic() {
        assert_eq!(Square::from_algebraic("e"), None); // too short
        assert_eq!(Square::from_algebraic("e44"), None); // too long
        assert_eq!(Square::from_algebraic("i4"), None); // bad file
        assert_eq!(Square::from_algebraic("e9"), None); // bad rank
        assert_eq!(Square::from_algebraic("44"), None); // bad file
    }

    #[test]
    fn algebraic_round_trip() {
        for index in 0u8..64 {
            let sq = Square::from_index(index).expect("0..64 is valid");
            let text = sq.to_algebraic();
            assert_eq!(
                Square::from_algebraic(&text),
                Some(sq),
                "round trip failed for {text}"
            );
        }
    }
}
