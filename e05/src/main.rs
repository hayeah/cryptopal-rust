extern crate failure;
extern crate hex;

fn xor_stream<'a>(
    input: impl Iterator<Item = u8> + 'a,
    key: &'a [u8],
) -> impl Iterator<Item = u8> + 'a {
    return input.zip(key.into_iter().cycle()).map(|(a, b)| a ^ b);
}

// fn xor_stream_boxed(
//     input: Box<IntoIterator<Item = u8>>,
//     key: Vec<u8>,
// ) -> Box<dyn Iterator<Item = u8>> {
//     return Box::new(input.zip(key.into_iter().cycle()).map(|(a, b)| a ^ b));
// }

// fn xor_stream_boxed(
//     input: Box<dyn IntoIterator<IntoIter = Iterator<Item = u8>, Item = u8>>,
//     key: Vec<u8>,
// ) -> Box<dyn Iterator<Item = u8>> {
//     return Box::new(
//         input
//             .into_iter()
//             .zip(key.into_iter().cycle())
//             .map(|(a, b)| a ^ b),
//     );
// }

fn main() -> Result<(), failure::Error> {
    let key = "ICE";
    let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";

    let output: Vec<u8> = xor_stream(input.bytes(), key.as_bytes()).collect();

    // let output2 = xor_stream_boxed(Box::new(inputBoxed.bytes()), key.as_bytes().to_vec());

    println!("{}", hex::encode(output));

    return Ok(());
}
