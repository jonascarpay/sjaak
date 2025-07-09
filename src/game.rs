use crate::{
    bitboard::BitBoard,
    coord::Square,
    piece::{PieceIndex, Side},
};

pub struct Position {
    pieces: [BitBoard; 12],
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

    pub fn get_board(&mut self, piece: PieceIndex) -> &mut BitBoard {
        piece.debug_assert_valid();
        unsafe { self.pieces.get_unchecked_mut(piece.to_index() as usize) }
    }
}
