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
    pub const fn bits_mut(&mut self) -> &mut u64 {
        &mut self.bits
    }

    pub const fn get_square(self) -> Option<Square> {
        Square::from_index(self.bits.trailing_zeros() as u8)
    }

    pub const fn popcount(self) -> u32 {
        self.bits.count_ones()
    }

    pub const fn pop(&mut self) -> Option<(Square, BitBoard)> {
        if self.bits != 0 {
            let sq = Square::from_index(self.bits.trailing_zeros() as u8).expect("Impossible");
            let prev = self.bits;
            self.bits &= prev - 1;
            Some((sq, BitBoard::from_bits(self.bits ^ prev)))
        } else {
            None
        }
    }

    pub const fn pop_bitboard(&mut self) -> Option<Square> {
        match self.pop() {
            None => None,
            Some((sq, _)) => Some(sq),
        }
    }

    pub const fn pop_square(&mut self) -> Option<BitBoard> {
        match self.pop() {
            None => None,
            Some((_, bb)) => Some(bb),
        }
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
    // TODO Maybe this is better called minus?
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
    pub const fn is_supserset_of(self, rhs: BitBoard) -> bool {
        rhs.difference(self).is_empty()
    }
    #[inline(never)]
    pub const fn is_strict_supserset_of(self, rhs: BitBoard) -> bool {
        rhs.difference(self).is_empty() && self.bits != rhs.bits
    }

    pub const fn apply(&mut self, rhs: BitBoard) {
        self.bits ^= rhs.bits
    }
    pub fn apply_move(&mut self, rhs: BitBoard) {
        debug_assert_eq!(rhs.popcount(), 2);
        debug_assert_eq!(self.intersect(rhs).popcount(), 1);
        self.apply(rhs)
    }
    pub const fn apply_mask(&mut self, mask: BitBoard) {
        self.bits &= mask.to_bits()
    }

    pub const fn lshift(self, sh: i8) -> BitBoard {
        BitBoard::from_bits(self.to_bits() << sh)
    }
    pub const fn rshift(self, sh: i8) -> BitBoard {
        BitBoard::from_bits(self.to_bits() >> sh)
    }

    #[inline(always)]
    const fn checked_shift(self, lbits: i8, forbidden: BitBoard) -> BitBoard {
        // TODO maybe this should be Option
        debug_assert!(!self.intersects(forbidden));
        let bits = if lbits > 0 {
            self.bits << lbits
        } else {
            self.bits >> lbits.abs()
        };
        let new = BitBoard::from_bits(bits);
        debug_assert!(self.popcount() == new.popcount());
        new
    }
    pub const fn north(self) -> BitBoard {
        self.checked_shift(8, BitBoard::R8)
    }
    pub const fn east(self) -> BitBoard {
        self.checked_shift(1, BitBoard::FH)
    }
    pub const fn south(self) -> BitBoard {
        self.checked_shift(-8, BitBoard::R1)
    }
    pub const fn west(self) -> BitBoard {
        self.checked_shift(-1, BitBoard::FA)
    }

    pub const fn northeast(self) -> BitBoard {
        self.checked_shift(9, BitBoard::R8.union(BitBoard::FH))
    }
    pub const fn northwest(self) -> BitBoard {
        self.checked_shift(7, BitBoard::R8.union(BitBoard::FA))
    }
    pub const fn southeast(self) -> BitBoard {
        self.checked_shift(-7, BitBoard::R1.union(BitBoard::FH))
    }
    pub const fn southwest(self) -> BitBoard {
        self.checked_shift(-9, BitBoard::R1.union(BitBoard::FA))
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
    type Item = (Square, BitBoard);
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.popcount() as usize;
        (len, Some(len))
    }
}

impl std::iter::ExactSizeIterator for BitBoard {}
impl std::iter::FusedIterator for BitBoard {}

impl Default for BitBoard {
    fn default() -> Self {
        Self::new()
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
    fn to_bitboard_is_from_squares(sq: Square) -> bool {
        sq.to_bitboard() == BitBoard::from_squares([sq].into_iter())
    }

    #[quickcheck]
    fn set_get(bb: BitBoard, sq: Square, val: bool) -> bool {
        bb.set_to(sq, val).contains(sq) == val
    }

    #[quickcheck]
    fn get_set(bb: BitBoard, sq: Square) -> bool {
        bb == bb.set_to(sq, bb.contains(sq))
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

    #[test]
    fn shift_unit_tests() {
        fn mk_bb(str: &str) -> BitBoard {
            Square::from_str(str).unwrap().to_bitboard()
        }
        // TODO why does this overflow???
        assert_eq!(mk_bb("d4").north(), mk_bb("d5"));
        assert_eq!(mk_bb("d4").northeast(), mk_bb("e5"));
        assert_eq!(mk_bb("d4").east(), mk_bb("e4"));
        assert_eq!(mk_bb("d4").southeast(), mk_bb("e3"));
        assert_eq!(mk_bb("d4").south(), mk_bb("d3"));
        assert_eq!(mk_bb("d4").southwest(), mk_bb("c3"));
        assert_eq!(mk_bb("d4").west(), mk_bb("c4"));
        assert_eq!(mk_bb("d4").northwest(), mk_bb("c5"));
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

    // Iterator tests
    #[quickcheck]
    fn bitboard_iter_roundrtip(bb: BitBoard) -> bool {
        bb.fold(BitBoard::EMPTY, |acc, (_, sbb)| acc.union(sbb)) == bb
    }

    #[quickcheck]
    fn bitboard_iter_is_square_iter(bb: BitBoard) -> bool {
        bb.clone().all(|(sq, bb)| sq.to_bitboard() == bb)
    }

    #[quickcheck]
    fn len_is_count(bb: BitBoard) -> bool {
        let len = bb.popcount();
        let mut count = 0;
        for _ in bb {
            count += 1;
        }
        len == count
    }

    #[quickcheck]
    fn from_squares_is_id(bb: BitBoard) -> bool {
        BitBoard::from_squares(bb.map(|(sq, _)| sq)) == bb
    }
}
