extern crate cryptopal;

use cryptopal::pkcs7;

fn main() {
  let v = vec![0, 0, 0, 0];
  println!("data={:?} padded-data={:?}", v, pkcs7::padding(&v, 8));
}
