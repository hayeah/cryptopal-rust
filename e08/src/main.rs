extern crate cryptopal;
extern crate failure;

use cryptopal::detect::detect_ecb_ctext;
use cryptopal::encoding::HexLinesDecoder;

fn main() -> Result<(), failure::Error> {
  let decoder = HexLinesDecoder::file("8.txt")?;

  for (i, item) in decoder.iter().enumerate() {
    if detect_ecb_ctext(&item?) {
      println!("detected ECB: line={}", i);
    }
  }

  return Ok(());
}
