use crate::{bitboard::BitBoard, coord::Square};

pub fn knight_moves(sq: Square) -> BitBoard {
    knight_moves_cached_unsafe(sq)
}

#[rustfmt::skip]
pub const fn knight_moves_ref(sq: Square) -> BitBoard {
    let mut bb = BitBoard::EMPTY;

    if let Some(sq) = sq.offset( 1, 2) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset( 1,-2) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset(-1, 2) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset(-1,-2) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset( 2, 1) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset( 2,-1) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset(-2, 1) { bb.set_assign(sq); }
    if let Some(sq) = sq.offset(-2,-1) { bb.set_assign(sq); }

    bb
}

static MOVES: [BitBoard; 64] = {
    let mut table = [BitBoard::EMPTY; 64];
    let mut i = 0;
    while i < 64 {
        let sq = Square::from_index(i as u8).unwrap();
        table[i] = knight_moves_ref(sq);
        i += 1;
    }
    table
};

pub const fn knight_moves_cached(sq: Square) -> BitBoard {
    MOVES[sq.to_index() as usize]
}

pub fn knight_moves_cached_unsafe(sq: Square) -> BitBoard {
    unsafe { *MOVES.get_unchecked(sq.to_index() as usize) }
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        bitboard::BitBoard,
        coord::Square,
        pieces::knight::{
            knight_moves, knight_moves_cached, knight_moves_cached_unsafe, knight_moves_ref,
        },
    };

    #[test]
    fn unit() {
        assert_eq!(
            knight_moves(Square::from_str("a1").unwrap()),
            BitBoard::from_bits(0x20400)
        );
        assert_eq!(
            knight_moves(Square::from_str("e4").unwrap()),
            BitBoard::from_bits(0x284400442800)
        );
    }

    #[quickcheck]
    fn always_at_least_2(sq: Square) -> bool {
        knight_moves(sq).len() >= 2
    }

    #[quickcheck]
    fn safe_is_ref(sq: Square) -> bool {
        knight_moves_cached(sq) == knight_moves_ref(sq)
    }

    #[quickcheck]
    fn unsafe_is_ref(sq: Square) -> bool {
        knight_moves_cached_unsafe(sq) == knight_moves_ref(sq)
    }
}
