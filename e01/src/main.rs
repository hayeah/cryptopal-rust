extern crate base64;
extern crate hex;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let bytes = hex::decode(input)?;

    let output = base64::encode(&bytes);

    println!("{}", output);

    return Ok(());
}
