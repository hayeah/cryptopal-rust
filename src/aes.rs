use aesni::block_cipher_trait::BlockCipher;
use aesni::Aes128;

use generic_array::GenericArray;
use typenum::U16;

use super::xor_bytes;
use super::pkcs7;

const BLOCK_SIZE: usize = 16;

fn decrypt_single_block(data: &mut [u8], key: &[u8]) {
  // The From conversion from key to GenericArray will check for length
  let cipher = Aes128::new(key.into());

  let block = &mut data[0..BLOCK_SIZE];

  // can i enforce data to be an array?

  cipher.decrypt_block(block.into());
}

// Decrypt a buffer in place, ECB mode
pub fn decrypt_ecb(data: &mut [u8], key: &[u8]) {
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

    decrypt_single_block(block, key);

    i += BLOCK_SIZE;
  }
}

pub type Block128 = GenericArray<u8, U16>;

// type AES128Block = [u8; BLOCK_SIZE];

pub struct CBCDecrypter {
  key: Block128,

  // initially the iv
  mix: Block128,
}

impl CBCDecrypter {
  pub fn new(key: Block128, iv: Block128) -> CBCDecrypter {
    CBCDecrypter { key: key, mix: iv }
  }

  pub fn decrypt_block(&mut self, block: &mut [u8]) {
    // mix = iv
    // ptext1 = aes_decrypt(ctext1, key)
    // ptext1 = ptext1 ^ mix
    //
    // mix = ctext1
    // ptext2 = aes_decrypt(ctext2, key)
    // ptext1 = ptext1 ^ ctext1
    //
    // ...

    let ctext = Block128::clone_from_slice(block);

    decrypt_single_block(block, &self.key); // aes_decrypt(ctext1, key)
    xor_bytes(block, &self.mix); // ptext1 = ptext1 ^ mix

    self.mix.copy_from_slice(&ctext); // mix = ctext1
  }

  /// Decrypt AES cipher text inplace, then strip the PKCS7 padding
  pub fn decrypt(&mut self, data: &mut Vec<u8>) {
    for chunk in data.chunks_exact_mut(BLOCK_SIZE) {
      self.decrypt_block(chunk);
    }

    pkcs7::unpad_mut(data);
  }
}
