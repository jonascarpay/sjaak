use std::fmt::{self};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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
    const fn from_xy(x: u8, y: u8) -> Self {
        Square {
            index: (y << 3) | x,
        }
    }
    pub const fn from_coord(file: File, rank: Rank) -> Self {
        Self::from_xy(file.to_u8(), rank.to_u8())
    }
    pub const fn to_index(self) -> u8 {
        self.index
    }
    pub const fn from_index(index: u8) -> Option<Self> {
        if index < 64 {
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
        (Rank::from_u8(y).unwrap(), File::from_u8(x).unwrap())
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
    pub fn from_str(str: &str) -> Option<Self> {
        let mut chars = str.chars();
        let c1 = chars.next()?;
        let c2 = chars.next()?;
        if chars.next().is_some() {
            return None;
        }
        Self::from_chars(c1, c2)
    }
    pub fn all() -> impl ExactSizeIterator<Item = Square> {
        (0..64).map(|index| Square { index })
    }
    pub const fn is_dark(self) -> bool {
        let (rank, file) = self.to_xy();
        (rank % 2) == (file % 2)
    }
    pub const fn is_light(self) -> bool {
        !self.is_dark()
    }
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
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
    #[inline]
    pub const fn from_u8(value: u8) -> Option<Rank> {
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
    pub const fn to_u8(self) -> u8 {
        self as u8
    }
    #[inline]
    pub const fn from_u8(value: u8) -> Option<File> {
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
}

impl TryFrom<u8> for Rank {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Rank::from_u8(value).map_or(Err(()), Ok)
    }
}

impl TryFrom<u8> for File {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        File::from_u8(value).map_or(Err(()), Ok)
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
            Rank::from_u8(value % 8).unwrap()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                self.to_u8()
                    .shrink()
                    .map(|val| Rank::from_u8(val % 8).unwrap()),
            )
        }
    }

    impl Arbitrary for File {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let value: u8 = Arbitrary::arbitrary(g);
            File::from_u8(value % 8).unwrap()
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                self.to_u8()
                    .shrink()
                    .map(|val| File::from_u8(val % 8).unwrap()),
            )
        }
    }

    impl Arbitrary for Square {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let value: u8 = Arbitrary::arbitrary(g);
            Square::from_index(value % 64).unwrap()
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(
                self.to_index()
                    .shrink()
                    .map(|val| Square::from_index(val % 64).unwrap()),
            )
        }
    }

    #[test]
    fn unit_tests() {
        assert_eq!(Rank::R1 as u8, 0);
        assert_eq!(
            Square::from_coord(File::FA, Rank::R1,),
            Square::from_index(0).unwrap()
        );
        assert_eq!(
            Square::from_coord(File::FB, Rank::R1,),
            Square::from_index(1).unwrap()
        );
        assert_eq!(
            Square::from_coord(File::FH, Rank::R8,),
            Square::from_index(63).unwrap()
        );
        assert_eq!(
            Some(Square::from_coord(File::FH, Rank::R1,)),
            Square::from_str("h1")
        );
        assert_eq!(Square::all().len(), 64)
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
        Rank::from_u8(rank.to_u8()) == Some(rank)
    }

    #[quickcheck]
    fn file_u8_roundtrip(file: File) -> bool {
        File::from_u8(file.to_u8()) == Some(file)
    }
}
