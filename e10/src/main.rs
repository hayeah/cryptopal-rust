extern crate cryptopal;
extern crate failure;

use cryptopal::encoding;
use cryptopal::aes::{Block128, CBCDecrypter};


fn main() -> Result<(), failure::Error> {
    let mut data = encoding::read_base64_file("10.txt")?;

    // println!("{:?} mod16={}", data, data.len() % 16);

    let key = Block128::clone_from_slice("YELLOW SUBMARINE".as_bytes());
    // let iv = Block128::clone_from_slice(&vec![0;16]);
    let iv: Block128 = [0;16].into();
    let mut decrypter = CBCDecrypter::new(key, iv);
    decrypter.decrypt(&mut data);
    let ptext = String::from_utf8(data)?;
    println!("ptext: {:?}", ptext);

    return Ok(());
}
