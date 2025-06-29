use crate::{
    bitboard::BitBoard,
    coord::{File, Rank, Square},
};

pub fn rook_moves(sq: Square) -> BitBoard {
    rook_moves_bitwise(sq)
}

pub const fn rook_moves_bitwise(sq: Square) -> BitBoard {
    let (rank, file) = sq.to_coord();
    rank.to_bitboard().symmetric_difference(file.to_bitboard())
}

pub fn rook_moves_iter(sq: Square) -> BitBoard {
    let (rank, file) = sq.to_coord();
    let mut bb = BitBoard::new();
    for file in File::ALL {
        bb.set_assign(Square::from_coord(file, rank));
    }
    for rank in Rank::ALL {
        bb.set_assign(Square::from_coord(file, rank));
    }
    bb.unset(sq)
}

// TODO apparently from_fn is const-able in newer versions, check out at some point
pub const ROOK_MOVE_LUT: [BitBoard; 64] = {
    let mut table: [BitBoard; 64] = [BitBoard::EMPTY; 64];
    let mut i = 0;
    while i < 64 {
        table[i] = rook_moves_bitwise(Square::from_index(i as u8).unwrap());
        i += 1;
    }
    table
};

pub const fn rook_moves_lookup(sq: Square) -> BitBoard {
    ROOK_MOVE_LUT[sq.to_index() as usize]
}

#[cfg(test)]
pub mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{coord::Square, sliding_pieces::rook::rook_moves};

    #[quickcheck]
    fn rook_moves_shift(sq: Square) -> bool {
        super::rook_moves_bitwise(sq) == rook_moves(sq)
    }

    #[quickcheck]
    fn rook_moves_iter(sq: Square) -> bool {
        super::rook_moves_iter(sq) == rook_moves(sq)
    }

    #[quickcheck]
    fn rook_moves_lookup(sq: Square) -> bool {
        super::rook_moves_lookup(sq) == rook_moves(sq)
    }

    #[quickcheck]
    fn has_14_squares(sq: Square) -> bool {
        rook_moves(sq).len() == 14
    }

    #[quickcheck]
    fn is_bidirectional(src: Square) -> bool {
        rook_moves(src).all(|tgt| rook_moves(tgt).any(|src_| src == src_))
    }
}
