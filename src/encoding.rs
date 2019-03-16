extern crate base64;

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

  // return Ok(buf);
}
