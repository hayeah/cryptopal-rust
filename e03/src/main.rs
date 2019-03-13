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
fn non_english_score(s: &str) -> f64 {
    let mut char_counts: HashMap<char, i64> = HashMap::new();

    for ch in s.chars() {
        match char_counts.get_mut(&ch) {
            Some(count) => {
                *count += 1;
            }
            _ => {
                char_counts.insert(ch, 1);
            }
        }
    }

    let len = s.len() as f64;
    let mut score: f64 = 0.0;

    for (ch, count) in char_counts.iter() {
        // scoring for English-like text
        if !(ch.is_alphanumeric() || ch.is_whitespace() || ch.is_ascii_punctuation()) {
            score += (*count as f64) * 1000.0;
            // score += *count as f64;
            continue;
        }

        if ch.is_ascii_punctuation() {
            // give punctuation a slight penalty
            score += 100.0;
            continue;
        }

        let freq = len / (*count as f64);
        let english_freq: f64 = match LETTERS_FREQ.get(ch) {
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

    let mut results: Vec<DecodeResult> = vec![DecodeResult::default(); 255];
    results.truncate(0);

    let mut buf = vec![0; input.len()];
    for key in 0x00..0xff {
        xor(&mut buf, &input, key);

        match String::from_utf8(buf) {
            Err(err) => {
                // Reuse the buffer that failed to parse into a string
                buf = err.into_bytes();
            }
            Ok(s) => {
                let score = non_english_score(&s);

                results.push(DecodeResult {
                    key: key,
                    ptext: s,
                    score: score,
                });

                // Allocate a new buffer for the next decode
                buf = vec![0; input.len()];
            }
        }
    }

    results.sort_by(|a, b| b.partial_cmp(&a).unwrap());

    for r in results {
        println!("key={} score={} ptext={:?}", r.key, r.score, r.ptext);
    }

    return Ok(());
}
