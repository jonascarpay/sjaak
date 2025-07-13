use crate::{bitboard::BitBoard, coord::Rank};

pub struct PawnPushes {
    pub single: BitBoard,
    pub double: BitBoard,
    pub promotion: BitBoard,
}

impl PawnPushes {
    pub const fn white(white_pawns: BitBoard, empty_squares: BitBoard) -> Self {
        const BACKRANK: BitBoard = BitBoard::R8;
        let single_push = white_pawns.lshift(8).intersect(empty_squares);
        let double_push = single_push
            .lshift(8)
            .intersect(empty_squares)
            .intersect(Rank::R4.to_bitboard());
        PawnPushes {
            single: single_push.difference(BACKRANK),
            double: double_push,
            promotion: single_push.intersect(BACKRANK),
        }
    }
    pub const fn black(black_pawns: BitBoard, empty_squares: BitBoard) -> Self {
        const BACKRANK: BitBoard = BitBoard::R1;
        let single_push = black_pawns.rshift(8).intersect(empty_squares);
        let double_push = single_push
            .rshift(8)
            .intersect(empty_squares)
            .intersect(Rank::R5.to_bitboard());
        PawnPushes {
            single: single_push.difference(BACKRANK),
            double: double_push,
            promotion: single_push.intersect(BACKRANK),
        }
    }
    pub const fn count_moves(&self) -> u32 {
        self.single.popcount() + self.double.popcount() + self.promotion.popcount() * 4
    }
}

// TODO en passant
pub struct PawnAttacks {
    pub east_attackers: BitBoard,
    pub west_attackers: BitBoard,
    pub east_promoters: BitBoard,
    pub west_promoters: BitBoard,
}

impl PawnAttacks {
    pub fn white(white_pawns: BitBoard, black: BitBoard) -> PawnAttacks {
        const BACKRANK: BitBoard = BitBoard::R8;
        let east = white_pawns
            .difference(BitBoard::FH)
            .lshift(9)
            .intersect(black);
        let west = white_pawns
            .difference(BitBoard::FA)
            .lshift(7)
            .intersect(black);
        PawnAttacks {
            east_attackers: east.difference(BACKRANK),
            west_attackers: west.difference(BACKRANK),
            east_promoters: east.intersect(BACKRANK),
            west_promoters: west.intersect(BACKRANK),
        }
    }
    pub fn black(black_pawns: BitBoard, white: BitBoard) -> PawnAttacks {
        const BACKRANK: BitBoard = BitBoard::R1;
        let east = black_pawns
            .difference(BitBoard::FH)
            .rshift(9)
            .intersect(white);
        let west = black_pawns
            .difference(BitBoard::FA)
            .rshift(7)
            .intersect(white);
        PawnAttacks {
            east_attackers: east.difference(BACKRANK),
            west_attackers: west.difference(BACKRANK),
            east_promoters: east.intersect(BACKRANK),
            west_promoters: west.intersect(BACKRANK),
        }
    }
    pub const fn count_moves(&self) -> u32 {
        self.east_attackers.popcount()
            + self.west_attackers.popcount()
            + self.east_promoters.popcount() * 4
            + self.west_promoters.popcount() * 4
    }
}

#[cfg(test)]
#[cfg(never)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{bitboard::BitBoard, coord::Square};

    #[test]
    fn white_pawn_attack() {
        let pawns = Square::from_str("d4").unwrap().to_bitboard();
        assert_eq!(
            white_pawn_attacks(pawns),
            Square::from_str("c5")
                .unwrap()
                .to_bitboard()
                .union(Square::from_str("e5").unwrap().to_bitboard())
        );
    }

    #[test]
    fn white_pawn_single_push() {
        let pawns = Square::from_str("d4").unwrap().to_bitboard();
        let (single, double) = white_pawn_pushes(pawns, BitBoard::EMPTY);
        assert_eq!(single, Square::from_str("d5").unwrap().to_bitboard());
        assert_eq!(double, BitBoard::EMPTY);
    }

    #[test]
    fn white_pawn_double_push() {
        let pawns = Square::from_str("d2").unwrap().to_bitboard();
        let (single, double) = white_pawn_pushes(pawns, BitBoard::EMPTY);
        assert_eq!(single, Square::from_str("d3").unwrap().to_bitboard());
        assert_eq!(double, Square::from_str("d4").unwrap().to_bitboard());
    }

    #[test]
    fn black_pawn_attack() {
        let pawns = Square::from_str("d4").unwrap().to_bitboard();
        assert_eq!(
            black_pawn_attacks(pawns),
            Square::from_str("c3")
                .unwrap()
                .to_bitboard()
                .union(Square::from_str("e3").unwrap().to_bitboard())
        );
    }

    #[test]
    fn black_pawn_single_push() {
        let pawns = Square::from_str("d4").unwrap().to_bitboard();
        let (single, double) = black_pawn_pushes(pawns, BitBoard::EMPTY);
        assert_eq!(single, Square::from_str("d3").unwrap().to_bitboard());
        assert_eq!(double, BitBoard::EMPTY);
    }

    #[test]
    fn black_pawn_double_push() {
        let pawns = Square::from_str("d7").unwrap().to_bitboard();
        let (single, double) = black_pawn_pushes(pawns, BitBoard::EMPTY);
        assert_eq!(single, Square::from_str("d6").unwrap().to_bitboard());
        assert_eq!(double, Square::from_str("d5").unwrap().to_bitboard());
    }

    #[quickcheck]
    fn attacks_are_symmetric_via_reverse(bb: BitBoard) -> bool {
        let bb = bb.intersect(PAWNS_ALLOWED);
        white_pawn_attacks(bb) == black_pawn_attacks(bb.reverse()).reverse()
    }

    #[quickcheck]
    fn attacks_are_symmetric_via_vertical_flip(bb: BitBoard) -> bool {
        let bb = bb.intersect(PAWNS_ALLOWED);
        white_pawn_attacks(bb) == black_pawn_attacks(bb.vflip()).vflip()
    }

    #[quickcheck]
    fn attacks_commute_with_horizontal_flip(bb: BitBoard) -> bool {
        let bb = bb.intersect(PAWNS_ALLOWED);
        white_pawn_attacks(bb) == white_pawn_attacks(bb.hflip()).hflip()
    }

    #[quickcheck]
    fn pushes_are_symmetric_via_reverse(pawns: BitBoard, blockers: BitBoard) -> bool {
        let pawns = pawns.intersect(PAWNS_ALLOWED);
        let (w_single, w_double) = white_pawn_pushes(pawns, blockers);
        let (b_single, b_double) = black_pawn_pushes(pawns.reverse(), blockers.reverse());
        w_single == b_single.reverse() && w_double == b_double.reverse()
    }

    #[quickcheck]
    fn pushes_are_symmetric_via_vertical_flip(pawns: BitBoard, blockers: BitBoard) -> bool {
        let pawns = pawns.intersect(PAWNS_ALLOWED);
        let (w_single, w_double) = white_pawn_pushes(pawns, blockers);
        let (b_single, b_double) = black_pawn_pushes(pawns.vflip(), blockers.vflip());
        w_single == b_single.vflip() && w_double == b_double.vflip()
    }

    #[quickcheck]
    fn pushes_commute_with_horizontal_flip(pawns: BitBoard, blockers: BitBoard) -> bool {
        let pawns = pawns.intersect(PAWNS_ALLOWED);
        let (s1, d1) = white_pawn_pushes(pawns, blockers);
        let (s2, d2) = white_pawn_pushes(pawns.hflip(), blockers.hflip());
        s1 == s2.hflip() && d1 == d2.hflip()
    }
}
