use crate::{bitboard::BitBoard, coord::Square};

pub fn king_moves(sq: Square) -> BitBoard {
    king_moves_cached_unsafe(sq)
}

// TODO castling

#[rustfmt::skip]
pub const fn king_moves_ref(sq: Square) -> BitBoard {
    let mut bb = BitBoard::EMPTY;

    if let Some(sq) = sq.offset(-1,-1) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset( 0,-1) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset( 1,-1) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset(-1, 0) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset( 1, 0) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset(-1, 1) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset( 0, 1) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset( 1, 1) { bb.set_assign(sq); }

    bb
}

static MOVES: [BitBoard; 64] = {
    let mut table = [BitBoard::EMPTY; 64];
    let mut i = 0;
    while i < 64 {
        let sq = Square::from_index(i as u8).unwrap();
        table[i] = king_moves_ref(sq);
        i += 1;
    }
    table
};

pub const fn king_moves_cached(sq: Square) -> BitBoard {
    MOVES[sq.to_index() as usize]
}

pub fn king_moves_cached_unsafe(sq: Square) -> BitBoard {
    unsafe { *MOVES.get_unchecked(sq.to_index() as usize) }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        bitboard::BitBoard,
        coord::Square,
        pieces::king::{king_moves, king_moves_cached, king_moves_cached_unsafe, king_moves_ref},
    };

    #[test]
    fn unit() {
        assert_eq!(
            king_moves(Square::from_str("a1").unwrap()),
            BitBoard::from_bits(0x302)
        );
        assert_eq!(
            king_moves(Square::from_str("e4").unwrap()),
            BitBoard::from_bits(0x3828380000)
        );
    }

    #[quickcheck]
    fn king_square_count(sq: Square) -> bool {
        let top_bottom = BitBoard::R1.union(BitBoard::R8);
        let left_right = BitBoard::FA.union(BitBoard::FH);
        let n = king_moves_ref(sq).len();
        match top_bottom.contains(sq) as u8 + left_right.contains(sq) as u8 {
            0 => n == 8,
            1 => n == 5,
            2 => n == 3,
            _ => unreachable!(),
        }
    }

    #[quickcheck]
    fn safe_is_ref(sq: Square) -> bool {
        king_moves_cached(sq) == king_moves_ref(sq)
    }

    #[quickcheck]
    fn unsafe_is_ref(sq: Square) -> bool {
        king_moves_cached_unsafe(sq) == king_moves_ref(sq)
    }
}
