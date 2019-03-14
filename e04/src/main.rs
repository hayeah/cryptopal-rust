extern crate cryptopal;
extern crate hex;

use cryptopal::single_char::Cracker;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("4.txt")?;

    let r = BufReader::new(f);

    for (i, line) in r.lines().enumerate() {
        let ctext = hex::decode(line?)?;
        let mut cracker = Cracker::new(ctext);

        if let Some(result) = cracker.best_result() {
            if result.score < 1000.0 {
                println!("bingo: line={} {}", i, result);
            }
        }
    }

    return Ok(());
}
