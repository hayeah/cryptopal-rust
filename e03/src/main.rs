extern crate cryptopal;

use cryptopal::single_char::{Cracker, DecodeResult};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let mut results: Vec<DecodeResult> = Cracker::new(input).collect();

    results.sort_by(|a, b| b.partial_cmp(&a).unwrap());

    for r in results {
        println!("{}", r);
    }

    return Ok(());
}
