extern crate cryptopal;
extern crate failure;

use cryptopal::aes::{Block128, CBCDecrypter, CBCEncrypter};
use cryptopal::encoding;

fn main() -> Result<(), failure::Error> {
  let mut data = encoding::read_base64_file("10.txt")?;
  let ctext = data.clone();

  let key = Block128::clone_from_slice("YELLOW SUBMARINE".as_bytes());
  let iv: Block128 = [0; 16].into();

  let mut decrypter = CBCDecrypter::new(&key, &iv);
  decrypter.decrypt(&mut data);
  let ptext = String::from_utf8(data)?;
  println!("ptext: {:?}", ptext);

  let mut encrypter = CBCEncrypter::new(&key, &iv);
  data = ptext.as_bytes().to_owned();
  encrypter.encrypt(&mut data);

  println!("encrypt(text) == ctext => {}", data == ctext);

  return Ok(());
}
