extern crate cryptopal;
extern crate rand;

use cryptopal::aes::{Block128, CBCEncrypter, ECBCipher, BLOCK_SIZE};
use cryptopal::detect::detect_ecb_ctext;
use rand::prelude::*;

#[derive(Debug)]
enum EncryptMode {
  ECB,
  CBC,
}

struct MysteryEncrypter {
  key: Block128,
  ecb: ECBCipher,
}

impl MysteryEncrypter {
  fn new() -> MysteryEncrypter {
    let mut key = [0u8; BLOCK_SIZE];

    rand::thread_rng().fill_bytes(&mut key);

    return MysteryEncrypter {
      key: Block128::clone_from_slice(&key),
      ecb: ECBCipher::new(&key),
    };
  }

  fn encrypt_cbc(&self, data: &mut Vec<u8>) {
    let mut iv = [0u8; BLOCK_SIZE];

    rand::thread_rng().fill_bytes(&mut iv);
    let mut encrypter = CBCEncrypter::new(&self.key.into(), &iv.into());
    encrypter.encrypt(data);
  }

  /// encrypt an input by randomly choosing cbc or ecb mode. Pads input with
  /// random prefix ans suffix.
  fn encrypt(&self, input: &[u8]) -> (Vec<u8>, EncryptMode) {
    let mut buf = [0u8; BLOCK_SIZE];
    rand::thread_rng().fill_bytes(&mut buf);

    // wrap input data with random prefix and suffix
    let prefix_size: usize = rand::thread_rng().gen_range(6, 10);
    let prefix = &buf[0..prefix_size];
    let suffix = &buf[prefix_size..];

    let mut data: Vec<u8> = Vec::with_capacity(input.len() + buf.len());
    data.extend_from_slice(prefix);
    data.extend_from_slice(input);
    data.extend_from_slice(suffix);

    let mode: EncryptMode;
    if random() {
      mode = EncryptMode::ECB;
      self.ecb.encrypt(&mut data);
    } else {
      mode = EncryptMode::CBC;
      self.encrypt_cbc(&mut data);
    }

    return (data, mode);
  }
}

fn trial(encrypter: &MysteryEncrypter) {
  // generate an input that's multiple of block size. If we find duplicate block
  // in the cipher text, then it's probably encrypted ECB mode.
  let buf = [0u8; BLOCK_SIZE * 32];
  let (ctext, mode) = encrypter.encrypt(&buf);

  let is_ecb = detect_ecb_ctext(&ctext);

  println!("actual-mode={:?} detected-ecb={}", mode, is_ecb);
}

fn main() {
  let encrypter = MysteryEncrypter::new();

  for _ in 0..10 {
    trial(&encrypter);
  }
}
