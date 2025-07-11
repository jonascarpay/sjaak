#[cfg(feature = "magic_bitboards")]
pub mod magic;
pub mod reference;

use crate::{bitboard::BitBoard, coord::Square};

#[cfg(not(feature = "magic_bitboards"))]
pub fn bishop_moves(sq: Square, blockers: BitBoard) -> BitBoard {
    reference::bishop_moves_reference(sq, blockers)
}

#[cfg(feature = "magic_bitboards")]
pub fn bishop_moves(sq: Square, blockers: BitBoard) -> BitBoard {
    magic::bishop_moves_magic_unsafe(sq, blockers)
}
