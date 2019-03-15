extern crate failure;
extern crate hamming;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use std::rc::Rc;

fn read_base64_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, failure::Error> {
    let f = File::open(path)?;
    let r = BufReader::new(f);

    let mut buf: Vec<u8> = Vec::new();

    for line in r.lines() {
        buf.write(line?.as_bytes())?;
    }

    return Ok(buf);
}

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

#[derive(Debug)]
struct KeySizeResult {
    keysize: usize,
    score: f64,
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

fn main() -> Result<(), failure::Error> {
    let data = Rc::new(read_base64_file("6.txt")?);

    let keysize_analyzer = KeySizeAnalyzer::new(data.clone(), 2..40);

    for result in keysize_analyzer {
        println!("{:?}", result);
    }

    return Ok(());
}
