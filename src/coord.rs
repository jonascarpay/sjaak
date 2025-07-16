use std::{
    fmt::{self},
    iter::FusedIterator,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
// Played around with representing this as an enum, but it doesn't optimize away very nicely.
// For example, just to to_index . from_index roundtrip did not optimize into a noop
pub struct Square {
    index: u8,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rank, file) = self.to_coord();
        write!(f, "{}{}", file.to_char(), rank.to_char(),)
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

impl Square {
    pub const NUM_SQUARES: u8 = 64;
    const fn from_xy(x: u8, y: u8) -> Self {
        Square {
            index: (y << 3) | x,
        }
    }
    pub const fn from_coord(file: File, rank: Rank) -> Self {
        Self::from_xy(file.to_index(), rank.to_index())
    }
    pub const fn to_index(self) -> u8 {
        self.index
    }
    pub const fn from_index(index: u8) -> Option<Self> {
        if index < Self::NUM_SQUARES {
            Some(Square { index })
        } else {
            None
        }
    }
    #[inline(always)]
    pub const fn to_xy(self) -> (u8, u8) {
        let x = self.index % 8;
        let y = (self.index >> 3) % 8;
        (x, y)
    }
    pub const fn to_coord(self) -> (Rank, File) {
        let (x, y) = self.to_xy();
        (Rank::from_index(y).unwrap(), File::from_index(x).unwrap())
    }
    pub const fn from_chars(file: char, rank: char) -> Option<Self> {
        let rank = Rank::from_char(rank);
        let file = File::from_char(file);
        if let (Some(rank), Some(file)) = (rank, file) {
            Some(Self::from_coord(file, rank))
        } else {
            None
        }
    }
    // TODO this conflicts with std::FromStr, implement that instead?
    pub fn from_str(str: &str) -> Option<Self> {
        let mut chars = str.chars();
        let c1 = chars.next()?;
        let c2 = chars.next()?;
        if chars.next().is_some() {
            return None;
        }
        Self::from_chars(c1, c2)
    }
    pub fn iter_all() -> impl ExactSizeIterator<Item = Square> + Clone {
        (0..Self::NUM_SQUARES).map(|index| Square { index })
    }

    pub const fn is_dark(self) -> bool {
        let (rank, file) = self.to_xy();
        (rank % 2) == (file % 2)
    }
    pub const fn is_light(self) -> bool {
        !self.is_dark()
    }

    pub const fn hflip(self) -> Square {
        Square {
            index: self.to_index() ^ 7,
        }
    }

    pub const fn vflip(self) -> Square {
        Square {
            index: self.to_index() ^ 56,
        }
    }

    pub const fn reverse(self) -> Square {
        Square {
            index: 63 - self.to_index(),
        }
    }

    #[inline(always)]
    pub const fn offset(self, offset_file: i8, offset_rank: i8) -> Option<Square> {
        // cargo asm shows that in general this function optimizes/specializes very nicely
        let (rank, file) = self.to_coord();
        if let (Some(rank), Some(file)) = (rank.offset(offset_rank), file.offset(offset_file)) {
            Some(Square::from_coord(file, rank))
        } else {
            None
        }
    }

    pub const A1: Square = Square { index: 0 };
    pub const B1: Square = Square { index: 1 };
    pub const C1: Square = Square { index: 2 };
    pub const D1: Square = Square { index: 3 };
    pub const E1: Square = Square { index: 4 };
    pub const F1: Square = Square { index: 5 };
    pub const G1: Square = Square { index: 6 };
    pub const H1: Square = Square { index: 7 };
    pub const A2: Square = Square { index: 8 };
    pub const B2: Square = Square { index: 9 };
    pub const C2: Square = Square { index: 10 };
    pub const D2: Square = Square { index: 11 };
    pub const E2: Square = Square { index: 12 };
    pub const F2: Square = Square { index: 13 };
    pub const G2: Square = Square { index: 14 };
    pub const H2: Square = Square { index: 15 };
    pub const A3: Square = Square { index: 16 };
    pub const B3: Square = Square { index: 17 };
    pub const C3: Square = Square { index: 18 };
    pub const D3: Square = Square { index: 19 };
    pub const E3: Square = Square { index: 20 };
    pub const F3: Square = Square { index: 21 };
    pub const G3: Square = Square { index: 22 };
    pub const H3: Square = Square { index: 23 };
    pub const A4: Square = Square { index: 24 };
    pub const B4: Square = Square { index: 25 };
    pub const C4: Square = Square { index: 26 };
    pub const D4: Square = Square { index: 27 };
    pub const E4: Square = Square { index: 28 };
    pub const F4: Square = Square { index: 29 };
    pub const G4: Square = Square { index: 30 };
    pub const H4: Square = Square { index: 31 };
    pub const A5: Square = Square { index: 32 };
    pub const B5: Square = Square { index: 33 };
    pub const C5: Square = Square { index: 34 };
    pub const D5: Square = Square { index: 35 };
    pub const E5: Square = Square { index: 36 };
    pub const F5: Square = Square { index: 37 };
    pub const G5: Square = Square { index: 38 };
    pub const H5: Square = Square { index: 39 };
    pub const A6: Square = Square { index: 40 };
    pub const B6: Square = Square { index: 41 };
    pub const C6: Square = Square { index: 42 };
    pub const D6: Square = Square { index: 43 };
    pub const E6: Square = Square { index: 44 };
    pub const F6: Square = Square { index: 45 };
    pub const G6: Square = Square { index: 46 };
    pub const H6: Square = Square { index: 47 };
    pub const A7: Square = Square { index: 48 };
    pub const B7: Square = Square { index: 49 };
    pub const C7: Square = Square { index: 50 };
    pub const D7: Square = Square { index: 51 };
    pub const E7: Square = Square { index: 52 };
    pub const F7: Square = Square { index: 53 };
    pub const G7: Square = Square { index: 54 };
    pub const H7: Square = Square { index: 55 };
    pub const A8: Square = Square { index: 56 };
    pub const B8: Square = Square { index: 57 };
    pub const C8: Square = Square { index: 58 };
    pub const D8: Square = Square { index: 59 };
    pub const E8: Square = Square { index: 60 };
    pub const F8: Square = Square { index: 61 };
    pub const G8: Square = Square { index: 62 };
    pub const H8: Square = Square { index: 63 };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Rank {
    R1 = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum File {
    FA = 0,
    FB = 1,
    FC = 2,
    FD = 3,
    FE = 4,
    FF = 5,
    FG = 6,
    FH = 7,
}

impl Rank {
    pub const ALL: [Rank; 8] = [
        Rank::R1,
        Rank::R2,
        Rank::R3,
        Rank::R4,
        Rank::R5,
        Rank::R6,
        Rank::R7,
        Rank::R8,
    ];
    pub const fn to_char(self) -> char {
        match self {
            Rank::R1 => '1',
            Rank::R2 => '2',
            Rank::R3 => '3',
            Rank::R4 => '4',
            Rank::R5 => '5',
            Rank::R6 => '6',
            Rank::R7 => '7',
            Rank::R8 => '8',
        }
    }
    pub const fn from_ascii(byte: u8) -> Option<Self> {
        match char::from_u32(byte as u32) {
            None => None,
            Some(c) => Self::from_char(c),
        }
    }
    pub const fn from_char(char: char) -> Option<Self> {
        match char {
            '1' => Some(Rank::R1),
            '2' => Some(Rank::R2),
            '3' => Some(Rank::R3),
            '4' => Some(Rank::R4),
            '5' => Some(Rank::R5),
            '6' => Some(Rank::R6),
            '7' => Some(Rank::R7),
            '8' => Some(Rank::R8),
            _ => None,
        }
    }
    #[inline]
    pub const fn to_index(self) -> u8 {
        self as u8
    }
    #[inline]
    pub const fn from_index(value: u8) -> Option<Rank> {
        match value {
            0 => Some(Rank::R1),
            1 => Some(Rank::R2),
            2 => Some(Rank::R3),
            3 => Some(Rank::R4),
            4 => Some(Rank::R5),
            5 => Some(Rank::R6),
            6 => Some(Rank::R7),
            7 => Some(Rank::R8),
            _ => None,
        }
    }

    #[inline]
    const fn offset(self, offset: i8) -> Option<Self> {
        // verified by cargo asm that this optimizes/specializes nicely
        Self::from_index((self.to_index() as i8 + offset) as u8)
    }
}

impl File {
    pub const ALL: [File; 8] = [
        File::FA,
        File::FB,
        File::FC,
        File::FD,
        File::FE,
        File::FF,
        File::FG,
        File::FH,
    ];
    pub const fn to_char(self) -> char {
        match self {
            File::FA => 'a',
            File::FB => 'b',
            File::FC => 'c',
            File::FD => 'd',
            File::FE => 'e',
            File::FF => 'f',
            File::FG => 'g',
            File::FH => 'h',
        }
    }
    pub const fn from_ascii(byte: u8) -> Option<Self> {
        match char::from_u32(byte as u32) {
            None => None,
            Some(char) => Self::from_char(char),
        }
    }
    pub const fn from_char(char: char) -> Option<Self> {
        match char {
            'A' => Some(File::FA),
            'B' => Some(File::FB),
            'C' => Some(File::FC),
            'D' => Some(File::FD),
            'E' => Some(File::FE),
            'F' => Some(File::FF),
            'G' => Some(File::FG),
            'H' => Some(File::FH),
            'a' => Some(File::FA),
            'b' => Some(File::FB),
            'c' => Some(File::FC),
            'd' => Some(File::FD),
            'e' => Some(File::FE),
            'f' => Some(File::FF),
            'g' => Some(File::FG),
            'h' => Some(File::FH),
            _ => None,
        }
    }
    #[inline]
    pub const fn to_index(self) -> u8 {
        self as u8
    }
    #[inline]
    pub const fn from_index(value: u8) -> Option<File> {
        match value {
            0 => Some(File::FA),
            1 => Some(File::FB),
            2 => Some(File::FC),
            3 => Some(File::FD),
            4 => Some(File::FE),
            5 => Some(File::FF),
            6 => Some(File::FG),
            7 => Some(File::FH),
            _ => None,
        }
    }

    #[inline]
    const fn offset(self, offset: i8) -> Option<Self> {
        Self::from_index((self.to_index() as i8 + offset) as u8)
    }
}

// TODO this is probably not used in a hot loop.
// TODO or anywhere?
// If it were, Option<Square> could be optimized into a more compact representation, maybe
pub struct Ray {
    next: Option<Square>,
    offset_rank: i8,
    offset_file: i8,
}

impl Ray {
    pub const fn new_incl(src: Square, offset_file: i8, offset_rank: i8) -> Self {
        Ray {
            next: Some(src),
            offset_rank,
            offset_file,
        }
    }
    pub const fn new_excl(src: Square, offset_file: i8, offset_rank: i8) -> Self {
        let mut res = Self::new_incl(src, offset_file, offset_rank);
        res.step();
        res
    }
    pub const fn step(&mut self) -> Option<Square> {
        let current = self.next;
        if let Some(sq) = current {
            self.next = sq.offset(self.offset_file, self.offset_rank);
        }
        current
    }
}

impl Iterator for Ray {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        self.step()
    }
}

impl FusedIterator for Ray {}

// TODO Can probably all be replaced by Ray, check once magic has been implemented
#[rustfmt::skip]
impl Square {
    pub const fn north_by(self,     offset: i8) -> Option<Square> { self.offset(0, offset)  }
    pub const fn south_by(self,     offset: i8) -> Option<Square> { self.offset(0, -offset) }
    pub const fn east_by(self,      offset: i8) -> Option<Square> { self.offset(offset,  0)       }
    pub const fn west_by(self,      offset: i8) -> Option<Square> { self.offset(-offset, 0)       }
    pub const fn northeast_by(self, offset: i8) -> Option<Square> { self.offset(offset,  offset)  }
    pub const fn southeast_by(self, offset: i8) -> Option<Square> { self.offset(offset,  -offset) }
    pub const fn northwest_by(self, offset: i8) -> Option<Square> { self.offset(-offset, offset)  }
    pub const fn southwest_by(self, offset: i8) -> Option<Square> { self.offset(-offset, -offset) }
    pub const fn north(self)     -> Option<Square> { self.offset(0,  1)  }
    pub const fn south(self)     -> Option<Square> { self.offset(0,  -1) }
    pub const fn east(self)      -> Option<Square> { self.offset(1,  0)  }
    pub const fn west(self)      -> Option<Square> { self.offset(-1, 0) }
    pub const fn northeast(self) -> Option<Square> { self.offset(1,  1)  }
    pub const fn southeast(self) -> Option<Square> { self.offset(1,  -1) }
    pub const fn northwest(self) -> Option<Square> { self.offset(-1, 1)  }
    pub const fn southwest(self) -> Option<Square> { self.offset(-1, -1) }
}

#[rustfmt::skip]
impl Rank {
    pub const fn north_by(self, offset: i8) -> Option<Self> { self.offset(offset) }
    pub const fn south_by(self, offset: i8) -> Option<Self> { self.offset(-offset) }
    pub const fn south(self) -> Option<Rank> { self.offset(-1) }
    pub const fn north(self) -> Option<Rank> { self.offset(1) }
}

#[rustfmt::skip]
impl File {
    pub const fn east_by(self, offset: i8) -> Option<Self> { self.offset(offset) }
    pub const fn west_by(self, offset: i8) -> Option<Self> { self.offset(-offset) }
    pub const fn east(self) -> Option<File> { self.offset(1) }
    pub const fn west(self) -> Option<File> { self.offset(-1) }
}

impl TryFrom<u8> for Rank {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Rank::from_index(value).ok_or(())
    }
}

impl TryFrom<u8> for File {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        File::from_index(value).ok_or(())
    }
}

#[cfg(test)]
pub mod tests {
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    use super::{File, Rank, Square};

    impl Arbitrary for Rank {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let value: u8 = Arbitrary::arbitrary(g);
            Rank::from_index(value % 8).unwrap()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                self.to_index()
                    .shrink()
                    .map(|val| Rank::from_index(val % 8).unwrap()),
            )
        }
    }

    impl Arbitrary for File {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let value: u8 = Arbitrary::arbitrary(g);
            File::from_index(value % 8).unwrap()
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                self.to_index()
                    .shrink()
                    .map(|val| File::from_index(val % 8).unwrap()),
            )
        }
    }

    impl Arbitrary for Square {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let value: u8 = Arbitrary::arbitrary(g);
            Square::from_index(value % Self::NUM_SQUARES).unwrap()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                self.to_index()
                    .shrink()
                    .map(|val| Square::from_index(val % Self::NUM_SQUARES).unwrap()),
            )
        }
    }

    #[test]
    fn unit_tests() {
        assert_eq!(Rank::R1 as u8, 0);
        assert_eq!(Square::from_coord(File::FA, Rank::R1,), Square::A1);
        assert_eq!(Square::from_coord(File::FB, Rank::R1,), Square::B1);
        assert_eq!(Square::from_coord(File::FH, Rank::R8,), Square::H8);
        assert_eq!(Some(Square::H1), Square::from_str("h1"));
        assert_eq!(Square::iter_all().len(), 64)
    }

    #[quickcheck]
    fn coord_roundtrip(sq: Square) -> bool {
        let (rank, file) = sq.to_coord();
        Square::from_coord(file, rank) == sq
    }

    #[quickcheck]
    fn xy_roundtrip(sq: Square) -> bool {
        let (file, rank) = sq.to_xy();
        Square::from_xy(file, rank) == sq
    }

    #[quickcheck]
    fn str_roundtrip(sq: Square) -> bool {
        let str = format!("{}", sq);
        Square::from_str(&str) == Some(sq)
    }

    #[quickcheck]
    fn square_u8_roundtrip(sq: Square) -> bool {
        Square::from_index(sq.to_index()) == Some(sq)
    }

    #[quickcheck]
    fn rank_u8_roundtrip(rank: Rank) -> bool {
        Rank::from_index(rank.to_index()) == Some(rank)
    }

    #[quickcheck]
    fn file_u8_roundtrip(file: File) -> bool {
        File::from_index(file.to_index()) == Some(file)
    }

    #[quickcheck]
    fn offset_bidirectional(src: Square, offset_file: i8, offset_rank: i8) -> bool {
        let offset_rank = offset_rank / 2; // Avoid overflows
        let offset_file = offset_file / 2; // Avoid overflows
        if let Some(dst) = src.offset(offset_file, offset_rank) {
            dst.offset(-offset_file, -offset_rank) == Some(src)
        } else {
            true
        }
    }

    #[quickcheck]
    fn only_offset_0_is_id(src: Square, offset_file: i8, offset_rank: i8) -> bool {
        let offset_rank = offset_rank / 2; // Avoid overflows
        let offset_file = offset_file / 2; // Avoid overflows

        let dst = src.offset(offset_file, offset_rank);
        let is_id = dst == Some(src);
        let is_zero = offset_file == 0 && offset_rank == 0;
        is_id == is_zero
    }

    #[quickcheck]
    fn hflip_involution(sq: Square) -> bool {
        sq.hflip().hflip() == sq
    }

    #[quickcheck]
    fn vflip_involution(sq: Square) -> bool {
        sq.vflip().vflip() == sq
    }

    #[quickcheck]
    fn reverse_involution(sq: Square) -> bool {
        sq.reverse().reverse() == sq
    }

    #[quickcheck]
    fn flips_commute(sq: Square) -> bool {
        sq.vflip().hflip() == sq.hflip().vflip()
    }

    #[quickcheck]
    fn flips_are_reverse(sq: Square) -> bool {
        sq.vflip().hflip() == sq.reverse()
    }
}
