use crate::{
    castling_rights::CastlingRights,
    coord::Square,
    piece::{Piece, Side},
};

// TODO Rename? Are these the keys? Is this the table?
pub struct ZobristTable {
    piece_table: [u64; Self::PIECE_TABLE_SIZE],
    en_passant_square_table: [u64; Self::EN_PASSANT_SQUARE_TABLE_SIZE], // TODO compress?
    castling_rights_table: [u64; Self::CASTLING_RIGHTS_TABLE_SIZE],
    side_table: [u64; Self::SIDE_TABLE_SIZE],
}

pub static ZOBRIST_TABLE: ZobristTable = ZobristTable::new(0x9999);

impl ZobristTable {
    const PIECE_TABLE_SIZE: usize = (Piece::NUM_PIECES as usize) * (Square::NUM_SQUARES as usize);
    const EN_PASSANT_SQUARE_TABLE_SIZE: usize = Square::NUM_SQUARES as usize;
    const CASTLING_RIGHTS_TABLE_SIZE: usize = CastlingRights::CARDINALITY as usize;
    const SIDE_TABLE_SIZE: usize = Side::CARDINALITY as usize;

    const fn new(seed: u128) -> Self {
        const fn mk_table<const SIZE: usize>(rng: &mut Lcg) -> [u64; SIZE] {
            let mut table = [0; SIZE];
            let mut i = 0;
            while i < SIZE {
                table[i] = rng.next();
                i += 1;
            }
            table
        }

        let mut rng = Lcg::new(seed);

        ZobristTable {
            piece_table: mk_table(&mut rng),
            en_passant_square_table: mk_table(&mut rng),
            castling_rights_table: mk_table(&mut rng),
            side_table: mk_table(&mut rng),
        }
    }

    pub fn hash_piece(&self, piece: Piece, square: Square) -> u64 {
        // TODO why does the ASM here contain a 0xFF mask, on aarch64 at least?
        let index = piece.to_index() as usize * 64 + square.to_index() as usize;
        unsafe { *self.piece_table.get_unchecked(index) }
    }
    pub fn hash_en_passant_square(&self, square: Square) -> u64 {
        // TODO why does the ASM here contain a 0xFF mask, on aarch64 at least?
        unsafe {
            *self
                .en_passant_square_table
                .get_unchecked(square.to_index() as usize)
        }
    }
    pub fn hash_castling_rights(&self, castling_rights: &CastlingRights) -> u64 {
        // TODO why does the ASM here contain an add, on aarch64 at least?
        unsafe {
            *self
                .castling_rights_table
                .get_unchecked(castling_rights.to_index() as usize)
        }
    }
    pub fn hash_side(&self, side: Side) -> u64 {
        // TODO why does the ASM here contain an add, on aarch64 at least?
        unsafe { *self.side_table.get_unchecked(side.to_index() as usize) }
    }
}

// A simple 128-bit LCG RNG.
// Should provide plenty of randomness for Zobrist purposes, and constructing the table at compile
// time.
struct Lcg {
    state: u128,
}

impl Lcg {
    const fn new(seed: u128) -> Self {
        Lcg { state: seed }
    }
    const fn next(&mut self) -> u64 {
        const MULT: u128 = 47026247687942121848144207491837418733;
        self.state = self.state.wrapping_mul(MULT).wrapping_add(1);
        (self.state >> 64) as u64
    }
}
