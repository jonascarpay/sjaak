use crate::bitboard::BitBoard;

impl BitBoard {
    pub const fn iter_powerset(self) -> PowerSetIter {
        PowerSetIter::from_bitboard(self)
    }
}

#[derive(Clone, Debug)]
// Yields both the full and empty sets
pub struct PowerSetIter {
    mask: u64,
    next: u64,
    done: bool,
}

impl PowerSetIter {
    pub const fn from_bitboard(bb: BitBoard) -> Self {
        PowerSetIter {
            mask: bb.to_bits(),
            next: bb.to_bits(),
            done: false,
        }
    }
    pub const fn remaining(&self) -> u64 {
        1 << self.next.count_ones()
    }
    pub const fn pop(&mut self) -> Option<BitBoard> {
        if self.done {
            None
        } else {
            let current = self.next;
            if current == 0 {
                self.done = true;
            } else {
                self.next = (current - 1) & self.mask;
            }
            Some(BitBoard::from_bits(current))
        }
    }
}

impl Iterator for PowerSetIter {
    type Item = BitBoard;
    fn next(&mut self) -> Option<BitBoard> {
        self.pop()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining() as usize;
        (len, Some(len))
    }
}

impl std::iter::ExactSizeIterator for PowerSetIter {}
impl std::iter::FusedIterator for PowerSetIter {}

#[cfg(test)]
mod tests {
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    use crate::bitboard::BitBoard;

    #[derive(Clone, Debug)]
    struct SmallBitBoard(BitBoard);

    impl Arbitrary for SmallBitBoard {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut bb: BitBoard = Arbitrary::arbitrary(g);
            while bb.popcount() > 8 {
                bb = bb.intersect(Arbitrary::arbitrary(g));
            }
            SmallBitBoard(bb)
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.0.iter_powerset().skip(1).map(|bb| SmallBitBoard(bb)))
        }
    }

    #[quickcheck]
    fn count_is_accurate(SmallBitBoard(bb): SmallBitBoard) -> bool {
        let ps = bb.iter_powerset();
        let l = ps.len();
        let mut count = 0;
        for _ in ps {
            println!("{}", count);
            count += 1;
        }
        count == l
    }

    #[quickcheck]
    fn powerset_contains_every_unit_set(SmallBitBoard(bb): SmallBitBoard) -> bool {
        bb.clone()
            .all(|(_, sbb)| bb.iter_powerset().any(|bb| sbb == bb))
    }

    #[quickcheck]
    fn powerset_contains_original(SmallBitBoard(bb): SmallBitBoard) -> bool {
        bb.iter_powerset().any(|bb_| bb_ == bb)
    }

    #[quickcheck]
    fn powerset_contains_empty_set(SmallBitBoard(bb): SmallBitBoard) -> bool {
        bb.iter_powerset().any(|bb| bb.is_empty())
    }
}
