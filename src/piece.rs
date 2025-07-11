use crate::{coord::Square, zobrist_table};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceIndex {
    WhitePawn = 0,
    BlackPawn = 1,
    WhiteKnight = 2,
    BlackKnight = 3,
    WhiteBishop = 4,
    BlackBishop = 5,
    WhiteRook = 6,
    BlackRook = 7,
    WhiteQueen = 8,
    BlackQueen = 9,
    WhiteKing = 10,
    BlackKing = 11,
}

impl PieceIndex {
    pub const fn to_side(self) -> Side {
        // Checked that this optimizes away correctly
        Side::from_index(self.to_index() & 1).unwrap()
    }
    pub const fn to_piece(self) -> Piece {
        // Checked that this optimizes away correctly
        Piece::from_index(self.to_index() >> 1).unwrap()
    }
    pub const fn from_side_piece(side: Side, piece: Piece) -> PieceIndex {
        // Checked that this optimizes away correctly
        Self::from_index(piece.to_index() * 2 + side.to_index()).unwrap()
    }
    pub const fn to_index(self) -> u8 {
        self as u8
    }
    pub fn get_key(self, sq: Square) -> u64 {
        zobrist_table::get_key(sq, self)
    }
    pub const fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::WhitePawn),
            1 => Some(Self::BlackPawn),
            2 => Some(Self::WhiteKnight),
            3 => Some(Self::BlackKnight),
            4 => Some(Self::WhiteBishop),
            5 => Some(Self::BlackBishop),
            6 => Some(Self::WhiteRook),
            7 => Some(Self::BlackRook),
            8 => Some(Self::WhiteQueen),
            9 => Some(Self::BlackQueen),
            10 => Some(Self::WhiteKing),
            11 => Some(Self::BlackKing),
            _ => None,
        }
    }
    pub const fn from_fen_char(char: char) -> Option<Self> {
        match char {
            'P' => Some(Self::WhitePawn),
            'N' => Some(Self::WhiteKnight),
            'B' => Some(Self::WhiteBishop),
            'R' => Some(Self::WhiteRook),
            'Q' => Some(Self::WhiteQueen),
            'K' => Some(Self::WhiteKing),
            'p' => Some(Self::BlackPawn),
            'n' => Some(Self::BlackKnight),
            'b' => Some(Self::BlackBishop),
            'r' => Some(Self::BlackRook),
            'q' => Some(Self::BlackQueen),
            'k' => Some(Self::BlackKing),
            _ => None,
        }
    }
    pub const fn to_fen_char(self) -> char {
        match self {
            Self::WhitePawn => 'P',
            Self::WhiteKnight => 'N',
            Self::WhiteBishop => 'B',
            Self::WhiteRook => 'R',
            Self::WhiteQueen => 'Q',
            Self::WhiteKing => 'K',
            Self::BlackPawn => 'p',
            Self::BlackKnight => 'n',
            Self::BlackBishop => 'b',
            Self::BlackRook => 'r',
            Self::BlackQueen => 'q',
            Self::BlackKing => 'k',
        }
    }
}

pub enum Side {
    White = 0,
    Black = 1,
}

impl Side {
    pub const fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::White),
            1 => Some(Self::Black),
            _ => None,
        }
    }
    pub const fn to_index(self) -> u8 {
        self as u8
    }
}

pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    pub const fn to_index(self) -> u8 {
        self as u8
    }
    pub const fn from_index(index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Pawn),
            1 => Some(Self::Knight),
            2 => Some(Self::Bishop),
            3 => Some(Self::Rook),
            4 => Some(Self::Queen),
            5 => Some(Self::King),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    use super::PieceIndex;

    impl Arbitrary for PieceIndex {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let ix: usize = Arbitrary::arbitrary(g);
            PieceIndex::from_index(ix as u8 % 12).unwrap()
        }
    }

    #[quickcheck]
    fn fen_roundtrip(pc: PieceIndex) -> bool {
        PieceIndex::from_fen_char(pc.to_fen_char()) == Some(pc)
    }

    #[quickcheck]
    fn index_roundtrip(pc: PieceIndex) -> bool {
        PieceIndex::from_index(pc.to_index()) == Some(pc)
    }

    #[quickcheck]
    fn side_piece_roundtrip(pc: PieceIndex) -> bool {
        pc == PieceIndex::from_side_piece(pc.to_side(), pc.to_piece())
    }
}
