use crate::{
    bitboard::BitBoard,
    coord::{Rank, Square},
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

pub const fn piecewise_white_pawn_attack(sq: Square) -> BitBoard {
    white_pawn_attacks(sq.to_bitboard())
}

pub const fn piecewise_black_pawn_attack(sq: Square) -> BitBoard {
    black_pawn_attacks(sq.to_bitboard())
}

pub const fn white_pawn_pushes(white_pawns: BitBoard, blockers: BitBoard) -> BitBoard {
    let empty_square = blockers.complement();
    let single_push = white_pawns.lshift(8).intersect(empty_square);
    let double_push = single_push
        .lshift(8)
        .intersect(empty_square)
        .intersect(Rank::R4.to_bitboard());
    single_push.union(double_push)
}

pub const fn black_pawn_pushes(black_pawns: BitBoard, blockers: BitBoard) -> BitBoard {
    let empty_square = blockers.complement();
    let single_push = black_pawns.rshift(8).intersect(empty_square);
    let double_push = single_push
        .rshift(8)
        .intersect(empty_square)
        .intersect(Rank::R5.to_bitboard());
    single_push.union(double_push)
}

pub const fn piecewise_white_pawn_push(sq: Square, blockers: BitBoard) -> BitBoard {
    white_pawn_pushes(sq.to_bitboard(), blockers)
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        bitboard::BitBoard,
        coord::Square,
        pieces::pawn::{
            black_pawn_attacks, black_pawn_pushes, piecewise_white_pawn_attack,
            piecewise_white_pawn_push, white_pawn_attacks, white_pawn_pushes,
        },
    };

    use super::PAWNS_FORBIDDEN;

    const PAWNS_ALLOWED: BitBoard = PAWNS_FORBIDDEN.complement();

    #[test]
    fn unit() {
        assert_eq!(
            piecewise_white_pawn_attack(Square::from_str("d4").unwrap()),
            BitBoard::from_bits(0x1400000000)
        );
        assert_eq!(
            piecewise_white_pawn_push(Square::from_str("d4").unwrap(), BitBoard::EMPTY),
            Square::from_str("d5").unwrap().to_bitboard()
        );
        assert_eq!(
            piecewise_white_pawn_push(Square::from_str("d2").unwrap(), BitBoard::EMPTY),
            Square::from_str("d3")
                .unwrap()
                .to_bitboard()
                .union(Square::from_str("d4").unwrap().to_bitboard())
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
        white_pawn_pushes(pawns, blockers)
            == black_pawn_pushes(pawns.reverse(), blockers.reverse()).reverse()
    }

    #[quickcheck]
    fn pushes_are_symmetric_via_vertical_flip(pawns: BitBoard, blockers: BitBoard) -> bool {
        let pawns = pawns.intersect(PAWNS_ALLOWED);
        white_pawn_pushes(pawns, blockers)
            == black_pawn_pushes(pawns.vflip(), blockers.vflip()).vflip()
    }

    #[quickcheck]
    fn pushes_commute_with_horizontal_flip(pawns: BitBoard, blockers: BitBoard) -> bool {
        let pawns = pawns.intersect(PAWNS_ALLOWED);
        white_pawn_pushes(pawns, blockers)
            == white_pawn_pushes(pawns.hflip(), blockers.hflip()).hflip()
    }
}
