use crate::piece::Side;

// TODO check if just having 4 bools is faster.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CastlingRights {
    bitset: u8,
}

pub enum CastlingSide {
    KingSide = 0,
    QueenSide = 1,
}

impl CastlingRights {
    pub const CARDINALITY: u8 = 16; // TODO rename? rename others?
    pub const fn new_full() -> Self {
        CastlingRights { bitset: 0b1111 }
    }
    pub const fn new_empty() -> Self {
        CastlingRights { bitset: 0b0000 }
    }

    const fn mask(side: Side, castling_side: CastlingSide) -> u8 {
        1 << (side as u8 * 2 + castling_side as u8)
    }

    pub const fn can_castle(&self, side: Side, castling_side: CastlingSide) -> bool {
        self.bitset & Self::mask(side, castling_side) == 0
    }

    pub const fn revoke(&mut self, side: Side, castling_side: CastlingSide) {
        self.bitset &= !Self::mask(side, castling_side)
    }

    pub const fn restore(&mut self, side: Side, castling_side: CastlingSide) {
        self.bitset |= Self::mask(side, castling_side)
    }

    /// 0-16
    /// For Zobrist purposes
    pub const fn to_index(&self) -> u8 {
        self.bitset
    }
}
