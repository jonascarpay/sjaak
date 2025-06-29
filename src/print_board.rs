use crate::coord::{File, Rank, Square};
use std::io::Write;

// https://en.wikipedia.org/wiki/Chess_symbols_in_Unicode

pub fn format_board_fancy<F: Fn(Square) -> char>(get_char: F) -> std::io::Result<String> {
    // If the buffer is filled with ASCII characters, a buffer size of 709
    // will always contain exactly the entire board. But, unicode chess pieces mess with this, so
    // let's forego the optimization and pick a safe 1024.
    let mut output: Vec<u8> = Vec::with_capacity(1024);

    writeln!(output, "  ╔════════════════════════╗")?;
    for rank in Rank::ALL.into_iter().rev() {
        write!(output, "{} ║", rank.to_char())?;
        for file in File::ALL.into_iter() {
            let sq = Square::from_coord(file, rank);
            let char = get_char(sq);
            if sq.is_light() {
                write!(output, "\x1b[7m {} \x1b[0m", char)?;
            } else {
                write!(output, " {} ", char)?;
            }
        }
        writeln!(output, "║")?;
    }
    writeln!(output, "  ╚════════════════════════╝")?;
    writeln!(output, "    a  b  c  d  e  f  g  h")?;

    Ok(String::from_utf8(output).unwrap())
}
