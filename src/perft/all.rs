use crate::{
    bitboard::BitBoard,
    castling_rights::CastlingRights,
    coord::Square,
    piece::{Piece, PieceType, Side},
    pieces::{
        bishop::bishop_moves,
        king::king_moves,
        knight::knight_moves,
        pawn::{PawnAttacks, PawnPushes},
        queen::queen_moves,
        rook::rook_moves,
    },
    position::Position,
    zobrist_table::ZOBRIST_TABLE,
};

impl Node {
    fn count_jumper_moves<F: Fn(Square) -> BitBoard>(&self, piece: Piece, movegen: F) -> usize {
        assert!(piece.piece_type().is_jumper()); // This _should_ always be optimized out, and
                                                 // provides an easy check if everything is inlined correctly
        self.piece(piece)
            .iter()
            .map(|(sq, _)| {
                movegen(sq)
                    .difference(self.occupancy(piece.side()))
                    .popcount() as usize
            })
            .sum()
    }

    fn pawn_pushes(&self, side: Side) -> PawnPushes {
        match side {
            Side::White => PawnPushes::white(
                self.piece(Piece::WhitePawn),
                self.occupancy_total.complement(),
            ),
            Side::Black => PawnPushes::black(
                self.piece(Piece::BlackPawn),
                self.occupancy_total.complement(),
            ),
        }
    }

    fn count_slider_moves<F: Fn(Square, BitBoard) -> BitBoard>(
        &self,
        piece: Piece,
        movegen: F,
    ) -> usize {
        assert!(piece.piece_type().is_slider()); // This _should_ always be optimized out and
                                                 // provides an easy check if everything is inlined correctly
        self.piece(piece)
            .iter()
            .map(|(sq, _)| {
                movegen(sq, self.occupancy_total)
                    .difference(self.occupancy(piece.side()))
                    .popcount() as usize
            })
            .sum()
    }

    pub fn perft(&self, depth: u8) -> usize {
        self.perft_white(depth)
    }

    pub fn perft_white(&self, depth: u8) -> usize {
        if depth > 0 {
            // pawn push single promote
            // pawn push single
            // pawn push double
            // pawn attack
            // pawn attack promote
            // knight
            // bishop
            // rook
            // queen
            // king
            // castle
            todo!()
        } else {
            if self.white_king_attacked() {
                0
            } else {
                1
            }
        }
    }

    pub fn perft_black(&self, depth: u8) -> usize {
        if depth > 0 {
            // pawn
            // knight
            // bishop
            // rook
            // queen
            // king
            // castle
            todo!()
        } else {
            if self.black_king_attacked() {
                0
            } else {
                1
            }
        }
    }

    pub fn count_black_moves(&self) -> usize {
        self.pawn_pushes(Side::Black).count_moves() as usize
            + self.pawn_attacks(Side::Black).count_moves() as usize
            + self.count_jumper_moves(Piece::BlackKnight, knight_moves)
            + self.count_jumper_moves(Piece::BlackKing, king_moves)
            + self.count_slider_moves(Piece::BlackBishop, bishop_moves)
            + self.count_slider_moves(Piece::BlackRook, rook_moves)
            + self.count_slider_moves(Piece::BlackQueen, queen_moves)
    }

    #[inline(never)]
    pub fn count_white_moves(&self) -> usize {
        self.pawn_pushes(Side::White).count_moves() as usize
            + self.pawn_attacks(Side::White).count_moves() as usize
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
        let n0 = Position::POSITION_1.to_bitboard();
        assert_eq!(n0.perft(0), 1);
        assert_eq!(n0.perft(1), 20);
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
        let n0 = Position::POSITION_2.to_bitboard();
        // Castling
        assert_eq!(n0.count_white_moves(), 48);
        // 2039 97862 4085603 193690690 8031647685
    }

    #[test]
    fn position_3() {
        let n0 = Position::POSITION_3.to_bitboard();
        // Discovered checks
        assert_eq!(n0.count_white_moves(), 14);
        // 191 2812 43238 674624 11030083 178633661 3009794393
    }

    #[test]
    fn position_4() {
        let n0 = Position::POSITION_4.to_bitboard();
        // King in check
        assert_eq!(n0.count_white_moves(), 6);
        // 264 9467 422333 15833292 706045033
    }

    #[test]
    fn position_5() {
        let n0 = Position::POSITION_5.to_bitboard();
        // Castling
        assert_eq!(n0.count_white_moves(), 44);
        // 1486 62379 2103487 89941194
    }

    #[test]
    fn position_6() {
        let n0 = Position::POSITION_6.to_bitboard();
        assert_eq!(n0.count_white_moves(), 46);
        // 2079 89,890 3,894,594 164,075,551 6,923,051,137 287,188,994,746 11,923,589,843,526
        //      490,154,852,788,714
    }
}
