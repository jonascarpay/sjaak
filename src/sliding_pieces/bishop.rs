use crate::{bitboard::BitBoard, coord::Square};

pub fn bishop_moves(sq: Square) -> BitBoard {
    // This functions defines which among the all implementations we designate as the "preferred"
    // one.
    bishop_moves_iter(sq)
}

#[rustfmt::skip]
pub const fn bishop_moves_iter(sq: Square) -> BitBoard {
    let mut bb = BitBoard::new();
    let mut i = 1;
    while i < 8 {
        if let Some(sq) = sq.northeast_by(i) { bb.set_assign(sq); }
        if let Some(sq) = sq.southeast_by(i) { bb.set_assign(sq); }
        if let Some(sq) = sq.northwest_by(i) { bb.set_assign(sq); }
        if let Some(sq) = sq.southwest_by(i) { bb.set_assign(sq); }
        i += 1;
    }
    bb
}

// TODO apparently from_fn is const-able in newer versions, check out at some point
pub const BISHOP_MOVE_LUT: [BitBoard; 64] = {
    let mut table: [BitBoard; 64] = [BitBoard::EMPTY; 64];
    let mut i = 0;
    while i < 64 {
        table[i] = bishop_moves_iter(Square::from_index(i as u8).unwrap());
        i += 1;
    }
    table
};

pub const fn bishop_moves_lookup(sq: Square) -> BitBoard {
    BISHOP_MOVE_LUT[sq.to_index() as usize]
}

#[cfg(test)]
pub mod tests {
    use quickcheck_macros::quickcheck;

    use crate::coord::Square;

    use super::bishop_moves;

    #[quickcheck]
    fn bishop_moves_iter(sq: Square) -> bool {
        super::bishop_moves_iter(sq) == bishop_moves(sq)
    }

    #[quickcheck]
    fn bishop_moves_lookup(sq: Square) -> bool {
        super::bishop_moves_lookup(sq) == bishop_moves(sq)
    }

    #[quickcheck]
    fn has_at_least_7_squares(sq: Square) -> bool {
        bishop_moves(sq).len() >= 7
    }

    #[quickcheck]
    fn has_at_most_13_squares(sq: Square) -> bool {
        bishop_moves(sq).len() <= 13
    }

    #[quickcheck]
    fn preserves_color(src: Square) -> bool {
        bishop_moves(src).all(|tgt| tgt.is_light() == src.is_light())
    }

    #[quickcheck]
    fn is_bidirectional(src: Square) -> bool {
        bishop_moves(src).all(|tgt| bishop_moves(tgt).any(|src_| src == src_))
    }
}
