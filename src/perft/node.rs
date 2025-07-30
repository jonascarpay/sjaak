use crate::{
    bitboard::BitBoard,
    castling_rights::CastlingRights,
    coord::Square,
    piece::{Piece, PieceType, Side},
    pieces::{
        bishop::bishop_moves, king::king_moves, knight::knight_moves, pawn::PawnAttacks,
        rook::rook_moves,
    },
    position::Position,
    zobrist_table::ZOBRIST_TABLE,
};

#[derive(Clone)]
pub struct Node {
    pub pieces: [BitBoard; 12], // TODO probably just unroll this
    pub side: Side,
    pub occupancy_white: BitBoard,
    pub occupancy_black: BitBoard,
    pub occupancy_total: BitBoard,
    pub castling_rights: CastlingRights,
    // TODO For now, A1 == 0 means no en passant possible. A little ugly, but if it turns out to
    // work I'll wrap it up in a cleaner API.
    pub en_passant_square: Square,
}

impl Node {
    pub const fn piece(&self, piece: Piece) -> BitBoard {
        self.pieces[piece as usize]
    }
    pub const fn piece_mut(&mut self, piece: Piece) -> &mut BitBoard {
        &mut self.pieces[piece as usize]
    }
    pub const fn occupancy(&self, side: Side) -> BitBoard {
        match side {
            Side::White => self.occupancy_white,
            Side::Black => self.occupancy_black,
        }
    }
    fn hash(&self) -> u64 {
        let mut hash = 0;
        for pc_ix in 0..Piece::NUM_PIECES {
            for (sq, _) in self.pieces[pc_ix as usize].iter() {
                hash ^= ZOBRIST_TABLE.hash_piece(Piece::from_index(pc_ix).unwrap(), sq);
            }
        }
        hash ^= ZOBRIST_TABLE.hash_en_passant_square(self.en_passant_square);
        hash ^= ZOBRIST_TABLE.hash_castling_rights(&self.castling_rights);
        hash ^= ZOBRIST_TABLE.hash_side(self.side);
        hash
    }
    pub const fn from_position(pos: &Position) -> Node {
        let mut pieces = [BitBoard::EMPTY; 12];
        let mut occupancy_white = BitBoard::EMPTY;
        let mut occupancy_black = BitBoard::EMPTY;
        let mut i = 0;
        while i < 64 {
            let sq = Square::from_index(i).unwrap();
            if let Some(piece) = pos.get_piece(sq) {
                pieces[piece as usize].set_assign(sq);
                match piece.side() {
                    Side::White => occupancy_white.set_assign(sq),
                    Side::Black => occupancy_black.set_assign(sq),
                }
            }
            i += 1;
        }
        Node {
            pieces,
            side: pos.side,
            occupancy_white,
            occupancy_black,
            occupancy_total: occupancy_black.union(occupancy_white),
            castling_rights: pos.castling_rights,
            en_passant_square: match pos.en_passant_square {
                Some(sq) => sq,
                None => Square::from_index(0).unwrap(),
            },
        }
    }
    pub fn square_is_attacked_by(&self, side: Side, sq: Square, bb: BitBoard) -> bool {
        debug_assert_eq!(sq.to_bitboard(), bb);

        if knight_moves(sq).intersects(self.piece(Piece::from_side_piece(side, PieceType::Knight)))
        {
            return true;
        }

        // TODO remove the pawn attacks module
        if self.pawn_attacks(side).threat().intersects(bb) {
            return true;
        }

        let bishops: BitBoard = self.piece(Piece::from_side_piece(side, PieceType::Bishop));
        let queens: BitBoard = self.piece(Piece::from_side_piece(side, PieceType::Queen));
        if bishop_moves(sq, self.occupancy_total).intersects(bishops.union(queens)) {
            return true;
        }

        let rooks: BitBoard = self.piece(Piece::from_side_piece(side, PieceType::Rook));
        if rook_moves(sq, self.occupancy_total).intersects(rooks.union(queens)) {
            return true;
        }

        if king_moves(sq).intersects(self.piece(Piece::from_side_piece(side, PieceType::King))) {
            return true;
        }

        false
    }

    pub fn white_king_attacked(&self) -> bool {
        let bb = self.piece(Piece::WhiteKing);
        debug_assert!(!bb.is_empty());
        // TODO Obviously not ideal. Maybe have a nonempty bitboard, that would optimize away with
        // unwrap. Big change though.
        let sq = unsafe { bb.get_square().unwrap_unchecked() };
        self.square_is_attacked_by(Side::Black, sq, bb)
    }

    pub fn black_king_attacked(&self) -> bool {
        let bb = self.piece(Piece::BlackKing);
        debug_assert!(!bb.is_empty());
        // TODO Obviously not ideal. Maybe have a nonempty bitboard, that would optimize away with
        // unwrap. Big change though.
        let sq = unsafe { bb.get_square().unwrap_unchecked() };
        self.square_is_attacked_by(Side::White, sq, bb)
    }
    // TODO remove
    fn pawn_attacks(&self, side: Side) -> PawnAttacks {
        match side {
            Side::White => PawnAttacks::white(self.piece(Piece::WhitePawn), self.occupancy_black),
            Side::Black => PawnAttacks::black(self.piece(Piece::BlackPawn), self.occupancy_white),
        }
    }

    pub fn debug_validate_occupancies(&self) {
        let mut white = BitBoard::EMPTY;
        for piece in Piece::WHITE_PIECES {
            let bb = self.piece(piece);
            debug_assert!(!white.intersects(bb));
            white.apply(bb);
        }
        let mut black = BitBoard::EMPTY;
        for piece in Piece::BLACK_PIECES {
            let bb = self.piece(piece);
            debug_assert!(!black.intersects(bb));
            black.apply(bb);
        }
        debug_assert!(!white.intersects(black));
        debug_assert_eq!(white, self.occupancy_white);
        debug_assert_eq!(black, self.occupancy_black);
        debug_assert_eq!(white.union(black), self.occupancy_total);
    }

    // Does NOT update the total occupancy
    #[inline]
    pub fn capture_black(&mut self, sq_bb: BitBoard) {
        let dst_mask = sq_bb.complement();
        self.occupancy_black.apply_mask(dst_mask);
        self.piece_mut(Piece::BlackPawn).apply_mask(dst_mask);
        self.piece_mut(Piece::BlackKnight).apply_mask(dst_mask);
        self.piece_mut(Piece::BlackBishop).apply_mask(dst_mask);
        self.piece_mut(Piece::BlackRook).apply_mask(dst_mask);
        self.piece_mut(Piece::BlackQueen).apply_mask(dst_mask);
        self.piece_mut(Piece::BlackKing).apply_mask(dst_mask); // TODO is this necessary??
    }

    // Does NOT update the total occupancy
    #[inline]
    pub fn capture_white(&mut self, sq_bb: BitBoard) {
        let dst_mask = sq_bb.complement();
        self.occupancy_white.apply_mask(dst_mask);
        self.piece_mut(Piece::WhitePawn).apply_mask(dst_mask);
        self.piece_mut(Piece::WhiteKnight).apply_mask(dst_mask);
        self.piece_mut(Piece::WhiteBishop).apply_mask(dst_mask);
        self.piece_mut(Piece::WhiteRook).apply_mask(dst_mask);
        self.piece_mut(Piece::WhiteQueen).apply_mask(dst_mask);
        self.piece_mut(Piece::WhiteKing).apply_mask(dst_mask); // TODO is this necessary??
    }

    #[inline]
    pub fn apply_capture(&mut self, piece: Piece, from: BitBoard, to: BitBoard) {
        let move_bb = from.union(to);

        debug_assert!(self.occupancy_total.is_supserset_of(move_bb)); // Contains pieces at src and dst

        self.occupancy_total.apply(from);
        self.pieces[piece.to_index() as usize].apply_move(move_bb);

        match piece.side() {
            Side::White => {
                debug_assert!(self.occupancy_white.intersects(from));
                debug_assert!(self.occupancy_black.intersects(to));
                self.occupancy_white.apply_move(move_bb);
                self.capture_black(to);
            }
            Side::Black => {
                debug_assert!(self.occupancy_black.intersects(from));
                debug_assert!(self.occupancy_white.intersects(to));
                self.occupancy_black.apply_move(move_bb);
                self.capture_white(to);
            }
        }
    }

    #[inline]
    pub fn apply_move(&mut self, piece: Piece, move_bb: BitBoard) {
        self.pieces[piece.to_index() as usize].apply_move(move_bb);
        self.occupancy_total.apply_move(move_bb);
        match piece.side() {
            Side::White => self.occupancy_white.apply_move(move_bb),
            Side::Black => self.occupancy_black.apply_move(move_bb),
        }
    }
    pub fn reset_en_passant(&mut self) {
        self.en_passant_square = Square::A1;
    }
}

impl Position {
    pub const fn to_bitboard(&self) -> Node {
        Node::from_position(self)
    }
}

#[cfg(test)]
impl Node {
    pub const POSITION_1: Node = Position::POSITION_1.to_bitboard();
    pub const POSITION_2: Node = Position::POSITION_2.to_bitboard();
    pub const POSITION_3: Node = Position::POSITION_3.to_bitboard();
    pub const POSITION_4: Node = Position::POSITION_4.to_bitboard();
    pub const POSITION_5: Node = Position::POSITION_5.to_bitboard();
    pub const POSITION_6: Node = Position::POSITION_6.to_bitboard();
}
