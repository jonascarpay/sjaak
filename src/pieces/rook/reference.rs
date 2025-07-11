use crate::{bitboard::BitBoard, coord::Square};

pub const fn rook_moves_reference(sq: Square, blockers: BitBoard) -> BitBoard {
    let mut bb = BitBoard::new();

    let mut east = sq.east();
    while let Some(sq) = east {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        east = sq.east();
    }

    let mut north = sq.north();
    while let Some(sq) = north {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        north = sq.north();
    }

    let mut west = sq.west();
    while let Some(sq) = west {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        west = sq.west();
    }

    let mut south = sq.south();
    while let Some(sq) = south {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        south = sq.south();
    }

    bb
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{bitboard::BitBoard, coord::Square, pieces::rook::reference::rook_moves_reference};

    #[quickcheck]
    fn rook_moves_iter_no_mask_is_const(sq: Square) -> bool {
        let (rank, file) = sq.to_coord();
        let bb_ref = rank.to_bitboard().symmetric_difference(file.to_bitboard());
        let bb = rook_moves_reference(sq, BitBoard::EMPTY);
        bb == bb_ref
    }

    #[quickcheck]
    fn rim_blockers_dont_matter(sq: Square, blockers: BitBoard) -> bool {
        BitBoard::RIM.contains(sq)
            || rook_moves_reference(sq, BitBoard::RIM.intersect(blockers))
                == rook_moves_reference(sq, BitBoard::EMPTY)
    }

    #[quickcheck]
    fn commutes_hflip(sq: Square, blockers: BitBoard) -> bool {
        rook_moves_reference(sq.hflip(), blockers.hflip())
            == rook_moves_reference(sq, blockers).hflip()
    }

    #[quickcheck]
    fn commutes_vflip(sq: Square, blockers: BitBoard) -> bool {
        rook_moves_reference(sq.vflip(), blockers.vflip())
            == rook_moves_reference(sq, blockers).vflip()
    }

    #[quickcheck]
    fn commutes_reverse(sq: Square, blockers: BitBoard) -> bool {
        rook_moves_reference(sq.reverse(), blockers.reverse())
            == rook_moves_reference(sq, blockers).reverse()
    }
}
