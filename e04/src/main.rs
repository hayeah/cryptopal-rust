extern crate cryptopal;

#[macro_use]
extern crate failure;

extern crate hex;

use cryptopal::single_char::{Cracker, DecodeResult};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Fail, Debug)]
enum Error {
  #[fail(display = "Invalid hex string: {}", _0)]
  InvalidHexErr(hex::FromHexError),
  #[fail(display = "No English plain text is found for line {}", _0)]
  NotFoundErr(usize),
}

impl From<hex::FromHexError> for Error {
  fn from(e: hex::FromHexError) -> Error {
    Error::InvalidHexErr(e)
  }
}

fn crack(line: &str, lineno: usize) -> Result<DecodeResult, Error> {
  let ctext = hex::decode(line)?;
  let mut cracker = Cracker::new(&ctext);
  match cracker.best_result() {
    Some(result) => {
      if result.score < 1000.0 {
        return Ok(result);
      } else {
        return Err(Error::NotFoundErr(lineno));
      }
    }
    None => Err(Error::NotFoundErr(lineno)),
  }
}

fn main() -> Result<(), failure::Error> {
  let f = File::open("4.txt")?;

  let r = BufReader::new(f);

  for (i, line) in r.lines().enumerate() {
    if let Ok(result) = crack(&line?, i + 1) {
      println!("BINGO: line={} {}", i + 1, result);
    }
  }

  return Ok(());
}
