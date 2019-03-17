extern crate cryptopal;
extern crate failure;

use cryptopal::aes::ECBCipher;

fn main() -> Result<(), failure::Error> {
  let mut data = cryptopal::encoding::read_base64_file("7.txt")?;
  let ctext = data.clone();

  let key = "YELLOW SUBMARINE".as_bytes();

  let ecb = ECBCipher::new(&key);
  ecb.decrypt(&mut data);
  let ptext = String::from_utf8(data)?;
  println!("{:?}", ptext);

  // re-encrypt, and compare to original data
  data = ptext.as_bytes().to_owned();
  ecb.encrypt(&mut data);
  assert_eq!(data, ctext);

  return Ok(());
}
