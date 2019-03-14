#[macro_use]
extern crate lazy_static;

extern crate hex;

use std::collections::HashMap;
use std::error::Error;

fn xor(dst: &mut [u8], src: &[u8], mask: u8) {
    for i in 0..src.len() {
        dst[i] = src[i] ^ mask;
    }
}

#[derive(PartialEq, PartialOrd, Default, Clone)]
struct DecodeResult {
    score: f64,
    key: u8,
    ptext: String,
}

// SingleCharKeyCracker iterates through the ASCII bytes, as key to a xor cipher
struct SingleCharKeyCracker {
    ctext: Vec<u8>,
    current_byte: usize,

    decode_buf: Option<Vec<u8>>,
}

impl SingleCharKeyCracker {
    fn new(ctext: Vec<u8>) -> SingleCharKeyCracker {
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
                    let score = non_english_score(key as usize, &s);

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

lazy_static! {
    static ref LETTERS_FREQ_TUPLES: Vec<(char, f64)> = {
        [
            ('e', 12.49),
            ('t', 9.28),
            ('a', 8.04),
            ('o', 7.64),
            ('i', 7.57),
            ('n', 7.23),
            ('s', 6.51),
            ('r', 6.28),
            ('h', 5.05),
            ('l', 4.07),
            ('d', 3.82),
            ('c', 3.34),
            ('u', 2.73),
            ('m', 2.51),
            ('f', 2.40),
            ('p', 2.14),
            ('g', 1.87),
            ('w', 1.68),
            ('y', 1.66),
            ('b', 1.48),
            ('v', 1.05),
            ('k', 5.4),
            ('x', 2.3),
            ('j', 1.6),
            ('q', 1.2),
            ('z', 9.0),
        ]
        .to_vec()
    };
    static ref LETTERS_FREQ: HashMap<char, f64> = { LETTERS_FREQ_TUPLES.iter().cloned().collect() };
}

// A string that matches the expected english letters frequency has score of 0.
// The higher the score, the less english it looks.
fn non_english_score(_key: usize, s: &str) -> f64 {
    let mut char_counts: HashMap<char, i64> = HashMap::new();

    let mut score: f64 = 0.0;

    for ch in s.chars() {
        // scoring for English-like text
        if !(ch.is_alphanumeric() || ch.is_whitespace() || ch.is_ascii_punctuation()) {
            score += 1000.0;
        }

        if ch.is_uppercase() {
            // penalty for upper case letters because they are rarer
            score += 100.0;
        }

        if ch.is_ascii_punctuation() {
            // give punctuation a slight penalty
            score += 100.0;
        }

        if ch == ' ' {
            // encourage whitespace
            score -= 100.0;
        }

        let lch = ch.to_ascii_lowercase();

        char_counts
            .entry(lch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        //
        // Or equivalently:
        //
        // let count = char_counts.entry(ch).or_insert(0);
        // *count += 1;
    }

    let len = s.len() as f64;

    for (ch, count) in &char_counts {
        let lch = ch.to_ascii_lowercase();

        let freq = len / (*count as f64);
        let english_freq: f64 = match LETTERS_FREQ.get(&lch) {
            Some(freq) => *freq,
            None => 0.0f64,
        };

        let diff = freq - english_freq;
        score += (diff * diff).sqrt();
    }

    return score;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input =
        hex::decode("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736")?;

    let mut results: Vec<DecodeResult> = SingleCharKeyCracker::new(input).collect();

    results.sort_by(|a, b| b.partial_cmp(&a).unwrap());

    for r in results {
        println!("key={} score={} ptext={:?}", r.key, r.score, r.ptext);
    }

    return Ok(());
}
