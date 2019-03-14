extern crate failure;
extern crate hex;

fn xor_stream<'a>(input: &'a [u8], key: &'a [u8]) -> impl Iterator<Item = u8> + 'a {
    return input
        .iter()
        .zip(key.into_iter().cycle())
        .map(|(a, b)| a ^ b);
}

fn main() -> Result<(), failure::Error> {
    let key = "ICE";
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

    let output: Vec<u8> = xor_stream(input.as_bytes(), key.as_bytes()).collect();

    println!("{}", hex::encode(output));

    // println!("Hello, world!");

    return Ok(());
}
