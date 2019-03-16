extern crate cryptopal;
#[macro_use]
extern crate failure;
extern crate base64;
extern crate hamming;

use cryptopal::encoding::read_base64_file;
use cryptopal::single_char;

use std::rc::Rc;

fn hamming_score(data: &[u8], keysize: usize) -> f64 {
  let n = data.len() / keysize;

  // Compute the average of hamming edit distance between blocks
  let mut score: f64 = 0.0;

  for i in 0..n - 1 {
    let block1 = &data[i * keysize..(i + 1) * keysize];
    let block2 = &data[(i + 1) * keysize..(i + 2) * keysize];

    let d = hamming::distance(block1, block2);

    // normalize hamming distance by length of blocksize
    score += d as f64 / keysize as f64;
  }

  return score / n as f64;
}

#[derive(PartialEq, PartialOrd, Debug)]
struct KeySizeResult {
  score: f64,
  keysize: usize,
}

impl Eq for KeySizeResult {}

impl Ord for KeySizeResult {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.partial_cmp(other).unwrap()
  }
}

struct KeySizeAnalyzer {
  // data: &'a [u8],
  data: Rc<Vec<u8>>,
  range: std::ops::Range<usize>,
}

// TODO Can I rewrite this with lifetime?
impl KeySizeAnalyzer {
  fn new(data: Rc<Vec<u8>>, range: std::ops::Range<usize>) -> KeySizeAnalyzer {
    KeySizeAnalyzer {
      data: data,
      range: range,
    }
  }

  fn iter(&self) -> impl Iterator<Item = KeySizeResult> {
    let data = self.data.clone();
    return self
      .range
      .clone()
      .into_iter()
      .map(move |keysize| KeySizeResult {
        keysize: keysize,
        score: hamming_score(&data, keysize),
      });
  }
}

impl IntoIterator for KeySizeAnalyzer {
  type Item = KeySizeResult;

  type IntoIter = Box<dyn Iterator<Item = Self::Item>>;

  fn into_iter(self) -> Self::IntoIter {
    Box::new(self.iter())
  }
}

fn block_column(buf: &mut Vec<u8>, data: &[u8], blocksize: usize, col: usize) {
  buf.truncate(0);
  let mut i = 0 + col;
  while i < data.len() {
    buf.push(data[i]);
    i += blocksize;
  }
}

#[derive(Debug)]
struct CrackResult {
  score: i64,
  key: String,
  ptext: String,
}

fn xor_bytes(buf: &mut [u8], key: &[u8]) {
  for (i, byte) in buf.iter_mut().enumerate() {
    *byte = *byte ^ key[i % key.len()];
  }
}

fn crack(data: &[u8], blocksize: usize) -> Result<CrackResult, failure::Error> {
  let mut block: Vec<u8> = vec![0; (data.len() / blocksize) + 1];
  let mut key: Vec<u8> = vec![0; blocksize];

  for col in 0..blocksize {
    block_column(&mut block, data, blocksize, col);

    let mut cracker = single_char::Cracker::new(&block);

    if let Some(result) = cracker.first(2000.0) {
      // println!("score={}", result.score);
      key[col] = result.key;
    } else {
      return Err(format_err!("no key found"));
    }
  }

  let mut ptext_bytes: Vec<u8> = data.to_vec();
  xor_bytes(&mut ptext_bytes, &key);

  let ptext = String::from_utf8(ptext_bytes)?;

  return Ok(CrackResult {
    score: cryptopal::english::nonfit_score(&ptext) as i64,
    ptext: ptext,
    key: String::from_utf8(key)?,
  });
}

fn main() -> Result<(), failure::Error> {
  let data = Rc::new(read_base64_file("6.txt")?);

  let keysize_analyzer = KeySizeAnalyzer::new(data.clone(), 2..40);
  let mut keysize_results: Vec<KeySizeResult> = keysize_analyzer.into_iter().collect();
  keysize_results.sort();

  // for result in &keysize_results {
  //     println!("{:?}", result);
  // }

  for result in keysize_results.iter().take(10) {
    match crack(&data, result.keysize) {
      Ok(bingo) => {
        println!("{}", bingo.ptext);
        println!("keysize={} key={:?}", result.keysize, bingo.key);
      }
      Err(err) => {
        println!("keysize={} {}", result.keysize, err);
      }
    }
  }

  return Ok(());
}
