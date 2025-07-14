use crate::{
    bitboard::BitBoard,
    castling_rights::CastlingRights,
    coord::Square,
    piece::{Piece, Side},
    pieces::{
        bishop::bishop_moves,
        king::king_moves,
        knight::knight_moves,
        pawn::{PawnAttacks, PawnPushes},
        queen::queen_moves,
        rook::rook_moves,
    },
    position::Position,
};

pub struct PositionBB {
    pieces: [BitBoard; 12],
    occupancy_white: BitBoard,
    occupancy_black: BitBoard,
    occupancy_total: BitBoard,
    castling_rights: CastlingRights,
    // TODO For now, A1 == 0 means no en passant possible. A little ugly, but if it turns out to
    // work nicely I'll wrap it up in a better API.
    en_passant_square: Square,
}

impl Position {
    pub const fn to_bitboard(&self) -> PositionBB {
        PositionBB::from_position(self)
    }
}

impl PositionBB {
    pub const fn piece(&self, piece: Piece) -> BitBoard {
        self.pieces[piece as usize]
    }
    pub const fn occupancy(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.occupancy_white,
            Side::Black => self.occupancy_black,
        }
    }
    pub const fn from_position(pos: &Position) -> PositionBB {
        let mut pieces = [BitBoard::EMPTY; 12];
        let mut occupancy_white = BitBoard::EMPTY;
        let mut occupancy_black = BitBoard::EMPTY;
        let mut i = 0;
        while i < 64 {
            let sq = Square::from_index(i).unwrap();
            if let Some(piece) = pos.get_piece(sq) {
                pieces[piece as usize].set_assign(sq);
                match piece.side() {
                    Side::White => occupancy_white.set_assign(sq),
                    Side::Black => occupancy_black.set_assign(sq),
                }
            }
            i += 1;
        }
        PositionBB {
            pieces,
            occupancy_white,
            occupancy_black,
            occupancy_total: occupancy_black.union(occupancy_white),
            castling_rights: *pos.castling_rights(),
            en_passant_square: match pos.en_passant_square() {
                Some(sq) => *sq,
                None => Square::from_index(0).unwrap(),
            },
        }
    }

    fn count_jumper_moves<F: Fn(Square) -> BitBoard>(&self, piece: Piece, movegen: F) -> usize {
        assert!(piece.piece_type().is_jumper()); // This _should_ always be optimized out, and
                                                 // provides an easy check if everything is inlined correctly
        self.piece(piece)
            .map(|sq| movegen(sq).difference(self.occupancy(piece.side())).len())
            .sum()
    }

    fn count_slider_moves<F: Fn(Square, BitBoard) -> BitBoard>(
        &self,
        piece: Piece,
        movegen: F,
    ) -> usize {
        assert!(piece.piece_type().is_slider()); // This _should_ always be optimized out and
                                                 // provides an easy check if everything is inlined correctly
        self.piece(piece)
            .map(|sq| {
                movegen(sq, self.occupancy_total)
                    .difference(self.occupancy(piece.side()))
                    .len()
            })
            .sum()
    }

    pub fn count_black_moves(&self) -> usize {
        let pushes = PawnPushes::black(
            self.piece(Piece::BlackPawn),
            self.occupancy_total.complement(),
        )
        .count_moves() as usize;

        let attacks = PawnAttacks::black(self.piece(Piece::BlackPawn), self.occupancy_white)
            .count_moves() as usize;

        pushes
            + attacks
            + self.count_jumper_moves(Piece::BlackKnight, knight_moves)
            + self.count_jumper_moves(Piece::BlackKing, king_moves)
            + self.count_slider_moves(Piece::BlackBishop, bishop_moves)
            + self.count_slider_moves(Piece::BlackRook, rook_moves)
            + self.count_slider_moves(Piece::BlackQueen, queen_moves)
    }

    pub fn count_white_moves(&self) -> usize {
        let pushes = PawnPushes::white(
            self.piece(Piece::WhitePawn),
            self.occupancy_total.complement(),
        )
        .count_moves() as usize;

        let attacks = PawnAttacks::white(self.piece(Piece::WhitePawn), self.occupancy_black)
            .count_moves() as usize;

        pushes
            + attacks
            + self.count_jumper_moves(Piece::WhiteKnight, knight_moves)
            + self.count_jumper_moves(Piece::WhiteKing, king_moves)
            + self.count_slider_moves(Piece::WhiteBishop, bishop_moves)
            + self.count_slider_moves(Piece::WhiteRook, rook_moves)
            + self.count_slider_moves(Piece::WhiteQueen, queen_moves)
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn position_1() {
        let pos = Position::POSITION_1.to_bitboard();
        assert_eq!(pos.count_white_moves(), 20);
        //               400
        //             8_902
        //           197_281
        //         4_865_609
        //       119_060_324
        //     3_195_901_860
        //    84_998_978_956
        // 2_439_530_234_167
    }

    #[test]
    fn position_2() {
        let pos = Position::POSITION_2.to_bitboard();
        // Castling
        assert_eq!(pos.count_white_moves(), 48);
        // 2039 97862 4085603 193690690 8031647685
    }

    #[test]
    fn position_3() {
        let pos = Position::POSITION_3.to_bitboard();
        // Discovered checks
        assert_eq!(pos.count_white_moves(), 14);
        // 191 2812 43238 674624 11030083 178633661 3009794393
    }

    #[test]
    fn position_4() {
        let pos = Position::POSITION_4.to_bitboard();
        // King in check
        assert_eq!(pos.count_white_moves(), 6);
        // 264 9467 422333 15833292 706045033
    }

    #[test]
    fn position_5() {
        let pos = Position::POSITION_5.to_bitboard();
        // Castling
        assert_eq!(pos.count_white_moves(), 44);
        // 1486 62379 2103487 89941194
    }

    #[test]
    fn position_6() {
        let pos = Position::POSITION_6.to_bitboard();
        assert_eq!(pos.count_white_moves(), 46);
        // 2079 89,890 3,894,594 164,075,551 6,923,051,137 287,188,994,746 11,923,589,843,526
        //      490,154,852,788,714
    }
}
