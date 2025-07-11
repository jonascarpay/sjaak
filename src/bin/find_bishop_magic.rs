use shah::{
    coord::{File, Square},
    pieces::{
        bishop::magic::{magic_lut_size, BISHOP_INDEX_BITS, BISHOP_MAGICS},
        magic_value::MagicValue,
    },
};
use std::{sync::Mutex, thread};
use tinyrand::Rand;

fn main() {
    let table: Mutex<[(usize, u64); 64]> = Mutex::new(BISHOP_MAGICS);

    thread::scope(|s| {
        for _ in 0..num_cpus::get() {
            s.spawn(|| {
                let mut rng = tinyrand_std::thread_rand();

                loop {
                    let index = (rng.next_u64() % 64) as usize;
                    let best = { table.lock().unwrap()[index].0 };
                    let sq = Square::from_index(index as u8).unwrap();
                    // let bits = index_bits(sq);
                    // if best == (1 << index_bits(sq)) {
                    //     continue;
                    // }
                    for _ in 0..0xFFFF {
                        let magic = MagicValue::random(&mut || rng.next_u64(), BISHOP_INDEX_BITS);
                        if let Some(new_best) = magic_lut_size(sq, magic, best) {
                            let mut table = table.lock().unwrap();
                            let prev_best = table[index].0;
                            if new_best < prev_best {
                                println!(
                                    "----- {}: {} (-{}) -----",
                                    sq,
                                    new_best,
                                    prev_best - new_best
                                );
                                table[index] = (new_best, magic.to_u64());
                                let mut total_size = 0;
                                print!("[        ");
                                for file in File::ALL {
                                    print!(" /* {} */                    ", file.to_char());
                                }
                                println!();
                                for y in 0..8 {
                                    print!("  /* {} */", y + 1);
                                    for x in 0..8 {
                                        let i = y << 3 | x;
                                        let (size, magic) = table[i];

                                        print!(" ({:4},0x{:016x}),", size, magic);
                                        total_size += size;
                                    }
                                    println!();
                                }
                                println!("]");
                                println!("{} B", total_size * 8);
                            }
                            break;
                        }
                    }
                }
            });
        }
    });
}
