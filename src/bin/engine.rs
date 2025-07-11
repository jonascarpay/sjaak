use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let mut log_file = File::create("/tmp/sjaak_engine.log")?;
    writeln!(log_file, "Startup complete")?;

    for line in io::stdin().lock().lines() {
        let line = line?;
        writeln!(log_file, "<< {}", line)?;
        eprintln!("<< {}", line);
        let mut parts = line.trim().split_whitespace();
        let command = parts.next();

        match command {
            Some("uci") => {
                println!("id name sjaak");
                println!("id author jmc");
                println!("uciok");
            }
            Some("isready") => {
                println!("readyok");
            }
            Some("ucinewgame") => {
                // We don't need to do anything here yet.
            }
            Some("position") => {
                // We don't need to do anything here yet.
            }
            Some("go") => {
                // We don't need to do anything here yet.
            }
            Some("stop") => {
                break;
            }
            _ => {}
        }
    }
    Ok(())
}
