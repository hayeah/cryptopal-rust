use std::fmt;

use crate::english;

pub fn xor(dst: &mut [u8], src: &[u8], mask: u8) {
  for i in 0..src.len() {
    dst[i] = src[i] ^ mask;
  }
}

#[derive(PartialEq, PartialOrd, Default, Clone)]
pub struct DecodeResult {
  pub score: f64,
  pub key: u8,
  pub ptext: String,
}

impl Eq for DecodeResult {}

impl Ord for DecodeResult {
  fn cmp(&self, other: &DecodeResult) -> std::cmp::Ordering {
    return self.score.partial_cmp(&other.score).unwrap();
  }
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
pub struct Cracker<'a> {
  ctext: &'a [u8],
  current_byte: usize,

  decode_buf: Option<Vec<u8>>,
}

impl<'a> Cracker<'a> {
  pub fn new(ctext: &'a [u8]) -> Cracker<'a> {
    return Cracker {
      ctext: ctext,
      current_byte: 0,
      decode_buf: None,
    };
  }

  /// Returns the first solution that is has score less than limit
  pub fn first(&mut self, limit: f64) -> Option<DecodeResult> {
    self.find(|result| {
      result.score < limit
    })
  }

  pub fn best_results(&mut self) -> Vec<DecodeResult> {
    let mut results: Vec<DecodeResult> = self.collect();
    results.sort();
    return results;
  }

  pub fn best_result(&mut self) -> Option<DecodeResult> {
    return self.min();
  }
}

impl<'a> Iterator for Cracker<'a> {
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
          let score = english::nonfit_score(&s);

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
