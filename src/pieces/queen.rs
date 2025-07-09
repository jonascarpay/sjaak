use crate::{bitboard::BitBoard, coord::Square};

use super::{bishop::bishop_moves, rook::rook_moves};

pub fn queen_moves(sq: Square, blockers: BitBoard) -> BitBoard {
    rook_moves(sq, blockers).union(bishop_moves(sq, blockers))
}
