use crate::{coord::Square, zobrist_table};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PieceIndex {
    index: u8,
}

impl PieceIndex {
    pub const fn new(side: Side, piece: Piece) -> PieceIndex {
        PieceIndex {
            index: side as u8 + piece as u8 * 2,
        }
    }
    pub const fn to_side(self) -> Side {
        if self.index & 1 == 0 {
            Side::White
        } else {
            Side::Black
        }
    }
    pub const fn to_piece(self) -> Piece {
        match self.index >> 1 {
            0 => Piece::Pawn,
            1 => Piece::Knight,
            2 => Piece::Bishop,
            3 => Piece::Rook,
            4 => Piece::Queen,
            5 => Piece::King,
            _ => unreachable!(),
        }
    }
    pub const fn to_index(self) -> u8 {
        self.index
    }
    pub fn get_key(self, sq: Square) -> u64 {
        zobrist_table::get_key(sq, self)
    }
    pub const fn from_fen_char(char: char) -> Option<Self> {
        use {Piece::*, Side::*};
        match char {
            'P' => Some(Self::new(White, Pawn)),
            'N' => Some(Self::new(White, Knight)),
            'B' => Some(Self::new(White, Bishop)),
            'R' => Some(Self::new(White, Rook)),
            'Q' => Some(Self::new(White, Queen)),
            'K' => Some(Self::new(White, King)),
            'p' => Some(Self::new(Black, Pawn)),
            'n' => Some(Self::new(Black, Knight)),
            'b' => Some(Self::new(Black, Bishop)),
            'r' => Some(Self::new(Black, Rook)),
            'q' => Some(Self::new(Black, Queen)),
            'k' => Some(Self::new(Black, King)),
            _ => None,
        }
    }
    pub const fn to_fen_char(self) -> char {
        match (self.to_side(), self.to_piece()) {
            (Side::White, Piece::Pawn) => 'P',
            (Side::White, Piece::Knight) => 'N',
            (Side::White, Piece::Bishop) => 'B',
            (Side::White, Piece::Rook) => 'R',
            (Side::White, Piece::Queen) => 'Q',
            (Side::White, Piece::King) => 'K',
            (Side::Black, Piece::Pawn) => 'p',
            (Side::Black, Piece::Knight) => 'n',
            (Side::Black, Piece::Bishop) => 'b',
            (Side::Black, Piece::Rook) => 'r',
            (Side::Black, Piece::Queen) => 'q',
            (Side::Black, Piece::King) => 'k',
        }
    }
    pub fn debug_assert_valid(self) {
        debug_assert!(self.to_index() < 12)
    }
}

pub enum Side {
    White = 0,
    Black = 1,
}

// TODO This -> PieceType and PieceIndex -> Piece?
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Side {
    pub const fn to_index(self, piece: Piece) -> PieceIndex {
        PieceIndex::new(self, piece)
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
            PieceIndex {
                index: (ix % 12) as u8,
            }
        }
    }

    #[quickcheck]
    fn fen_roundtrip(pc: PieceIndex) -> bool {
        PieceIndex::from_fen_char(pc.to_fen_char()) == Some(pc)
    }

    #[quickcheck]
    fn side_piece_roundtrip(pc: PieceIndex) -> bool {
        pc == PieceIndex::new(pc.to_side(), pc.to_piece())
    }
}
