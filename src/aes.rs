use aesni::block_cipher_trait::BlockCipher;
use aesni::Aes128;

use generic_array::GenericArray;
use typenum::U16;

use super::pkcs7;
use super::xor_bytes;

const BLOCK_SIZE: usize = 16;

pub type Block128 = GenericArray<u8, U16>;

pub struct ECBCipher {
  cipher: Aes128,
}

impl ECBCipher {
  pub fn new(key: &[u8]) -> ECBCipher {
    ECBCipher {
      cipher: Aes128::new(key.into()),
    }
  }

  pub fn decrypt(&self, data: &mut Vec<u8>) {
    for chunk in data.chunks_exact_mut(BLOCK_SIZE) {
      self.cipher.decrypt_block(Block128::from_mut_slice(chunk));
    }

    pkcs7::unpad_mut(data);
  }

  pub fn encrypt(&self, data: &mut Vec<u8>) {
    pkcs7::padding_mut(data, BLOCK_SIZE as u8);
    
    for chunk in data.chunks_exact_mut(BLOCK_SIZE) {
      self.cipher.encrypt_block(Block128::from_mut_slice(chunk));
    }
  }
}

pub struct CBCDecrypter {
  cipher: Aes128,

  // initially the iv
  mix: Block128,
}

impl CBCDecrypter {
  pub fn new(key: &Block128, iv: &Block128) -> CBCDecrypter {
    CBCDecrypter {
      cipher: Aes128::new(key),
      mix: iv.clone(),
    }
  }

  fn decrypt_block(&mut self, block: &mut Block128) {
    // mix = iv
    // ptext1 = aes_decrypt(ctext1)
    // ptext1 = ptext1 ^ mix
    //
    // mix = ctext1
    // ptext2 = aes_decrypt(ctext2)
    // ptext1 = ptext1 ^ ctext1
    //
    // ...

    let ctext = Block128::clone_from_slice(block);

    self.cipher.decrypt_block(block); // aes_decrypt(ctext1)
    xor_bytes(block, &self.mix); // ptext1 = ptext1 ^ mix

    self.mix.copy_from_slice(&ctext); // mix = ctext1
  }

  /// Decrypt AES cipher text inplace, then strip the PKCS7 padding
  pub fn decrypt(&mut self, data: &mut Vec<u8>) {
    for chunk in data.chunks_exact_mut(BLOCK_SIZE) {
      self.decrypt_block(chunk.into());
    }

    pkcs7::unpad_mut(data);
  }
}

pub struct CBCEncrypter {
  cipher: Aes128,
  // initially the iv
  mix: Block128,
}

impl CBCEncrypter {
  pub fn new(key: &Block128, iv: &Block128) -> CBCEncrypter {
    CBCEncrypter {
      cipher: Aes128::new(key),
      mix: iv.clone(),
    }
  }

  fn encrypt_block(&mut self, block: &mut [u8]) {
    // mix = iv
    // ctext1 = aes_encrypt(mix ^ ptext1)
    //
    // mix = ctext1
    // ctext2 = aes_encrypt(mix ^ ptext2)
    //
    // ...
    xor_bytes(block, &self.mix); // mix ^ ptext1
    self.cipher.encrypt_block(block.into()); // ctext1 = aes_encrypt(mix ^ ptext1)
    self.mix.copy_from_slice(block); // mix = ctext1
  }

  /// Decrypt AES cipher text inplace, then strip the PKCS7 padding
  pub fn encrypt(&mut self, data: &mut Vec<u8>) {
    pkcs7::padding_mut(data, BLOCK_SIZE as u8);

    for chunk in data.chunks_exact_mut(BLOCK_SIZE) {
      self.encrypt_block(chunk);
    }
  }
}
