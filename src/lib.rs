#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

extern crate generic_array;
extern crate typenum;

pub mod aes;
pub mod detect;
pub mod encoding;
pub mod english;
pub mod pkcs7;
pub mod single_char;

fn xor_bytes(buf: &mut [u8], key: &[u8]) {
  for (i, byte) in buf.iter_mut().enumerate() {
    *byte = *byte ^ key[i % key.len()];
  }
}
