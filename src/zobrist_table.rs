use crate::{coord::Square, piece::PieceIndex};

const ZOBRIST_TABLE_SIZE: usize = 12 * 64;
pub static ZOBRIST_TABLE: [u64; ZOBRIST_TABLE_SIZE] = {
    let mut rng = Lcg::new(999999);
    let mut table = [0; ZOBRIST_TABLE_SIZE];
    let mut i = 0;
    while i < ZOBRIST_TABLE_SIZE {
        table[i] = rng.next();
        i += 1;
    }
    table
};

pub const fn zobrist_index(sq: Square, piece: PieceIndex) -> usize {
    piece.to_index() as usize * 64 + sq.to_index() as usize
}

// TODO probably remove
pub const fn get_key_const(sq: Square, piece: PieceIndex) -> u64 {
    ZOBRIST_TABLE[zobrist_index(sq, piece)]
}

pub fn get_key(sq: Square, piece: PieceIndex) -> u64 {
    let index = zobrist_index(sq, piece);
    debug_assert!(index < ZOBRIST_TABLE_SIZE);
    unsafe { *ZOBRIST_TABLE.get_unchecked(index) }
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
