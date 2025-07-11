use crate::{coord::Square, zobrist_table};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Piece {
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

impl Piece {
    pub const fn to_side(self) -> Side {
        Side::from_index(self.to_index() & 1).unwrap() // ASM checked
    }
    pub const fn to_piece(self) -> PieceType {
        PieceType::from_index(self.to_index() >> 1).unwrap() // ASM checked
    }
    pub const fn from_side_piece(side: Side, piece: PieceType) -> Piece {
        Self::from_index(piece.to_index() * 2 + side.to_index()).unwrap() // ASM checked
    }
    pub const fn to_index(self) -> u8 {
        self as u8
    }
    pub fn get_key(self, sq: Square) -> u64 {
        zobrist_table::get_key(sq, self) // ASM checked
    }
    pub fn flip_side(self) -> Self {
        Self::from_index(self.to_index() ^ 1).unwrap() // ASM checked
    }
    pub fn is_black(self) -> bool {
        self.to_side() == Side::Black
    }
    pub fn to_white(self) -> Self {
        // A little ugly, but this is the only implementation I could find that optimizes correctly
        if self.is_black() {
            self.flip_side()
        } else {
            self
        }
    }
    pub fn to_black(self) -> Self {
        self.to_white().flip_side() // ASM checked
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
    pub const fn to_unicode(self) -> char {
        match self {
            Self::WhitePawn => '♙',
            Self::WhiteKnight => '♘',
            Self::WhiteBishop => '♗',
            Self::WhiteRook => '♖',
            Self::WhiteQueen => '♕',
            Self::WhiteKing => '♔',
            Self::BlackPawn => '♟',
            Self::BlackKnight => '♞',
            Self::BlackBishop => '♝',
            Self::BlackRook => '♜',
            Self::BlackQueen => '♛',
            Self::BlackKing => '♚',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl PieceType {
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

    use super::Piece;

    impl Arbitrary for Piece {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let ix: usize = Arbitrary::arbitrary(g);
            Piece::from_index(ix as u8 % 12).unwrap()
        }
    }

    #[quickcheck]
    fn fen_roundtrip(pc: Piece) -> bool {
        Piece::from_fen_char(pc.to_fen_char()) == Some(pc)
    }

    #[quickcheck]
    fn index_roundtrip(pc: Piece) -> bool {
        Piece::from_index(pc.to_index()) == Some(pc)
    }

    #[quickcheck]
    fn side_piece_roundtrip(pc: Piece) -> bool {
        pc == Piece::from_side_piece(pc.to_side(), pc.to_piece())
    }
}
