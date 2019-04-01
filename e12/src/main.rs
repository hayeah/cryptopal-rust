extern crate cryptopal;
extern crate rand;

extern crate failure;

use cryptopal::aes;
use cryptopal::aes::{ECBCipher, BLOCK_SIZE};
use cryptopal::detect::detect_ecb_blocksize;
use cryptopal::encoding::read_base64_file;
use cryptopal::pkcs7;
use rand::prelude::*;

struct MysteryEncrypter {
  secret: Vec<u8>,
  // key: Block128,
  ecb: ECBCipher,
}

impl MysteryEncrypter {
  fn new(secret: Vec<u8>) -> MysteryEncrypter {
    let mut key = [0u8; BLOCK_SIZE];

    rand::thread_rng().fill_bytes(&mut key);

    return MysteryEncrypter {
      secret,
      // key: Block128::clone_from_slice(&key),
      ecb: ECBCipher::new(&key),
    };
  }

  // encrypt an input by randomly choosing cbc or ecb mode. Pad input with
  // random prefix ans suffix.
  fn encrypt(&self, output: &mut Vec<u8>, input: &[u8]) {
    let mut buf = [0u8; BLOCK_SIZE];
    rand::thread_rng().fill_bytes(&mut buf);

    // wrap input data with random prefix and suffix
    let prefix_size: usize = rand::thread_rng().gen_range(0, 16);
    let prefix = &buf[0..prefix_size];
    // let suffix = &buf[prefix_size..];

    // println!("chosen prefix size: {}", prefix_size);

    output.truncate(0);
    output.extend_from_slice(prefix);
    output.extend_from_slice(input);
    output.extend_from_slice(&self.secret);
    // output.extend_from_slice(suffix);

    self.ecb.encrypt(output);
  }
}

const PADBLOCK: [u8; BLOCK_SIZE] = [1u8; BLOCK_SIZE];

struct Cracker {
  encrypter: MysteryEncrypter,
  obuf: Vec<u8>,
  ibuf: Vec<u8>,

  // used to hold the padding of the nth char
  // padding: Vec<u8>,
  // lookup: [[u8; 16]; 256],

  // decoded text
  ptext: Vec<u8>,
}

impl Cracker {
  fn new(encrypter: MysteryEncrypter) -> Cracker {
    let obuf: Vec<u8> = Vec::with_capacity(16 * 1024);
    let ibuf: Vec<u8> = Vec::with_capacity(16 * 1024);
    let ptext: Vec<u8> = Vec::with_capacity(16 * 1024);

    Cracker {
      encrypter,
      obuf,
      ibuf,
      ptext,
      // lookup: [[0u8; 16]; 256],
      // padding: vec![0u8; BLOCK_SIZE],
    }
  }

  // try_encrypt_input encrypts the input into obuf, using padding to
  // align with an unknown random prefix added by the encryptor. It returns the
  // start of the ctext if the alignment is successful.
  fn try_encrypt_input(&mut self, input: &[u8]) -> Option<usize> {
    self.ibuf.truncate(0);
    self.obuf.truncate(0);

    // padding for guessing prefix
    let prefix_size = rand::thread_rng().gen_range(0, 16);
    // 0: [prefix][rand(0..blocksize)]
    // 1: ['1' * blocksize]
    // 2: ['1' * blocksize]
    for _ in 0..(BLOCK_SIZE - prefix_size) {
      self.ibuf.push(0); // this should be a different byte from padding byte
    }
    self.ibuf.extend_from_slice(&PADBLOCK);
    self.ibuf.extend_from_slice(&PADBLOCK);
    self.ibuf.extend_from_slice(input);

    self.encrypter.encrypt(&mut self.obuf, &mut self.ibuf);

    let mut blocks = self.obuf.chunks(BLOCK_SIZE);

    blocks.next().unwrap(); // skip block0
    let block1 = blocks.next().unwrap();
    let block2 = blocks.next().unwrap();

    if block1 != block2 {
      return None;
    }

    // println!("guess prefix size: {}", prefix_size);

    return Some(BLOCK_SIZE * 3);
  }

  fn encrypt_input(&mut self, input: &[u8]) -> usize {
    loop {
      match self.try_encrypt_input(input) {
        None => continue,
        Some(start) => return start,
      };
    }
  }

  // try_crack_nth makes a guess, and crack the nth byte of the secret. Multiple
  // calls would increase the chance of success.
  fn nth_byte(&mut self, n: usize) -> Option<(u8, usize)> {
    // Add our own prefix
    // 0 -> 15
    // 1 -> 14
    // 15 -> 0
    let prefix = [255u8; BLOCK_SIZE];
    let index = n % BLOCK_SIZE; // ith byte in a block
    let padn = BLOCK_SIZE - (index + 1);

    let start = self.encrypt_input(&prefix[..padn]);

    // get the block we want to analyze to get the nth byte. Copy it from the
    // underlying buffer.
    let mut block = [0u8; BLOCK_SIZE];
    let block_index = n / BLOCK_SIZE;
    let i = block_index * BLOCK_SIZE + start;
    block.copy_from_slice(&self.obuf[i..i + BLOCK_SIZE]);
    let leftover = self.obuf[i + BLOCK_SIZE..].len();
    // println!("ctext block: {:?}", block);
    // println!("leftover: {:?}", leftover);

    match self.match_ctext(block) {
      Some(byte) => {
        self.ptext.push(byte);
        return Some((byte, leftover));
      }
      None => return None,
    };
  }

  fn find_block_ctext(&mut self, block: [u8; BLOCK_SIZE]) -> [u8; BLOCK_SIZE] {
    // TODO: add cache
    loop {
      match self.try_encrypt_input(&block) {
        Some(start) => {
          let mut ctext = [0u8; BLOCK_SIZE];
          ctext.copy_from_slice(&self.obuf[start..start + BLOCK_SIZE]);
          return ctext;
        }
        None => continue,
      }
    }
  }

  fn match_ctext(&mut self, ctext: [u8; BLOCK_SIZE]) -> Option<u8> {
    // take previous 15 chars of known text. If there is not enough known text,
    // use the default padding char.
    let mut knownblock = [0u8; BLOCK_SIZE];

    for i in 0..15 {
      let j = (self.ptext.len() as i64) - i - 1;
      if j >= 0 {
        knownblock[14 - (i as usize)] = self.ptext[j as usize];
      } else {
        knownblock[14 - (i as usize)] = 255; // default padding char
      }
    }

    // println!("known: {:?}", knownblock);

    // try to combine different bytes with the known block to see
    // if the ctext matches.
    for i in 0..=255 {
      // println!("try {}", i);
      knownblock[15] = i;

      let ctext2 = self.find_block_ctext(knownblock);

      if ctext2 == ctext {
        return Some(i);
      }
    }

    return None;
  }

  fn all(&mut self) {
    let mut i = 0;

    let mut bytes: Vec<u8> = Vec::new();

    loop {
      match self.nth_byte(i) {
        Some((byte, leftover)) => {
          bytes.push(byte);

          let c: char = byte.into();
          print!("{}", c);

          // print!("{}", Into::<char>::into(byte));

          if leftover > 0 {
            i += 1;
            continue;
          }
        }

        None => break,
      }
    }

    println!("\n");

    pkcs7::unpad_mut(&mut bytes);

    let result = String::from_utf8(bytes);
    println!("result: {:?}", result);
  }
}

fn main() -> Result<(), failure::Error> {
  let secret = read_base64_file("secret.txt")?;
  println!("length of secret: {}", secret.len());

  let mut key = [0u8; 16];
  rand::thread_rng().fill_bytes(&mut key);
  let cipher = aes::ECBCipher::new(&key);

  match detect_ecb_blocksize(&mut |data| cipher.encrypt_nopadding(data)) {
    Some(size) => println!("blocksize {}", size),
    None => (),
  }

  let encrypter = MysteryEncrypter::new(secret);
  let mut cracker = Cracker::new(encrypter);

  cracker.all();

  // println!("result: {:?}", result);

  // println!("{}", );

  return Ok(());
}
