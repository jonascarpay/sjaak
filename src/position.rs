use std::char;

use crate::{
    castling_rights::{CastlingRights, CastlingSide},
    coord::{File, Rank, Square},
    piece::{Piece, PieceType, Side},
    print_board::format_board_fancy,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pieces: [Option<Piece>; 64],
    side: Side,
    castling_rights: CastlingRights,
    en_passant_square: Option<Square>,
    halfmove_clock: u8,
    move_clock: usize,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board_str =
            format_board_fancy(|sq| self.get_piece(sq).map_or(' ', |piece| piece.to_unicode()))
                .unwrap();
        writeln!(f, "{}", board_str)
    }
}

// TODO validate FEN after
#[derive(Debug, PartialEq, Eq)]
pub enum PieceCountError {
    NotOneKing,
    MoreThanEightPawns,
    MoreThanSixteenPieces,
}

struct ConstParser<'a> {
    index: usize,
    bytes: &'a [u8],
}

impl<'a> ConstParser<'a> {
    pub const fn new(str: &'a str) -> Self {
        ConstParser {
            index: 0,
            bytes: str.as_bytes(),
        }
    }

    pub const fn pop_number(&mut self) -> usize {
        let mut total = 0;
        loop {
            if self.finished() {
                break;
            }
            let b = self.peek();
            if b >= b'0' && b <= b'9' {
                self.pop();
                total = total * 10 + (b - b'0') as usize;
            } else {
                break;
            }
        }
        total
    }

    pub const fn finished(&self) -> bool {
        self.index >= self.bytes.len()
    }

    pub const fn expect_space(&mut self) {
        if self.pop() != b' ' {
            panic!("Expected space");
        }
    }

    pub const fn peek(&mut self) -> u8 {
        self.bytes[self.index]
    }

    pub const fn pop(&mut self) -> u8 {
        let b = self.bytes[self.index];
        self.index += 1;
        b
    }
}

impl Position {
    pub const START_POS: Self = Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"); // TODO why doesn't this error
    pub const POSITION_1: Self =
        Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    pub const POSITION_2: Self =
        Position::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
    pub const POSITION_3: Self = Position::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1");
    pub const POSITION_4: Self =
        Position::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1");
    pub const POSITION_5: Self =
        Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    pub const POSITION_6: Self = Position::from_fen(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
    );

    // Currently only used at compile time, so we just panic on invalid strings.
    // Slightly more liberal than the actual spec, castling rights can be in any order, repeated,
    // and contain any amount of dashes.
    pub const fn from_fen(fen: &str) -> Position {
        let mut fen = ConstParser::new(fen);

        let pieces = {
            let mut pieces = [None; 64];
            let mut rank: i8 = 7;
            let mut file: i8 = 0;

            while !(rank == 0 && file == 8) {
                let b = fen.pop();
                let c = char::from_u32(b as u32).unwrap();
                if c == '/' {
                    if file != 8 {
                        panic!("Incomplete/overfull file");
                    }
                    rank -= 1;
                    file = 0;
                } else if let Some(digit) = c.to_digit(10) {
                    file += digit as i8;
                } else if let Some(piece) = Piece::from_fen_char(c) {
                    let index = (rank * 8 + file) as usize;
                    pieces[index] = Some(piece);
                    file += 1;
                } else {
                    panic!("Invalid character");
                }
            }
            pieces
        };
        fen.expect_space();

        let side = match fen.pop() {
            b'w' => Side::White,
            b'b' => Side::Black,
            _ => panic!("Unrecognized side char"),
        };

        fen.expect_space();

        let castling_rights = {
            let mut rights = CastlingRights::new_empty();
            loop {
                match fen.pop() {
                    b' ' => break,
                    b'-' => {}
                    b'K' => rights.restore(Side::White, CastlingSide::KingSide),
                    b'Q' => rights.restore(Side::White, CastlingSide::QueenSide),
                    b'k' => rights.restore(Side::Black, CastlingSide::KingSide),
                    b'q' => rights.restore(Side::Black, CastlingSide::QueenSide),
                    _ => panic!("Unrecognized side char"),
                }
            }
            rights
        };

        // No expect_space needed, castling rights already consumes the space

        let en_passant_square = {
            match fen.pop() {
                b'-' => None,
                b => {
                    let file = File::from_ascii(b).unwrap();
                    let rank = Rank::from_ascii(fen.pop()).unwrap();
                    Some(Square::from_coord(file, rank))
                }
            }
        };

        fen.expect_space();
        let halfmove_clock = fen.pop_number() as u8;
        fen.expect_space();
        let move_clock = fen.pop_number() - 1;

        Position {
            pieces,
            side,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            move_clock,
        }
    }

    pub fn to_fen(&self) -> String {
        let mut fen = String::with_capacity(80);
        for rank_idx in (0..8).rev() {
            let mut empty_count = 0;
            for file_idx in 0..8 {
                let index = rank_idx * 8 + file_idx;
                if let Some(piece) = self.pieces[index] {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece.to_fen_char());
                } else {
                    empty_count += 1;
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            if rank_idx > 0 {
                fen.push('/');
            }
        }
        fen
    }

    pub const fn get_piece(&self, sq: Square) -> Option<Piece> {
        self.pieces[sq.to_index() as usize]
    }

    pub fn check_piece_count(&self) -> Result<(), (Side, PieceCountError)> {
        let mut piece_counts: [u8; 12] = [0; 12];
        for sq in Square::iter_all() {
            if let Some(piece) = self.get_piece(sq) {
                piece_counts[piece as usize] += 1;
            }
        }

        for side in [Side::White, Side::Black] {
            let get = |pt: PieceType| piece_counts[Piece::from_side_piece(side, pt) as usize];
            let err = |val| Err((side, val));
            use PieceType::*;
            if get(King) != 1 {
                return err(PieceCountError::NotOneKing);
            }
            if get(Pawn) > 8 {
                return err(PieceCountError::MoreThanEightPawns);
            }
            if get(Pawn) + get(Knight) + get(Bishop) + get(Rook) + get(Queen) + get(King) > 16 {
                return err(PieceCountError::MoreThanSixteenPieces);
            }
        }
        Ok(())
    }

    pub const fn castling_rights(&self) -> &CastlingRights {
        &self.castling_rights
    }

    pub const fn en_passant_square(&self) -> &Option<Square> {
        &self.en_passant_square
    }
}
