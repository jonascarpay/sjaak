use crate::{bitboard::BitBoard, coord::Square, pieces::magic_value::MagicValue};

use super::reference::rook_moves_reference;

pub const fn rook_moves_magic(sq: Square, blockers: BitBoard) -> BitBoard {
    let (offset, mask, magic) = ROOK_TABLE_INDEX[sq.to_index() as usize];
    let index = offset + magic.to_index(mask.intersect(blockers), ROOK_INDEX_BITS);
    ROOK_TABLE[index]
}

pub fn rook_moves_magic_unsafe(sq: Square, blockers: BitBoard) -> BitBoard {
    let (offset, mask, magic) = unsafe { ROOK_TABLE_INDEX.get_unchecked(sq.to_index() as usize) };
    let index = offset + magic.to_index(mask.intersect(blockers), ROOK_INDEX_BITS);
    unsafe { *ROOK_TABLE.get_unchecked(index) }
}

const fn blocker_squares(sq: Square) -> BitBoard {
    const RANK_MASK: u64 = 0b0111_1110;
    const FILE_MASK: u64 = 0x00_01_01_01_01_01_01_00;
    let (x, y) = sq.to_xy();
    let rank_mask = RANK_MASK << (y << 3);
    let file_mask = FILE_MASK << x;
    BitBoard::from_bits((rank_mask | file_mask) & !sq.to_bitboard().to_bits())
}

pub const ROOK_INDEX_BITS: u8 = 12;

// TODO might be better to calculate sizes
// TODO static?
#[rustfmt::skip]
pub const ROOK_MAGICS: [(usize, u64); 64] =
[         /* a */                     /* b */                     /* c */                     /* d */                     /* e */                     /* f */                     /* g */                     /* h */
  /* 1 */ (4096, 0x80800292a0804000), (2048, 0x0020001000080020), (2048, 0x0040080010004005), (2048, 0x0040080004004002), (2048, 0x0040020004004001), (2056, 0x0020008020010202), (2048, 0x0040004000800100), (4096, 0x0900014126810012),
  /* 2 */ (2071, 0x8480081020410400), (1024, 0x0100100008040010), (1025, 0x8080080402010008), (1024, 0x0000200400200200), (1024, 0x0000200100020020), (1024, 0x2400200100200080), (1024, 0x0000400080004001), (2060, 0x0200200020004081),
  /* 3 */ (2048, 0x7c40002000100024), (1024, 0xa104001000080014), (1024, 0x3004000801020008), (1024, 0x0004002020020004), (1024, 0x0002002020010002), (1024, 0x1101002020008001), (1024, 0x0404004040008001), (2049, 0x1004802000400020),
  /* 4 */ (2050, 0x4840200010080010), (1024, 0x0000080010040012), (1024, 0x0484010008020008), (1024, 0x0020020020040020), (1024, 0x0020020020200100), (1024, 0x6010010020200080), (1024, 0x0030400040008001), (2060, 0x0822200020004081),
  /* 5 */ (2048, 0x8040001000200020), (1024, 0x4004000800100010), (1024, 0x0084020100080008), (1024, 0x0800200200200400), (1024, 0x0000200200200100), (1024, 0x0300200100200080), (1024, 0x0140008000404001), (2049, 0x8808802000200040),
  /* 6 */ (2051, 0x0040201004000802), (1025, 0x0810080201000400), (1210, 0x2000084040804200), (1024, 0x0000020004002020), (1024, 0x9902002001002002), (1024, 0xc000008001002020), (1024, 0x0208004000802020), (2175, 0x0082420088004204),
  /* 7 */ (2048, 0x0240001000200020), (1024, 0x0000080010040010), (1024, 0x8004010008020008), (1024, 0x0200200200040020), (1024, 0x9002001001008010), (1024, 0x2008200080010020), (1024, 0x0004200040008020), (2049, 0x0000802000400020),
  /* 8 */ (4096, 0x000041048000e131), (3840, 0x0000800900102041), (3968, 0x00a8088010200442), (3840, 0x6030080420401002), (3840, 0x4004042008100102), (3832, 0x3480040802048001), (3832, 0xa024024002040081), (4096, 0x2020008908e40042),
];

static ROOK_TABLE_INDEX: [(usize, BitBoard, MagicValue); 64] = {
    let mut table = [(0, BitBoard::EMPTY, MagicValue::new(0)); 64];
    let mut i = 0;
    let mut offset = 0;
    while i < 64 {
        table[i] = (
            offset,
            blocker_squares(Square::from_index(i as u8).unwrap()),
            MagicValue::new(ROOK_MAGICS[i].1),
        );
        offset += ROOK_MAGICS[i].0;
        i += 1;
    }
    table
};

const ROOK_TABLE_SIZE: usize = {
    let mut total = 0;
    let mut i = 0;
    while i < 64 {
        total += ROOK_MAGICS[i].0;
        i += 1;
    }
    total
};

#[allow(long_running_const_eval)]
static ROOK_TABLE: [BitBoard; ROOK_TABLE_SIZE] = {
    let mut table = [BitBoard::EMPTY; ROOK_TABLE_SIZE];
    let mut i = 0;
    while i < 64 {
        let table_offset = ROOK_TABLE_INDEX[i].0;
        let magic = MagicValue::new(ROOK_MAGICS[i].1);
        let sq = Square::from_index(i as u8).unwrap();
        let mut blockers = blocker_squares(sq).powerset();
        while let Some(blockers) = blockers.pop() {
            let index = magic.to_index(blockers, ROOK_INDEX_BITS);
            table[table_offset + index] = rook_moves_reference(sq, blockers);
        }
        i += 1;
    }
    table
};

pub fn magic_lut_size(sq: Square, magic: MagicValue, max_size: usize) -> Option<usize> {
    let mut max_index = 0;
    let mut lut = vec![BitBoard::EMPTY; max_size];
    for blockers in blocker_squares(sq).powerset() {
        let moves = rook_moves_reference(sq, blockers);
        let index = magic.to_index(blockers, ROOK_INDEX_BITS);
        if index < max_size {
            let entry = &mut lut[index];
            if entry.is_empty() {
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

// TODO Probably drop, now that we have a fixed amount
pub const fn index_bits(sq: Square) -> u8 {
    const FILE_RIM: BitBoard = BitBoard::FA.union(BitBoard::FH);
    const RANK_RIM: BitBoard = BitBoard::R1.union(BitBoard::R8);
    10 + (FILE_RIM.contains(sq) as u8) + (RANK_RIM.contains(sq) as u8)
}

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    use crate::{
        bitboard::BitBoard,
        coord::Square,
        pieces::rook::{
            magic::{blocker_squares, index_bits, rook_moves_magic, rook_moves_magic_unsafe},
            reference::rook_moves_reference,
        },
    };

    #[test]
    fn unit() {
        assert_eq!(
            blocker_squares(Square::from_str("a1").unwrap()),
            BitBoard::from_bits(0x101010101017e)
        );
        assert_eq!(
            blocker_squares(Square::from_str("d4").unwrap()),
            BitBoard::from_bits(0x8080876080800)
        );
    }

    #[quickcheck]
    fn blocker_mask_size(sq: Square) -> bool {
        let l = blocker_squares(sq).len();
        l >= 10 && l <= 12
    }

    #[quickcheck]
    fn blocker_squares_bits(sq: Square) -> bool {
        blocker_squares(sq).len() == index_bits(sq) as usize
    }

    #[quickcheck]
    fn rook_moves_magic_matches_ref(sq: Square, blockers: BitBoard) -> bool {
        let blockers = blockers.unset(sq);
        rook_moves_reference(sq, blockers) == rook_moves_magic(sq, blockers)
    }

    #[quickcheck]
    fn magic_safe_is_unsafe(sq: Square, blockers: BitBoard) -> bool {
        let blockers = blockers.unset(sq);
        rook_moves_magic(sq, blockers) == rook_moves_magic_unsafe(sq, blockers)
    }
}
