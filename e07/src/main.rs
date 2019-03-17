extern crate cryptopal;
extern crate failure;

use cryptopal::aes::ECBCipher;

fn main() -> Result<(), failure::Error> {
  let mut data = cryptopal::encoding::read_base64_file("7.txt")?;
  let key = "YELLOW SUBMARINE".as_bytes();

  let ecb = ECBCipher::new(&key);
  ecb.decrypt(&mut data);
  println!("{:?}", String::from_utf8(data)?);

  return Ok(());
}
