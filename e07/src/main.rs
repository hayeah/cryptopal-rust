extern crate cryptopal;
extern crate failure;

use cryptopal::aes;

fn main() -> Result<(), failure::Error> {
  let mut data = cryptopal::encoding::read_base64_file("7.txt")?;
  let key = "YELLOW SUBMARINE".as_bytes();

  aes::decrypt_ecb(&mut data, key.into());

  println!("{}", String::from_utf8(data)?);

  return Ok(());
}
