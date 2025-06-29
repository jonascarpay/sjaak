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
            format_board_fancy(|sq| if self.get(sq) { 'x' } else { ' ' }).unwrap()
        )
    }
}

impl BitBoard {
    pub const fn new() -> Self {
        Self::EMPTY
    }
    pub fn get(self, sq: Square) -> bool {
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

    pub fn from_squares<I: Iterator<Item = Square>>(iter: I) -> Self {
        iter.fold(Self::EMPTY, |acc, el| acc.union(el.to_bitboard()))
    }
    pub const fn to_bits(self) -> u64 {
        self.bits
    }
    pub const fn from_bits(bits: u64) -> Self {
        BitBoard { bits }
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
    pub const fn symmetric_difference(self, rhs: BitBoard) -> BitBoard {
        BitBoard {
            bits: self.bits ^ rhs.bits,
        }
    }

    pub const fn reverse(self) -> BitBoard {
        BitBoard {
            bits: self.bits.reverse_bits(),
        }
    }
    pub const EMPTY: BitBoard = BitBoard { bits: 0 };
    pub const RIM: BitBoard = {
        Rank::R1
            .to_bitboard()
            .union(Rank::R8.to_bitboard())
            .union(File::FA.to_bitboard())
            .union(File::FH.to_bitboard())
    };
}

impl Iterator for BitBoard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            None
        } else {
            let ix = self.bits.ilog2() as u8;
            let sq = Square::from_index(ix).unwrap();
            self.unset_assign(sq);
            Some(sq)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl ExactSizeIterator for BitBoard {
    fn len(&self) -> usize {
        self.bits.count_ones() as usize
    }
}

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
    pub fn bishop_moves(self) -> BitBoard {
        todo!()
    }
    pub fn rook_moves(self) -> BitBoard {
        todo!()
    }
    pub fn queen_moves(self) -> BitBoard {
        self.rook_moves().union(self.bishop_moves())
    }
    pub fn king_moves(self) -> BitBoard {
        todo!()
    }
}

impl Rank {
    pub const fn to_bitboard(self) -> BitBoard {
        BitBoard::from_bits(0xFF << (self.to_u8() << 3))
    }
}

impl File {
    pub const fn to_bitboard(self) -> BitBoard {
        BitBoard::from_bits(0x0101_0101_0101_0101 << self.to_u8())
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
        bb.set_to(sq, val).get(sq) == val
    }

    #[quickcheck]
    fn get_set(bb: BitBoard, sq: Square) -> bool {
        bb == bb.set_to(sq, bb.get(sq))
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
}
