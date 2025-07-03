use crate::bitboard::BitBoard;

#[derive(Clone)]
// Does not yield the empty set
// TODO: check if that makes sense, or if it's better to not yield the original
pub struct PowerSet {
    mask: u64,
    next: u64,
    done: bool,
}

impl std::fmt::Debug for PowerSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.clone().to_bitboard(), f)
    }
}

impl BitBoard {
    pub const fn powerset(self) -> PowerSet {
        PowerSet::from_bitboard(self)
    }
}

impl PowerSet {
    pub const fn from_bitboard(bb: BitBoard) -> Self {
        PowerSet {
            mask: bb.to_bits(),
            next: bb.to_bits(),
            done: false,
        }
    }
    const fn to_bitboard(&self) -> BitBoard {
        BitBoard::from_bits(self.mask)
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

impl Iterator for PowerSet {
    type Item = BitBoard;

    fn next(&mut self) -> Option<BitBoard> {
        self.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining() as usize;
        (len, Some(len))
    }
}

impl std::iter::ExactSizeIterator for PowerSet {}
impl std::iter::FusedIterator for PowerSet {}

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
            while bb.len() > 8 {
                bb = bb.intersect(Arbitrary::arbitrary(g));
            }
            SmallBitBoard(bb)
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.0.powerset().skip(1).map(|bb| SmallBitBoard(bb)))
        }
    }

    #[quickcheck]
    fn count_is_accurate(SmallBitBoard(bb): SmallBitBoard) -> bool {
        let ps = bb.powerset();
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
            .all(|sq| bb.powerset().any(|bb| sq.to_bitboard() == bb))
    }

    #[quickcheck]
    fn powerset_contains_original(SmallBitBoard(bb): SmallBitBoard) -> bool {
        bb.powerset().any(|bb_| bb_ == bb)
    }

    #[quickcheck]
    fn powerset_contains_empty_set(SmallBitBoard(bb): SmallBitBoard) -> bool {
        bb.powerset().any(|bb| bb == BitBoard::EMPTY)
    }
}
