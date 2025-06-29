use crate::{bitboard::BitBoard, coord::Square};

#[derive(Clone, Debug, PartialEq, Eq)]
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
            .union(self.king)
            .union(self.queens)
            .union(self.bishops)
            .union(self.knights)
            .union(self.rooks)
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
    // TODO write this more semantically
    pub const WHITE: Side = Side {
        pawns: BitBoard::from_bits(0xff00),
        king: BitBoard::from_bits(0x10),
        queens: BitBoard::from_bits(0x8),
        bishops: BitBoard::from_bits(0x24),
        knights: BitBoard::from_bits(0x42),
        rooks: BitBoard::from_bits(0x81),
    };
    pub const BLACK: Side = Side {
        pawns: BitBoard::from_bits(0x00ff_0000_0000_0000),
        king: BitBoard::from_bits(0x1000_0000_0000_0000),
        queens: BitBoard::from_bits(0x0800_0000_0000_0000),
        bishops: BitBoard::from_bits(0x2400_0000_0000_0000),
        knights: BitBoard::from_bits(0x4200_0000_0000_0000),
        rooks: BitBoard::from_bits(0x8100_0000_0000_0000),
    };
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
    fn black_is_mk_black() {
        let lhs = Side::BLACK;
        let rhs = Side::mk_black();
        assert_eq!(lhs, rhs, "{:#x?}", rhs);
    }

    #[test]
    fn white_is_valid() {
        side_is_valid(&Side::mk_white());
    }

    #[test]
    fn white_is_mk_white() {
        let lhs = Side::WHITE;
        let rhs = Side::mk_white();
        assert_eq!(lhs, rhs)
    }
}
