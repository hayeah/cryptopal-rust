#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate failure;

extern crate typenum;
extern crate generic_array;

pub mod aes;
pub mod encoding;
pub mod english;
pub mod pkcs7;
pub mod single_char;

fn xor_bytes(buf: &mut [u8], key: &[u8]) {
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte = *byte ^ key[i % key.len()];
    }
}
