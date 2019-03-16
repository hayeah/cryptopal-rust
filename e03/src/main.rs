extern crate cryptopal;

use cryptopal::single_char::Cracker;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let mut cracker = Cracker::new(&input);
    let results = cracker.best_results();

    // Print top 10 results
    for r in &results[0..3] {
        println!("{}", r);
    }

    return Ok(());
}
