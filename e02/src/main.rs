extern crate hex;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = hex::decode("1c0111001f010100061a024b53535009181c")?;
    let mask = hex::decode("686974207468652062756c6c277320657965")?;

    for i in 0..input.len() {
        input[i] = input[i] ^ mask[i];
    }

    let output = hex::encode(input);

    println!("{}", output);

    return Ok(());
}
