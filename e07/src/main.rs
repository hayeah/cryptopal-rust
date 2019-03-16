extern crate cryptopal;

extern crate failure;

use aesni::block_cipher_trait::BlockCipher;
use aesni::Aes128;

const BLOCK_SIZE: usize = 16;

fn decrypt_aes_ecb_block(data: &mut [u8], key: &[u8]) {
  // The From conversion from key to GenericArray will check for length
  let cipher = Aes128::new(key.into());

  let block = &mut data[0..BLOCK_SIZE];

  // can i enforce data to be an array?

  cipher.decrypt_block(block.into());
}

fn decrypt_aes_ecb(data: &mut [u8], key: &[u8]) {
  let mut buf = [0 as u8; BLOCK_SIZE];

  let mut i = 0;
  while i < data.len() {
    let end = i + BLOCK_SIZE;
    let block: &mut [u8];

    if end > data.len() {
      // If tail block, pad with 0s to make it 16 bytes
      buf.copy_from_slice(&data[i..]);
      block = &mut buf;
    } else {
      block = &mut data[i..i + BLOCK_SIZE];
    }

    decrypt_aes_ecb_block(block, key);

    i += BLOCK_SIZE;
  }
}

fn main() -> Result<(), failure::Error> {
  let mut data = cryptopal::encoding::read_base64_file("7.txt")?;
  let key = "YELLOW SUBMARINE".as_bytes();

  decrypt_aes_ecb(&mut data, key);

  println!("{}", String::from_utf8(data)?);

  return Ok(());
}
