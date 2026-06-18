//! FEN: the position serialization format (an external "API" for positions).
//!
//! [Forsyth-Edwards Notation](https://www.chessprogramming.org/Forsyth-Edwards_Notation)
//! is a text representation of a chess position — six space-separated fields:
//! `<placement> <side> <castling> <en-passant> <halfmove> <fullmove>`, e.g.
//! `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`.
//!
//! It is deliberately *not* how a position is stored internally (see
//! [`crate::board::Board`]); this module is the boundary that converts between
//! the two. Everything about reading and writing FEN text lives here.
//!
//! ## Rust concept: `Result` and error enums
//! [`Board::from_fen`] returns `Result<Board, FenError>`, not `Option`. `Option`
//! says "value or nothing"; `Result` says "value or *here's what went wrong*".
//! [`FenError`] enumerates the ways a FEN can be malformed, so a caller learns
//! *why* parsing failed. The `?` operator works on `Result` just like on
//! `Option` — unwrap `Ok`, or early-return the `Err`.

use crate::board::{Board, CastlingRights};
use crate::piece::{Color, Piece, PieceType};
use crate::square::Square;

impl Piece {
    /// Parse a piece from its FEN character.
    ///
    /// Returns `Some(piece)` for a valid FEN piece letter, `None` otherwise.
    /// Uppercase = White, lowercase = Black. Letters:
    /// `p`=Pawn, `n`=Knight, `b`=Bishop, `r`=Rook, `q`=Queen, `k`=King.
    ///
    /// Examples: `'N'` → white knight, `'q'` → black queen, `'1'` / `'x'` → `None`.
    pub fn from_fen(c: char) -> Option<Piece> {
        match c {
            'P' => Some(Piece {
                color: Color::White,
                kind: PieceType::Pawn,
            }),
            'p' => Some(Piece {
                color: Color::Black,
                kind: PieceType::Pawn,
            }),
            'N' => Some(Piece {
                color: Color::White,
                kind: PieceType::Knight,
            }),
            'n' => Some(Piece {
                color: Color::Black,
                kind: PieceType::Knight,
            }),
            'B' => Some(Piece {
                color: Color::White,
                kind: PieceType::Bishop,
            }),
            'b' => Some(Piece {
                color: Color::Black,
                kind: PieceType::Bishop,
            }),
            'R' => Some(Piece {
                color: Color::White,
                kind: PieceType::Rook,
            }),
            'r' => Some(Piece {
                color: Color::Black,
                kind: PieceType::Rook,
            }),
            'Q' => Some(Piece {
                color: Color::White,
                kind: PieceType::Queen,
            }),
            'q' => Some(Piece {
                color: Color::Black,
                kind: PieceType::Queen,
            }),
            'K' => Some(Piece {
                color: Color::White,
                kind: PieceType::King,
            }),
            'k' => Some(Piece {
                color: Color::Black,
                kind: PieceType::King,
            }),
            _ => None,
        }
    }

    /// The FEN character for this piece (the inverse of [`from_fen_char`]).
    ///
    /// White pieces are uppercase, Black pieces lowercase.
    ///
    /// [`from_fen_char`]: Piece::from_fen_char
    pub fn to_fen(&self) -> char {
        let mut c = match self.kind {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        };

        if self.color == Color::White {
            c = c.to_ascii_uppercase();
        }

        c
    }
}

/// Parse the **piece-placement field** of a FEN string — the part before the
/// first space, e.g. `"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"` — into a
/// 64-entry mailbox where index 0 = a1 and index 63 = h8 (LERF). Each entry is
/// `Some(piece)` or `None` for an empty square.
///
/// Returns `None` if any character is neither a digit `1`..`8` nor a valid FEN
/// piece letter.
///
/// ## The key subtlety: the rank flip
/// FEN lists **rank 8 first**, but we store **a1 at index 0**. So the *first*
/// `/`-separated substring is rank 8 and must land at rank index **7**
/// (squares 56–63); the *last* substring is rank 1 → rank index **0**
/// (squares 0–7). Within a rank, characters read left→right are files a→h
/// (file 0→7). Get this flip right and White ends up at the low indices, Black
/// at the high ones.
pub fn placement_from_fen(placement: &str) -> Option<[Option<Piece>; 64]> {
    let mut squares = [None; 64];
    for (i, rank_str) in placement.split('/').enumerate() {
        // Each loop iteration is one rank from 0-7
        let rank: u8 = 7 - i as u8;
        let mut file: u8 = 0;
        for c in rank_str.chars() {
            if let Some(n) = c.to_digit(10) {
                file += n as u8;
            } else {
                let piece = Piece::from_fen(c)?;
                let index = Square::from_file_rank(file, rank).index();
                squares[index as usize] = Some(piece);
                file += 1;
            }
        }
    }
    Some(squares)
}

/// Serialize a 64-square mailbox back into the FEN piece-placement field — the
/// inverse of [`parse_placement`]. Ranks are written high-to-low (rank 8
/// first), consecutive empty squares are run-length-encoded as a digit, and the
/// eight ranks are joined by `/`.
pub fn placement_to_fen(squares: &[Option<Piece>; 64]) -> String {
    let mut ranks: Vec<String> = Vec::new();
    for rank in 0u8..8 {
        // forward: rank 1, 2, ... 8
        let mut s = String::new();
        let mut empty_counter = 0u8;
        for file in 0u8..8 {
            // forward within the rank: a .. h (correct!)
            let index = Square::from_file_rank(file, rank).index() as usize;
            match squares[index] {
                Some(piece) => {
                    if empty_counter > 0 {
                        s.push((b'0' + empty_counter) as char);
                        empty_counter = 0;
                    }
                    s.push(piece.to_fen());
                }
                None => empty_counter += 1,
            }
        }
        if empty_counter > 0 {
            s.push((b'0' + empty_counter) as char);
        }
        ranks.push(s);
    }
    ranks.reverse(); // FEN writes rank 8 first — flip ONLY the rank order
    ranks.join("/") // join handles the separators
}

/// Why a FEN string failed to parse. Returned by [`Board::from_fen`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FenError {
    /// A FEN has six space-separated fields; this many were found.
    WrongFieldCount(usize),
    BadPlacement,
    BadSideToMove,
    BadCastling,
    BadEnPassant,
    BadHalfmove,
    BadFullmove,
}

impl CastlingRights {
    /// Parse the FEN castling field: `"-"` for none, otherwise some subset of
    /// the letters `K` (white kingside), `Q` (white queenside), `k`, `q`.
    /// Returns `None` if the field contains any other character.
    pub fn from_fen(fen_cstl: &str) -> Option<CastlingRights> {
        if fen_cstl == "-" {
            return Some(CastlingRights::default());
        }
        if fen_cstl.is_empty() {
            return None;
        }

        let mut rights = CastlingRights::default();
        for c in fen_cstl.chars() {
            match c {
                'K' => rights.white_kingside = true,
                'Q' => rights.white_queenside = true,
                'k' => rights.black_kingside = true,
                'q' => rights.black_queenside = true,
                _ => return None,
            }
        }
        Some(rights) // success
    }

    /// Serialize to the FEN castling field: letters in `K, Q, k, q` order, or
    /// `"-"` if no side can castle.
    pub fn to_fen(&self) -> String {
        let mut fen_cstl = String::new();
        if self.white_kingside {
            fen_cstl += "K";
        }
        if self.white_queenside {
            fen_cstl += "Q";
        }
        if self.black_kingside {
            fen_cstl += "k";
        }
        if self.black_queenside {
            fen_cstl += "q";
        }
        if fen_cstl.is_empty() {
            "-".to_string()
        } else {
            fen_cstl
        }
    }
}

impl Board {
    /// Parse a full FEN string into a [`Board`], or a [`FenError`] describing the
    /// first problem found.
    pub fn from_fen(fen_full: &str) -> Result<Board, FenError> {
        let fields: Vec<&str> = fen_full.split_whitespace().collect();
        if fields.len() != 6 {
            return Err(FenError::WrongFieldCount(fields.len()));
        }
        //   - field 0: parse_placement(..).ok_or(FenError::BadPlacement)?
        let squares = placement_from_fen(fields[0]).ok_or(FenError::BadPlacement)?;

        //   - field 1: "w" -> White, "b" -> Black, else BadSideToMove
        let side_to_move = match fields[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError::BadSideToMove),
        };

        //   - field 2: CastlingRights::from_fen(..).ok_or(FenError::BadCastling)?
        let castling = CastlingRights::from_fen(fields[2]).ok_or(FenError::BadCastling)?;

        //   - field 3: "-" -> None, else Some(Square::from_algebraic(..)
        //              .ok_or(FenError::BadEnPassant)?)
        let en_passant = if fields[3] == "-" {
            None
        } else {
            Some(Square::from_algebraic(fields[3]).ok_or(FenError::BadEnPassant)?)
        };

        //   - field 4: .parse::<u32>().map_err(|_| FenError::BadHalfmove)?
        let halfmove = fields[4]
            .parse::<u32>()
            .map_err(|_| FenError::BadHalfmove)?;

        //   - field 5: .parse::<u32>().map_err(|_| FenError::BadFullmove)?
        let fullmove = fields[5]
            .parse::<u32>()
            .map_err(|_| FenError::BadFullmove)?;

        Ok(Board {
            squares,
            side_to_move,
            castling,
            en_passant,
            halfmove,
            fullmove,
        })
    }

    /// Serialize this position to a FEN string (the inverse of [`from_fen`]).
    ///
    /// [`from_fen`]: Board::from_fen
    pub fn to_fen(&self) -> String {
        let fen_placement = placement_to_fen(&self.squares);
        let fen_to_move = if self.side_to_move == Color::White {
            "w"
        } else {
            "b"
        };

        let fen_castle = CastlingRights::to_fen(&self.castling);

        let fen_passant = match self.en_passant {
            Some(sq) => sq.to_algebraic(),
            None => "-".to_string(),
        };

        format!(
            "{} {} {} {} {} {}",
            fen_placement, fen_to_move, fen_castle, fen_passant, self.halfmove, self.fullmove
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{Color, PieceType};

    /// Tiny helper so the assertions below read cleanly.
    fn pc(color: Color, kind: PieceType) -> Option<Piece> {
        Some(Piece { color, kind })
    }

    /// The six canonical perft positions (chessprogramming.org). Each is a
    /// fully-normalized 6-field FEN, so parse -> serialize must reproduce it
    /// byte-for-byte. These are the same positions ticket #5 will perft-test.
    const CANONICAL: [&str; 6] = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    ];

    #[test]
    fn startpos_places_pieces_with_correct_rank_flip() {
        let sq = placement_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
            .expect("valid placement");

        // White back rank at the LOW indices (rank 1 = squares 0..=7).
        assert_eq!(sq[0], pc(Color::White, PieceType::Rook)); // a1
        assert_eq!(sq[3], pc(Color::White, PieceType::Queen)); // d1
        assert_eq!(sq[4], pc(Color::White, PieceType::King)); // e1
        // White pawns on rank 2 (squares 8..=15).
        assert_eq!(sq[8], pc(Color::White, PieceType::Pawn)); // a2
        assert_eq!(sq[15], pc(Color::White, PieceType::Pawn)); // h2
        // Black back rank at the HIGH indices (rank 8 = squares 56..=63).
        assert_eq!(sq[56], pc(Color::Black, PieceType::Rook)); // a8
        assert_eq!(sq[60], pc(Color::Black, PieceType::King)); // e8
        // Black pawns on rank 7 (squares 48..=55).
        assert_eq!(sq[48], pc(Color::Black, PieceType::Pawn)); // a7

        // The middle four ranks (squares 16..48) are empty.
        for (i, square) in sq.iter().enumerate().skip(16).take(32) {
            assert_eq!(*square, None, "index {i} should be empty");
        }
    }

    #[test]
    fn digits_expand_to_empty_squares() {
        // rank 1 = "4K3": 4 empties (a–d), white king on e1, 3 empties (f–h).
        let sq = placement_from_fen("8/8/8/8/8/8/8/4K3").expect("valid placement");
        assert_eq!(sq[4], pc(Color::White, PieceType::King)); // e1
        for (i, square) in sq.iter().enumerate() {
            if i != 4 {
                assert_eq!(*square, None, "index {i} should be empty");
            }
        }
    }

    #[test]
    fn rejects_invalid_piece_char() {
        // 'X' is neither a digit nor a valid FEN piece letter.
        assert_eq!(placement_from_fen("8/8/8/8/8/8/8/4X3"), None);
    }

    #[test]
    fn placement_round_trips() {
        for field in [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
            "8/8/8/8/8/8/8/4K3",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R",
        ] {
            let squares = placement_from_fen(field).expect("valid placement");
            assert_eq!(placement_to_fen(&squares), field, "round trip for {field}");
        }
    }

    #[test]
    fn castling_round_trips() {
        // All inputs are already in canonical K,Q,k,q order.
        for field in ["KQkq", "KQ", "kq", "Kq", "-"] {
            let rights = CastlingRights::from_fen(field).expect("valid castling field");
            assert_eq!(rights.to_fen(), field, "round trip for {field}");
        }
    }

    #[test]
    fn castling_dash_means_none() {
        assert_eq!(
            CastlingRights::from_fen("-"),
            Some(CastlingRights::default())
        );
    }

    #[test]
    fn castling_rejects_garbage() {
        assert_eq!(CastlingRights::from_fen("Xy"), None);
    }

    #[test]
    fn parses_startpos_fields() {
        let b = Board::from_fen(CANONICAL[0]).expect("startpos is valid");
        assert_eq!(b.side_to_move, Color::White);
        assert_eq!(
            b.castling,
            CastlingRights {
                white_kingside: true,
                white_queenside: true,
                black_kingside: true,
                black_queenside: true,
            }
        );
        assert_eq!(b.en_passant, None);
        assert_eq!(b.halfmove, 0);
        assert_eq!(b.fullmove, 1);
    }

    #[test]
    fn full_fen_round_trips() {
        for fen in CANONICAL {
            let board = Board::from_fen(fen).expect("canonical FEN is valid");
            assert_eq!(board.to_fen(), fen, "round trip for {fen}");
        }
    }

    #[test]
    fn rejects_malformed_fen() {
        // 3 whitespace-separated tokens, not 6.
        assert_eq!(
            Board::from_fen("not a fen"),
            Err(FenError::WrongFieldCount(3))
        );
        // Valid layout but a bad side-to-move field.
        assert!(matches!(
            Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR x KQkq - 0 1"),
            Err(FenError::BadSideToMove)
        ));
    }

    #[test]
    fn parses_white_knight() {
        assert_eq!(
            Piece::from_fen('N'),
            Some(Piece {
                color: Color::White,
                kind: PieceType::Knight
            }),
        );
    }

    #[test]
    fn parses_black_queen() {
        assert_eq!(
            Piece::from_fen('q'),
            Some(Piece {
                color: Color::Black,
                kind: PieceType::Queen
            }),
        );
    }

    #[test]
    fn rejects_non_piece_chars() {
        assert_eq!(Piece::from_fen('1'), None);
        assert_eq!(Piece::from_fen('x'), None);
        assert_eq!(Piece::from_fen(' '), None);
    }

    #[test]
    fn serializes_pieces_to_fen_chars() {
        let white_king = Piece {
            color: Color::White,
            kind: PieceType::King,
        };
        let black_pawn = Piece {
            color: Color::Black,
            kind: PieceType::Pawn,
        };
        assert_eq!(white_king.to_fen(), 'K');
        assert_eq!(black_pawn.to_fen(), 'p');
    }

    /// Every valid FEN piece char should survive a parse → serialize round trip.
    #[test]
    fn char_round_trip_for_all_pieces() {
        for c in ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'] {
            let piece = Piece::from_fen(c).expect("valid piece char should parse");
            assert_eq!(piece.to_fen(), c, "round trip failed for {c}");
        }
    }
}
