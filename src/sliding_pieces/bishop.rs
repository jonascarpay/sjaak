use crate::{bitboard::BitBoard, coord::Square};

use super::magic_value::MagicValue;

const fn bishop_moves_ref(sq: Square, blockers: BitBoard) -> BitBoard {
    let mut bb = BitBoard::new();

    let mut northeast = sq.northeast();
    while let Some(sq) = northeast {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        northeast = sq.northeast();
    }

    let mut northwest = sq.northwest();
    while let Some(sq) = northwest {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        northwest = sq.northwest();
    }

    let mut southeast = sq.southeast();
    while let Some(sq) = southeast {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        southeast = sq.southeast();
    }

    let mut southwest = sq.southwest();
    while let Some(sq) = southwest {
        bb.set_assign(sq);
        if blockers.contains(sq) {
            break;
        }
        southwest = sq.southwest();
    }

    bb
}

pub const fn bishop_moves(sq: Square, blockers: BitBoard) -> BitBoard {
    let (offset, mask, magic) = BISHOP_TABLE_INDEX[sq.to_index() as usize];
    let index = offset + magic.to_index(mask.intersect(blockers), BISHOP_INDEX_BITS);
    BISHOP_TABLE[index]
}

const fn blocker_squares(sq: Square) -> BitBoard {
    bishop_moves_ref(sq, BitBoard::EMPTY).intersect(BitBoard::RIM.complement())
}

pub const BISHOP_INDEX_BITS: u8 = 9;

// TODO might be better to calculate sizes
// TODO static?
#[rustfmt::skip]
pub const BISHOP_MAGICS: [(usize, u64); 64] =
[         /* a */                   /* b */                   /* c */                   /* d */                   /* e */                   /* f */                   /* g */                   /* h */
  /* 1 */ ( 64,0x0801020010102040), ( 32,0x4081002008102100), ( 32,0x0101001020020100), ( 48,0x0000804810004000), ( 63,0x0040440220000240), ( 60,0x000021c100800200), ( 62,0xa000210042004440), (126,0x1200101201040040),
  /* 2 */ ( 32,0x1010004200204010), ( 32,0x0200010100200404), ( 32,0x8061010008200408), ( 48,0x4208008060040020), ( 63,0x4200404402200040), ( 60,0x02100021c1008008), ( 62,0x2400604100410040), ( 62,0x0002080821010020),
  /* 3 */ ( 32,0x8080900401001020), ( 32,0x0800880200101020), (128,0x8082020042004004), (128,0x0080218200801002), (138,0x0020250080840000), (128,0x0000080080840084), ( 32,0x2200050820210040), ( 32,0x2240040008404010),
  /* 4 */ ( 32,0x1000840000410020), ( 32,0x0401010d00801010), (128,0x0002004200808080), (512,0x002400c804010010), (512,0xa000840000822001), (128,0x0000108020404042), ( 32,0x0000200808208018), ( 32,0x4040200210084011),
  /* 5 */ ( 32,0x5000420205004040), ( 32,0x0000208030020020), (128,0x0100110080040008), (512,0x0080200800010106), (512,0x0040010600010048), (128,0x0700220010008040), ( 32,0x2000102008004040), ( 32,0x00a0104008001004),
  /* 6 */ ( 32,0x00002082004000a2), ( 32,0x0000402080400030), (128,0x0400804100400084), (133,0x0005048840404020), (128,0x2421020040108100), (128,0x4090002020200041), ( 32,0x0080404080204004), ( 32,0x5081002020028004),
  /* 7 */ ( 62,0x0050210042005008), ( 62,0x0040402080402800), ( 36,0x8000001010806220), ( 61,0x0080000008403000), ( 32,0x0200000100202000), ( 32,0x8022040080101020), ( 32,0x0201002080800a00), ( 32,0x0401001080080811),
  /* 8 */ (126,0x0010201044040040), ( 62,0x4010004020210020), ( 36,0x0010a01008208060), ( 61,0x0800020404084030), ( 32,0x0208080801002022), ( 32,0x0100008200202020), ( 32,0xc208010100101040), ( 64,0x0000420080200840),
];

static BISHOP_TABLE_INDEX: [(usize, BitBoard, MagicValue); 64] = {
    let mut table = [(0, BitBoard::EMPTY, MagicValue::new(0)); 64];
    let mut i = 0;
    let mut offset = 0;
    while i < 64 {
        table[i] = (
            offset,
            blocker_squares(Square::from_index(i as u8).unwrap()),
            MagicValue::new(BISHOP_MAGICS[i].1),
        );
        offset += BISHOP_MAGICS[i].0;
        i += 1;
    }
    table
};

const BISHOP_TABLE_SIZE: usize = {
    let mut total = 0;
    let mut i = 0;
    while i < 64 {
        total += BISHOP_MAGICS[i].0;
        i += 1;
    }
    total
};

#[allow(long_running_const_eval)]
static BISHOP_TABLE: [BitBoard; BISHOP_TABLE_SIZE] = {
    let mut table = [BitBoard::EMPTY; BISHOP_TABLE_SIZE];
    let mut i = 0;
    while i < 64 {
        let table_offset = BISHOP_TABLE_INDEX[i].0;
        let magic = MagicValue::new(BISHOP_MAGICS[i].1);
        let sq = Square::from_index(i as u8).unwrap();
        let mut blockers = blocker_squares(sq).powerset();
        while let Some(blockers) = blockers.pop() {
            let index = magic.to_index(blockers, BISHOP_INDEX_BITS);
            table[table_offset + index] = bishop_moves_ref(sq, blockers);
        }
        i += 1;
    }
    table
};

pub fn magic_lut_size(sq: Square, magic: MagicValue, max_size: usize) -> Option<usize> {
    let mut max_index = 0;
    let mut lut = vec![BitBoard::EMPTY; max_size];
    for blockers in blocker_squares(sq).powerset() {
        let moves = bishop_moves_ref(sq, blockers);
        let index = magic.to_index(blockers, 9);
        if index < max_size {
            let entry = &mut lut[index];
            if *entry == BitBoard::EMPTY {
                max_index = std::cmp::max(index, max_index);
                *entry = moves;
            } else if *entry != moves {
                return None;
            }
        } else {
            return None;
        }
    }
    Some(max_index + 1)
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        bitboard::BitBoard,
        coord::Square,
        sliding_pieces::bishop::{bishop_moves, bishop_moves_ref, blocker_squares},
    };

    #[test]
    fn unit() {
        assert_eq!(
            blocker_squares(Square::from_str("d3").unwrap()),
            BitBoard::from_bits(0x402214001400)
        );
        assert_eq!(
            blocker_squares(Square::from_str("e4").unwrap()),
            BitBoard::from_bits(0x2442800284400)
        );
    }

    #[quickcheck]
    fn rim_blockers_dont_matter(sq: Square, blockers: BitBoard) -> bool {
        bishop_moves_ref(sq, blockers)
            == bishop_moves_ref(sq, blockers.intersect(BitBoard::RIM.complement()))
    }

    #[quickcheck]
    fn bishop_moves_magic_matches_ref(sq: Square, blockers: BitBoard) -> bool {
        let blockers = blockers.unset(sq);
        bishop_moves_ref(sq, blockers) == bishop_moves(sq, blockers)
    }

    #[quickcheck]
    fn blocker_mask_size(sq: Square) -> bool {
        let l = blocker_squares(sq).len();
        l >= 5 && l <= 9
    }
}
