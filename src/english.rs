use std::collections::HashMap;

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
pub fn nonfit_score(s: &str) -> f64 {
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
