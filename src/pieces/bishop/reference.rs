use crate::{bitboard::BitBoard, coord::Square};

pub const fn bishop_moves_reference(sq: Square, blockers: BitBoard) -> BitBoard {
    let mut bb = BitBoard::new();

    let mut northeast = sq.northeast();
    while let Some(sq) = northeast {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        northeast = sq.northeast();
    }

    let mut northwest = sq.northwest();
    while let Some(sq) = northwest {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        northwest = sq.northwest();
    }

    let mut southeast = sq.southeast();
    while let Some(sq) = southeast {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        southeast = sq.southeast();
    }

    let mut southwest = sq.southwest();
    while let Some(sq) = southwest {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        southwest = sq.southwest();
    }

    bb
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        bitboard::BitBoard,
        coord::Square,
        pieces::bishop::{bishop_moves, reference::bishop_moves_reference},
    };

    #[quickcheck]
    fn rim_blockers_dont_matter(sq: Square, blockers: BitBoard) -> bool {
        bishop_moves_reference(sq, blockers)
            == bishop_moves_reference(sq, blockers.intersect(BitBoard::RIM.complement()))
    }

    #[quickcheck]
    fn commutes_hflip(sq: Square, blockers: BitBoard) -> bool {
        bishop_moves(sq.hflip(), blockers.hflip()) == bishop_moves(sq, blockers).hflip()
    }

    #[quickcheck]
    fn commutes_vflip(sq: Square, blockers: BitBoard) -> bool {
        bishop_moves(sq.vflip(), blockers.vflip()) == bishop_moves(sq, blockers).vflip()
    }

    #[quickcheck]
    fn commutes_reverse(sq: Square, blockers: BitBoard) -> bool {
        bishop_moves(sq.reverse(), blockers.reverse()) == bishop_moves(sq, blockers).reverse()
    }
}
