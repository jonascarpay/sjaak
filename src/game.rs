use crate::{bitboard::BitBoard, coord::Square};

pub struct Side {
    pawns: BitBoard,
    king: BitBoard,
    queens: BitBoard,
    bishops: BitBoard,
    knights: BitBoard,
    rooks: BitBoard,
}

impl Side {
    pub const fn all(&self) -> BitBoard {
        self.pawns
            .or(self.king)
            .or(self.queens)
            .or(self.bishops)
            .or(self.knights)
            .or(self.rooks)
    }
    pub fn mk_black() -> Side {
        Side {
            pawns: BitBoard::from_squares(
                ["a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7"]
                    .into_iter()
                    .filter_map(Square::from_str),
            ),
            king: Square::from_str("e8").unwrap().to_bitboard(),
            queens: Square::from_str("d8").unwrap().to_bitboard(),
            bishops: BitBoard::from_squares(["c8", "f8"].into_iter().filter_map(Square::from_str)),
            knights: BitBoard::from_squares(["b8", "g8"].into_iter().filter_map(Square::from_str)),
            rooks: BitBoard::from_squares(["a8", "h8"].into_iter().filter_map(Square::from_str)),
        }
    }
    pub fn mk_white() -> Side {
        Side {
            pawns: BitBoard::from_squares(
                ["a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2"]
                    .into_iter()
                    .filter_map(Square::from_str),
            ),
            king: Square::from_str("e1").unwrap().to_bitboard(),
            queens: Square::from_str("d1").unwrap().to_bitboard(),
            bishops: BitBoard::from_squares(["c1", "f1"].into_iter().filter_map(Square::from_str)),
            knights: BitBoard::from_squares(["b1", "g1"].into_iter().filter_map(Square::from_str)),
            rooks: BitBoard::from_squares(["a1", "h1"].into_iter().filter_map(Square::from_str)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::side::Side;

    fn side_is_valid(side: &Side) {
        assert_eq!(side.all().len(), 16);
        assert_eq!(side.king.len(), 1);
        assert_eq!(side.queens.len(), 1);
        assert_eq!(side.bishops.len(), 2);
        assert_eq!(side.rooks.len(), 2);
        assert_eq!(side.knights.len(), 2);
        assert_eq!(side.pawns.len(), 8);
    }

    #[test]
    fn black_is_valid() {
        side_is_valid(&Side::mk_black());
    }
    #[test]
    fn white_is_valid() {
        side_is_valid(&Side::mk_white());
    }
}
