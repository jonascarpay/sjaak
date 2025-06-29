use crate::bitboard::BitBoard;

#[derive(Clone)]
// Does not return the empty set
// TODO: check if that makes sense
pub struct PowerSet {
    mask: u64,
    next: u64,
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
        }
    }
    const fn to_bitboard(self) -> BitBoard {
        BitBoard::from_bits(self.mask)
    }
    pub const fn remaining(&self) -> u64 {
        (1 << self.next.count_ones()) - 1
    }
}

impl Iterator for PowerSet {
    type Item = BitBoard;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next == 0 {
            return None;
        } else {
            let current = self.next;
            self.next = (current - 1) & self.mask;
            Some(BitBoard::from_bits(current))
        }
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

    use super::PowerSet;

    impl Arbitrary for PowerSet {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let mut bb: BitBoard = Arbitrary::arbitrary(g);
            while bb.len() > 8 {
                bb = bb.intersect(Arbitrary::arbitrary(g));
            }
            bb.powerset()
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.clone().skip(1).map(|bb| bb.powerset()))
        }
    }

    #[quickcheck]
    fn count_is_accurate(ps: PowerSet) -> bool {
        let l = ps.len();
        let mut count = 0;
        for _ in ps {
            println!("{}", count);
            count += 1;
        }
        count == l
    }

    #[quickcheck]
    fn powerset_contains_every_unit_set(bb: BitBoard) -> bool {
        if bb.len() < 8 {
            bb.clone()
                .all(|sq| bb.powerset().any(|bb| sq.to_bitboard() == bb))
        } else {
            true
        }
    }
}
