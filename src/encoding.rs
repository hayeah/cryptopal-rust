extern crate base64;
extern crate hex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Fail, Debug)]
pub enum Error {
  #[fail(display = "read file: {}", _0)]
  IOError(std::io::Error),

  #[fail(display = "base64 decode: {}", _0)]
  DecodeError(base64::DecodeError),

  #[fail(display = "hex decode: {}", _0)]
  HexDecodeError(hex::FromHexError),
}

pub fn read_base64_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
  let f = File::open(path).map_err(Error::IOError)?;
  let r = BufReader::new(f);

  let mut buf: Vec<u8> = Vec::new();

  for result in r.lines() {
    let line = result.map_err(Error::IOError)?;
    buf.write(line.as_bytes()).unwrap(); // Writing to buffer should have no error
  }

  return base64::decode(&buf).map_err(Error::DecodeError);
}

/// Read lines and treat them as hex encoded strings
pub struct HexLinesDecoder<R: BufRead> {
  bufr: R,
}

impl HexLinesDecoder<BufReader<File>> {
  pub fn file<P: AsRef<Path>>(path: P) -> Result<HexLinesDecoder<BufReader<File>>, Error> {
    let f = File::open(path).map_err(Error::IOError)?;
    let r = BufReader::new(f);

    Ok(HexLinesDecoder::new(r))
  }
}

impl<R: BufRead> HexLinesDecoder<R> {
  pub fn new(r: R) -> HexLinesDecoder<R> {
    HexLinesDecoder { bufr: r }
  }

  pub fn iter(self) -> impl Iterator<Item = Result<Vec<u8>, Error>> {
    self
      .bufr
      .lines()
      .map(|line| hex::decode(line.map_err(Error::IOError)?).map_err(Error::HexDecodeError))
  }
}

// impl<R: BufRead> IntoIterator for HexLinesDecoder<R> {
// }
