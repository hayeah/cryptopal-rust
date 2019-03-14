use std::fmt;

use crate::english;

pub fn xor(dst: &mut [u8], src: &[u8], mask: u8) {
  for i in 0..src.len() {
    dst[i] = src[i] ^ mask;
  }
}

#[derive(PartialEq, PartialOrd, Default, Clone)]
pub struct DecodeResult {
  score: f64,
  key: u8,
  ptext: String,
}

impl fmt::Display for DecodeResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "key={} score={} ptext={:?}",
      self.key, self.score, self.ptext
    )
  }
}

// SingleCharKeyCracker iterates through the ASCII bytes, as key to a xor cipher
pub struct SingleCharKeyCracker {
  ctext: Vec<u8>,
  current_byte: usize,

  decode_buf: Option<Vec<u8>>,
}

impl SingleCharKeyCracker {
  pub fn new(ctext: Vec<u8>) -> SingleCharKeyCracker {
    return SingleCharKeyCracker {
      ctext: ctext,
      current_byte: 0,
      decode_buf: None,
    };
  }
}

impl Iterator for SingleCharKeyCracker {
  type Item = DecodeResult;

  fn next(&mut self) -> Option<DecodeResult> {
    while self.current_byte <= 255 {
      let key = self.current_byte as u8;

      let mut buf = self
        .decode_buf
        .take()
        .unwrap_or_else(|| vec![0; self.ctext.len()]);

      xor(&mut buf, &self.ctext, key);

      match String::from_utf8(buf) {
        Err(err) => {
          // Reuse the buffer that failed to parse into an UTF8 string
          self.decode_buf = Some(err.into_bytes());
          self.current_byte += 1;
          continue; // Try the next key
        }
        Ok(s) => {
          let score = english::nonfit_score(key as usize, &s);

          self.current_byte += 1;

          return Some(DecodeResult {
            key: key,
            ptext: s,
            score: score,
          });
        }
      }
    }

    return None;
  }
}
