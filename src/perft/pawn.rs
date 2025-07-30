use crate::{bitboard::BitBoard, piece::Piece};

use super::node::Node;

const WHITE_PROMOTER_MASK: BitBoard = BitBoard::R7.union(BitBoard::R8);

impl Node {
    #[inline(always)]
    pub fn white_pawns(&self) -> BitBoard {
        self.piece(Piece::WhitePawn)
    }
    pub fn for_white_simple_push<F: FnMut(Node)>(&self, mut f: F) {
        let pawns = self
            .white_pawns()
            .difference(WHITE_PROMOTER_MASK)
            .difference(self.occupancy_total.rshift(8));
        for (_, bb) in pawns {
            let mut pos = self.clone();
            let bb_new = bb.lshift(8);
            pos.reset_en_passant();
            pos.apply_move(Piece::WhitePawn, bb.union(bb_new));
            f(pos);
        }
    }

    pub fn for_white_double_push<F: FnMut(Node)>(&self, mut f: F) {
        let pawns = self
            .white_pawns()
            .intersect(BitBoard::R2)
            .difference(self.occupancy_total.rshift(8))
            .difference(self.occupancy_total.rshift(16));
        for (_, bb) in pawns {
            let mut pos = self.clone();
            let bb_new = bb.lshift(16);
            pos.en_passant_square = bb.lshift(8).get_square().unwrap(); // TODO unchecked
            pos.apply_move(Piece::WhitePawn, bb.union(bb_new));
            f(pos);
        }
    }

    // TODO
    // In actual movegen, this shouldn't iterate over every possible piece type at once.
    // But, since for now we're optimizing for perft, that's fine.
    pub fn for_white_promotion_push<F: FnMut(Node)>(&self, mut f: F) {
        let pawns = self
            .white_pawns()
            .intersect(WHITE_PROMOTER_MASK)
            .difference(self.occupancy_total.rshift(8));
        for (_, bb) in pawns {
            let bb_new = bb.lshift(8);
            let bb_move = bb.union(bb_new);
            // TODO look at asm difference between apply -> copy -> apply and copy -> apply -> apply
            // TODO orrrr we allow to mutate self, apply to self, and then afterwards undo
            for piece in [
                Piece::WhiteQueen,
                Piece::WhiteKnight,
                Piece::WhiteRook,
                Piece::WhiteBishop,
            ]
            .into_iter()
            {
                let mut pos = self.clone();
                pos.reset_en_passant();
                pos.piece_mut(Piece::WhitePawn).apply(bb);
                pos.occupancy_white.apply_move(bb_move);
                pos.occupancy_total.apply_move(bb_move);
                pos.piece_mut(piece).apply(bb);
                f(pos);
            }
        }
    }

    // The rim is a mask that defines what pieces cannot be attacked in that particular direction
    pub fn for_white_simple_attack<F: FnMut(Node)>(&self, rim: BitBoard, shift_bits: i8, mut f: F) {
        let victims = self.occupancy_black.difference(rim.union(BitBoard::R8));
        let attackers = self.white_pawns().intersect(victims.rshift(shift_bits));
        for (_, bb) in attackers {
            let mut pos = self.clone();
            let bb_new = bb.lshift(shift_bits);
            pos.reset_en_passant();
            pos.apply_capture(Piece::WhitePawn, bb, bb_new);
            f(pos)
        }
    }

    pub fn for_white_east_simple_attack<F: FnMut(Node)>(&self, f: F) {
        self.for_white_simple_attack(BitBoard::FA, 9, f);
    }

    pub fn for_white_west_simple_attack<F: FnMut(Node)>(&self, f: F) {
        self.for_white_simple_attack(BitBoard::FH, 7, f);
    }

    // The rim is a mask that defines what pieces cannot be attacked in that particular direction
    pub fn for_white_promotion_attack<F: FnMut(Node)>(
        &self,
        rim: BitBoard,
        shift_bits: i8,
        mut f: F,
    ) {
        let victims = self.occupancy_black.difference(rim.union(BitBoard::R8));
        let attackers = self.white_pawns().intersect(victims.rshift(shift_bits));
        for (_, bb) in attackers {
            let mut pos = self.clone();
            let bb_new = bb.lshift(shift_bits);
            pos.reset_en_passant();
            pos.capture_black(bb_new);
            pos.apply_capture(Piece::WhitePawn, bb, bb_new);
            f(pos)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::perft::node::Node;

    #[test]
    fn white_simple_push() {
        fn count_white_simple_pushes(node: Node) -> usize {
            let mut i = 0;
            node.for_white_simple_push(|pos| {
                pos.debug_validate_occupancies();
                i += 1
            });
            i
        }
        assert_eq!(count_white_simple_pushes(Node::POSITION_1), 8);
        assert_eq!(count_white_simple_pushes(Node::POSITION_2), 4);
        assert_eq!(count_white_simple_pushes(Node::POSITION_3), 3);
        assert_eq!(count_white_simple_pushes(Node::POSITION_4), 5);
        assert_eq!(count_white_simple_pushes(Node::POSITION_5), 5);
        assert_eq!(count_white_simple_pushes(Node::POSITION_6), 5);
    }

    #[test]
    fn white_double_push() {
        fn count_white_double_pushes(node: Node) -> usize {
            let mut i = 0;
            node.for_white_double_push(|pos| {
                pos.debug_validate_occupancies();
                i += 1
            });
            i
        }
        assert_eq!(count_white_double_pushes(Node::POSITION_1), 8);
        assert_eq!(count_white_double_pushes(Node::POSITION_2), 2);
        assert_eq!(count_white_double_pushes(Node::POSITION_3), 2);
        assert_eq!(count_white_double_pushes(Node::POSITION_4), 3);
        assert_eq!(count_white_double_pushes(Node::POSITION_5), 4);
        assert_eq!(count_white_double_pushes(Node::POSITION_6), 2);
    }

    #[test]
    fn white_promo_push() {
        fn count_white_promo_pushes(node: Node) -> usize {
            let mut i = 0;
            node.for_white_promotion_push(|pos| {
                pos.debug_validate_occupancies();
                i += 1
            });
            i
        }
        assert_eq!(count_white_promo_pushes(Node::POSITION_1), 0);
        assert_eq!(count_white_promo_pushes(Node::POSITION_2), 0);
        assert_eq!(count_white_promo_pushes(Node::POSITION_3), 0);
        assert_eq!(count_white_promo_pushes(Node::POSITION_4), 0);
        assert_eq!(count_white_promo_pushes(Node::POSITION_5), 0);
        assert_eq!(count_white_promo_pushes(Node::POSITION_6), 0);
    }

    #[test]
    fn white_attack_east() {
        fn count_white_east_simple_attack(node: Node) -> usize {
            let mut i = 0;
            node.for_white_east_simple_attack(|pos| {
                pos.debug_validate_occupancies();
                i += 1
            });
            i
        }
        assert_eq!(count_white_east_simple_attack(Node::POSITION_1), 0);
        assert_eq!(count_white_east_simple_attack(Node::POSITION_2), 2);
        assert_eq!(count_white_east_simple_attack(Node::POSITION_3), 0);
        assert_eq!(count_white_east_simple_attack(Node::POSITION_4), 0);
        assert_eq!(count_white_east_simple_attack(Node::POSITION_5), 0);
        assert_eq!(count_white_east_simple_attack(Node::POSITION_6), 0);
    }

    #[test]
    fn white_attack_west() {
        fn count_white_west_simple_attack(node: Node) -> usize {
            let mut i = 0;
            node.for_white_west_simple_attack(|pos| {
                pos.debug_validate_occupancies();
                i += 1
            });
            i
        }
        assert_eq!(count_white_west_simple_attack(Node::POSITION_1), 0);
        assert_eq!(count_white_west_simple_attack(Node::POSITION_2), 0);
        assert_eq!(count_white_west_simple_attack(Node::POSITION_3), 0);
        assert_eq!(count_white_west_simple_attack(Node::POSITION_4), 0);
        assert_eq!(count_white_west_simple_attack(Node::POSITION_5), 0);
        assert_eq!(count_white_west_simple_attack(Node::POSITION_6), 0);
    }
}
