use crate::bitboard::BitBoard;

#[derive(Clone, Copy, Debug)]
pub struct MagicValue(u64);

// TODO
// Better, and consistent, naming for the square hierarchy
// - potential blocker squares
// - actual blockers
// - relevant blockers

impl MagicValue {
    pub const fn new(value: u64) -> Self {
        MagicValue(value)
    }

    pub fn random<Rng: FnMut() -> u64>(random_u64: &mut Rng, max_bits: u8) -> Self {
        let mut res = random_u64();
        while res.count_ones() > (max_bits as u32) {
            res &= random_u64();
        }
        MagicValue(res)
    }

    #[inline(always)]
    pub const fn to_index(self, potential_blockers: BitBoard, index_bits: u8) -> usize {
        let hash = potential_blockers.to_bits().wrapping_mul(self.0);
        (hash >> (64 - index_bits)) as usize
    }

    pub const fn to_u64(self) -> u64 {
        self.0
    }
}
