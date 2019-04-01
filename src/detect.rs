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

pub fn detect_ecb_blocksize<F>(encrypt: &mut F) -> Option<usize>
where
  F: FnMut(&mut [u8]),
{
  // generate 16 blocks of the same size, and same content, then check if all
  // the cipher blocks are the same.

  // We check block sizes from 1-64
  let mut data = [0u8; 64 * 16];
  // let mut data = vec![0u8; 64 * 16];

  for i in 1..=32 {
    let blocksize = i as usize;

    // Take 16 blocks
    let buf = &mut data[0..blocksize * 16];

    encrypt(buf);

    // Check for repeats
    let mut blocks = buf.chunks(blocksize);
    let block1 = blocks.next().unwrap();

    // Check that all blocks are the same, to weed out flukes.
    let samesame = blocks.all(|block| block1 == block);

    if samesame {
      return Some(blocksize);
    }

    // zero buf, to try next blocksize
    buf.iter_mut().for_each(|byte| *byte = 0)
  }

  return None;
}
