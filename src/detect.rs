use std::collections::HashSet;

use crate::aes::BLOCK_SIZE;

/// returns true if cipher text is likely encrypted in ECB mode
pub fn detect_ecb_ctext(ctext: &[u8]) -> bool {
  // find duplicate chunks
  let mut seen_chunks: HashSet<&[u8]> = HashSet::new();
  for chunk in ctext.chunks_exact(BLOCK_SIZE) {
    if seen_chunks.insert(chunk) == false {
      // had been inserted before
      return true;
    }
  }

  return false;
}
