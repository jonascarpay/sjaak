use crate::{
    coord::{File, Rank, Square},
    print_board::format_board_fancy,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct BitBoard {
    bits: u64,
}

impl std::fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "0x{:x}", self.bits)?;
        writeln!(
            f,
            "{}",
            format_board_fancy(|sq| if self.contains(sq) { 'x' } else { ' ' }).unwrap()
        )
    }
}

impl BitBoard {
    pub const fn new() -> Self {
        Self::EMPTY
    }
    pub const fn contains(self, sq: Square) -> bool {
        (self.bits >> sq.to_index()) & 1 == 1
    }

    pub const fn set(self, sq: Square) -> Self {
        Self {
            bits: self.bits | sq.to_bitboard().bits,
        }
    }
    pub const fn unset(self, sq: Square) -> Self {
        Self {
            bits: self.bits & !sq.to_bitboard().bits,
        }
    }
    pub const fn set_to(self, sq: Square, val: bool) -> Self {
        if val {
            self.set(sq)
        } else {
            self.unset(sq)
        }
    }

    pub const fn set_assign(&mut self, sq: Square) {
        self.bits |= sq.to_bitboard().bits;
    }
    pub const fn unset_assign(&mut self, sq: Square) {
        self.bits &= !sq.to_bitboard().bits;
    }
    pub const fn assign(&mut self, sq: Square, val: bool) {
        if val {
            self.set_assign(sq);
        } else {
            self.unset_assign(sq);
        }
    }
    pub const fn is_empty(&self) -> bool {
        self.to_bits() == 0
    }
    pub const fn is_nonempty(&self) -> bool {
        self.to_bits() != 0
    }
    pub const fn intersects(&self, rhs: BitBoard) -> bool {
        self.intersect(rhs).is_nonempty()
    }

    pub fn from_squares<I: Iterator<Item = Square>>(iter: I) -> Self {
        iter.fold(Self::EMPTY, |acc, el| acc.union(el.to_bitboard()))
    }
    pub const fn to_bits(self) -> u64 {
        self.bits
    }
    pub const fn from_bits(bits: u64) -> Self {
        BitBoard { bits }
    }

    pub const fn get_square(self) -> Option<Square> {
        Square::from_index(self.bits.trailing_zeros() as u8)
    }

    pub const fn popcount(self) -> u32 {
        self.bits.count_ones()
    }

    // Set theory //
    pub const fn union(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            bits: self.bits | rhs.bits,
        }
    }
    pub const fn intersect(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            bits: self.bits & rhs.bits,
        }
    }
    pub const fn complement(self) -> BitBoard {
        BitBoard { bits: !self.bits }
    }
    pub const fn difference(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            bits: self.bits & !rhs.bits,
        }
    }
    pub const fn symmetric_difference(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            bits: self.bits ^ rhs.bits,
        }
    }
    pub const fn lshift(self, sh: i8) -> BitBoard {
        BitBoard::from_bits(self.to_bits() << sh)
    }
    pub const fn rshift(self, sh: i8) -> BitBoard {
        BitBoard::from_bits(self.to_bits() >> sh)
    }

    pub const fn reverse(self) -> BitBoard {
        BitBoard {
            bits: self.bits.reverse_bits(),
        }
    }
    pub const fn vflip(self) -> BitBoard {
        BitBoard::from_bits(self.to_bits().swap_bytes())
    }
    pub const fn hflip(self) -> BitBoard {
        const K1: u64 = 0x5555555555555555; // ..01010101
        const K2: u64 = 0x3333333333333333; // ..00110011
        const K4: u64 = 0x0f0f0f0f0f0f0f0f; // ..00001111

        let bits = self.to_bits();
        let swap1 = ((bits >> 1) & K1) | ((bits & K1) << 1);
        let swap2 = ((swap1 >> 2) & K2) | ((swap1 & K2) << 2);
        let swap4 = ((swap2 >> 4) & K4) | ((swap2 & K4) << 4);

        BitBoard::from_bits(swap4)
    }

    pub const EMPTY: BitBoard = BitBoard { bits: 0 };
    pub const R1: BitBoard = Rank::R1.to_bitboard();
    pub const R2: BitBoard = Rank::R2.to_bitboard();
    pub const R7: BitBoard = Rank::R7.to_bitboard();
    pub const R8: BitBoard = Rank::R8.to_bitboard();
    pub const FA: BitBoard = File::FA.to_bitboard();
    pub const FH: BitBoard = File::FH.to_bitboard();
    pub const RIM: BitBoard = Self::R1.union(Self::R8).union(Self::FA).union(Self::FH);
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        let sq = Square::from_index(self.bits.trailing_zeros() as u8)?;
        self.bits &= self.bits - 1;
        Some(sq)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.popcount() as usize;
        (len, Some(len))
    }
}

impl ExactSizeIterator for BitBoard {}
impl std::iter::FusedIterator for BitBoard {}

impl Default for BitBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl std::ops::Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard { bits: !self.bits }
    }
}

impl Square {
    pub const fn to_bitboard(self) -> BitBoard {
        BitBoard {
            bits: 1 << self.to_index(),
        }
    }
}

impl Rank {
    pub const fn to_bitboard(self) -> BitBoard {
        BitBoard::from_bits(0xFF << (self.to_index() << 3))
    }
}

impl File {
    pub const fn to_bitboard(self) -> BitBoard {
        BitBoard::from_bits(0x0101_0101_0101_0101 << self.to_index())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bitboard::BitBoard,
        coord::{File, Rank, Square},
    };
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;
    #[cfg(test)]
    impl Arbitrary for BitBoard {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            BitBoard {
                bits: Arbitrary::arbitrary(g),
            }
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.bits.shrink().map(|bits| BitBoard { bits }))
        }
    }

    #[quickcheck]
    fn to_bitboard_iterator_roundtrip(sq: Square) -> bool {
        let v: Vec<Square> = sq.to_bitboard().collect();
        v == vec![sq]
    }

    #[quickcheck]
    fn to_bitboard_is_from_squares(sq: Square) -> bool {
        sq.to_bitboard() == BitBoard::from_squares([sq].into_iter())
    }

    #[quickcheck]
    fn len_is_count(bb: BitBoard) -> bool {
        let len = bb.len();
        let mut count = 0;
        for _ in bb {
            count += 1;
        }
        len == count
    }

    #[quickcheck]
    fn set_get(bb: BitBoard, sq: Square, val: bool) -> bool {
        bb.set_to(sq, val).contains(sq) == val
    }

    #[quickcheck]
    fn get_set(bb: BitBoard, sq: Square) -> bool {
        bb == bb.set_to(sq, bb.contains(sq))
    }

    #[quickcheck]
    fn from_squares_is_id(bb: BitBoard) -> bool {
        BitBoard::from_squares(bb) == bb
    }

    #[test]
    fn rim_bb() {
        let mut bb = BitBoard::new();
        for rank in Rank::ALL {
            bb.set_assign(Square::from_coord(File::FA, rank));
            bb.set_assign(Square::from_coord(File::FH, rank));
        }
        for file in File::ALL {
            bb.set_assign(Square::from_coord(file, Rank::R1));
            bb.set_assign(Square::from_coord(file, Rank::R8));
        }
        assert_eq!(bb, BitBoard::RIM)
    }

    #[quickcheck]
    fn square_to_bb_is_rank_file_bb_intersect(sq: Square) -> bool {
        let (rank, file) = sq.to_coord();
        sq.to_bitboard() == rank.to_bitboard().intersect(file.to_bitboard())
    }

    #[quickcheck]
    fn hflip_involution(bb: BitBoard) -> bool {
        bb.hflip().hflip() == bb
    }

    #[quickcheck]
    fn vflip_involution(bb: BitBoard) -> bool {
        bb.vflip().vflip() == bb
    }

    #[quickcheck]
    fn reverse_involution(bb: BitBoard) -> bool {
        bb.reverse().reverse() == bb
    }

    #[quickcheck]
    fn flips_commute(bb: BitBoard) -> bool {
        bb.vflip().hflip() == bb.hflip().vflip()
    }

    #[quickcheck]
    fn flips_are_reverse(bb: BitBoard) -> bool {
        bb.vflip().hflip() == bb.reverse()
    }

    #[quickcheck]
    fn sq_hflip_tobitboard_commutes(sq: Square) -> bool {
        sq.hflip().to_bitboard() == sq.to_bitboard().hflip()
    }

    #[quickcheck]
    fn sq_vflip_tobitboard_commutes(sq: Square) -> bool {
        sq.vflip().to_bitboard() == sq.to_bitboard().vflip()
    }

    #[quickcheck]
    fn sq_reverse_tobitboard_commutes(sq: Square) -> bool {
        sq.reverse().to_bitboard() == sq.to_bitboard().reverse()
    }

    #[quickcheck]
    fn difference_plus_intersections_is_id(a: BitBoard, b: BitBoard) -> bool {
        let diff = a.difference(b);
        let int = a.intersect(b);
        diff.union(int) == a
    }

    #[quickcheck]
    fn difference_minus_intersections_is_empty(a: BitBoard, b: BitBoard) -> bool {
        let diff = a.difference(b);
        let int = a.intersect(b);
        diff.intersect(int).is_empty()
    }
}
