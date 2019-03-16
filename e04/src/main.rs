extern crate cryptopal;

#[macro_use]
extern crate failure;

extern crate hex;

use cryptopal::encoding::HexLinesDecoder;
use cryptopal::single_char::{Cracker, DecodeResult};

#[derive(Fail, Debug)]
enum Error {
  #[fail(display = "No English plain text is found for line {}", _0)]
  NotFoundErr(usize),
}

fn crack(ctext: &[u8], lineno: usize) -> Result<DecodeResult, Error> {
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
  let decoder = HexLinesDecoder::file("4.txt")?;

  for (i, data) in decoder.iter().enumerate() {
    if let Ok(result) = crack(&data?, i + 1) {
      println!("BINGO: line={} {}", i + 1, result);
    }
  }

  return Ok(());
}
