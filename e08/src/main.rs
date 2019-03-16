extern crate cryptopal;
extern crate failure;

use cryptopal::encoding::HexLinesDecoder;
use std::collections::HashSet;

const BLOCK_SIZE: usize = 16;

fn detect_ecb_encoding(data: &[u8]) -> bool {
  // find duplicate chunks
  let mut seen_chunks: HashSet<&[u8]> = HashSet::new();
  for chunk in data.chunks_exact(BLOCK_SIZE) {
    if seen_chunks.insert(chunk) == false {
      // had been inserted before
      return true;
    }
  }

  return false;
}

fn main() -> Result<(), failure::Error> {
  let decoder = HexLinesDecoder::file("8.txt")?;

  for (i, item) in decoder.iter().enumerate() {
    if detect_ecb_encoding(&item?) {
      println!("detected ECB: line={}", i);
    }
  }

  return Ok(());
}
