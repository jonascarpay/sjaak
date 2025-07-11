use crate::{
    bitboard::BitBoard,
    coord::Square,
    piece::{Piece, Side},
};

pub struct Position {
    pieces: [BitBoard; 12],
    occupancy_white: BitBoard,
    occupancy_black: BitBoard,
    occupancy_both: BitBoard,
    to_play: Side,
    white_can_castle_kingside: bool,
    white_can_castle_queenside: bool,
    black_can_castle_kingside: bool,
    black_can_castle_queenside: bool,
    en_passant_quare: Option<Square>,
    halfmove_clock: u8,
    move_clock: usize,
}

impl Position {
    pub fn from_fen(str: &str) -> Option<Self> {
        todo!()
    }

    pub fn get_board(&self, piece: Piece) -> &BitBoard {
        unsafe { self.pieces.get_unchecked(piece.to_index() as usize) }
    }

    pub fn get_board_mut(&mut self, piece: Piece) -> &mut BitBoard {
        &mut self.pieces[piece.to_index() as usize]
    }

    #[inline(never)]
    pub fn add_piece(&mut self, piece: Piece, sq: Square) {
        self.get_board_mut(piece).set_assign(sq)
    }
}

// mov ecx, edx
// mov eax, 1
// shl rax, cl
// movzx ecx, sil
// or qword ptr [rdi + 8*rcx], rax
// ret
