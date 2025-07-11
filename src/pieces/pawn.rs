use crate::{
    bitboard::BitBoard,
    coord::Rank,
};

#[derive(Debug, PartialEq, Eq)]
pub enum PawnError {
    PawnOnEdgeRank,
}

const PAWNS_FORBIDDEN: BitBoard = BitBoard::R1.union(BitBoard::R8);

pub fn validate_pawn_bitboard(pawns: BitBoard) -> Result<(), PawnError> {
    if !pawns.intersect(PAWNS_FORBIDDEN).is_empty() {
        Err(PawnError::PawnOnEdgeRank)
    } else {
        Ok(())
    }
}

pub const fn white_pawn_attacks(white_pawns: BitBoard) -> BitBoard {
    // debug_assert_eq!(validate_pawn_bitboard(white_pawns), Ok(()));
    let northeast_attackers = white_pawns.intersect(BitBoard::FA.complement());
    let northwest_attackers = white_pawns.intersect(BitBoard::FH.complement());
    BitBoard::from_bits(northwest_attackers.to_bits() << 9 | northeast_attackers.to_bits() << 7)
}

pub const fn black_pawn_attacks(black_pawns: BitBoard) -> BitBoard {
    // debug_assert_eq!(validate_pawn_bitboard(black_pawns), Ok(()));
    let southeast_attackers = black_pawns.intersect(BitBoard::FA.complement());
    let southwest_attackers = black_pawns.intersect(BitBoard::FH.complement());
    BitBoard::from_bits(southwest_attackers.to_bits() >> 7 | southeast_attackers.to_bits() >> 9)
}

pub const fn white_pawn_pushes(white_pawns: BitBoard, blockers: BitBoard) -> (BitBoard, BitBoard) {
    let empty_square = blockers.complement();
    let single_push = white_pawns.lshift(8).intersect(empty_square);
    let double_push = single_push
        .lshift(8)
        .intersect(empty_square)
        .intersect(Rank::R4.to_bitboard());
    (single_push, double_push)
}

pub const fn black_pawn_pushes(black_pawns: BitBoard, blockers: BitBoard) -> (BitBoard, BitBoard) {
    let empty_square = blockers.complement();
    let single_push = black_pawns.rshift(8).intersect(empty_square);
    let double_push = single_push
        .rshift(8)
        .intersect(empty_square)
        .intersect(Rank::R5.to_bitboard());
    (single_push, double_push)
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        bitboard::BitBoard,
        coord::Square,
        pieces::pawn::{
            black_pawn_attacks, white_pawn_attacks, black_pawn_pushes, white_pawn_pushes,
        },
    };

    use super::PAWNS_FORBIDDEN;

    const PAWNS_ALLOWED: BitBoard = PAWNS_FORBIDDEN.complement();

    #[test]
    fn white_pawn_attack() {
        let pawns = Square::from_str("d4").unwrap().to_bitboard();
        assert_eq!(
            white_pawn_attacks(pawns),
            Square::from_str("c5").unwrap().to_bitboard()
                .union(Square::from_str("e5").unwrap().to_bitboard())
        );
    }

    #[test]
    fn white_pawn_single_push() {
        let pawns = Square::from_str("d4").unwrap().to_bitboard();
        let (single, double) = white_pawn_pushes(pawns, BitBoard::EMPTY);
        assert_eq!(
            single,
            Square::from_str("d5").unwrap().to_bitboard()
        );
        assert_eq!(double, BitBoard::EMPTY);
    }

    #[test]
    fn white_pawn_double_push() {
        let pawns = Square::from_str("d2").unwrap().to_bitboard();
        let (single, double) = white_pawn_pushes(pawns, BitBoard::EMPTY);
        assert_eq!(
            single,
            Square::from_str("d3").unwrap().to_bitboard()
        );
        assert_eq!(
            double,
            Square::from_str("d4").unwrap().to_bitboard()
        );
    }

    #[test]
    fn black_pawn_attack() {
        let pawns = Square::from_str("d4").unwrap().to_bitboard();
        assert_eq!(
            black_pawn_attacks(pawns),
            Square::from_str("c3").unwrap().to_bitboard()
                .union(Square::from_str("e3").unwrap().to_bitboard())
        );
    }

    #[test]
    fn black_pawn_single_push() {
        let pawns = Square::from_str("d4").unwrap().to_bitboard();
        let (single, double) = black_pawn_pushes(pawns, BitBoard::EMPTY);
        assert_eq!(
            single,
            Square::from_str("d3").unwrap().to_bitboard()
        );
        assert_eq!(double, BitBoard::EMPTY);
    }

    #[test]
    fn black_pawn_double_push() {
        let pawns = Square::from_str("d7").unwrap().to_bitboard();
        let (single, double) = black_pawn_pushes(pawns, BitBoard::EMPTY);
        assert_eq!(
            single,
            Square::from_str("d6").unwrap().to_bitboard()
        );
        assert_eq!(
            double,
            Square::from_str("d5").unwrap().to_bitboard()
        );
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
