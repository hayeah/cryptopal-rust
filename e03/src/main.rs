extern crate hex;

use std::cmp::Ord;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

fn xor(dst: &mut Vec<u8>, src: &Vec<u8>, mask: u8) {
    for i in 0..src.len() {
        dst[i] = src[i] ^ mask;
    }
}

#[derive(Eq)]
struct DecodeResult {
    key: u8,
    ptext: String,
    score: i64,
}

impl Ord for DecodeResult {
    fn cmp(&self, other: &DecodeResult) -> Ordering {
        // self.height.cmp(&other.height)
        return self.score.cmp(&other.score);
    }
}

impl PartialOrd for DecodeResult {
    fn partial_cmp(&self, other: &DecodeResult) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DecodeResult {
    fn eq(&self, other: &DecodeResult) -> bool {
        self.score == other.score
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let freq_table: HashMap<char, i64> = [
        ('e', 1249),
        ('t', 928),
        ('a', 804),
        ('o', 764),
        ('i', 757),
        ('n', 723),
        ('s', 651),
        ('r', 628),
        ('h', 505),
        ('l', 407),
        ('d', 382),
        ('c', 334),
        ('u', 273),
        ('m', 251),
        ('f', 240),
        ('p', 214),
        ('g', 187),
        ('w', 168),
        ('y', 166),
        ('b', 148),
        ('v', 105),
        ('k', 54),
        ('x', 23),
        ('j', 16),
        ('q', 12),
        ('z', 9),
    ]
    .iter()
    .cloned()
    .collect();

    let input =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let mut results: Vec<DecodeResult> = vec![];

    for c in 0x00..0xff {
        // a-z
        let mut buf = vec![0; input.len()];
        xor(&mut buf, &input, c);
        // let r = String::from_utf8(buf[0..buf.len() - 2].to_vec());
        let r = String::from_utf8(buf);

        if r.is_ok() {
            let s = r.unwrap();

            let mut score: i64 = 0;

            for ch in s.chars() {
                // scoring for English-like text
                if !ch.is_ascii() {
                    score -= 10000; // penalty for non-ascii char
                    continue;
                }

                if ch.is_control() {
                    score -= 10000;
                    continue;
                }

                let kchar = ch.to_ascii_lowercase();
                let f = freq_table.get(&kchar);
                if f.is_some() {
                    score += f.unwrap();
                } else {
                    score -= 1000; // penalty for non-letter char
                }
            }

            results.push(DecodeResult {
                key: c,
                ptext: s,
                score: score,
            })
        }
    }

    results.sort();

    for r in results {
        println!("key={} score={} ptext={:?}", r.key, r.score, r.ptext);
    }

    return Ok(());
}
