#[inline]
fn pkcs7_padding_size(data: &[u8], blocksize: u8) -> u8 {
  blocksize - (data.len() % blocksize as usize) as u8
}

#[inline]
fn pkcs7_padding_extend_helper(data: &mut Vec<u8>, padn: u8) {
  data.reserve(padn as usize);

  for _ in 0..padn {
    data.push(padn);
  }
}

pub fn padding_mut(data: &mut Vec<u8>, blocksize: u8) {
  let padn = pkcs7_padding_size(data, blocksize);
  pkcs7_padding_extend_helper(data, padn);
}

pub fn padding(data: &[u8], blocksize: u8) -> Vec<u8> {
  let padn = pkcs7_padding_size(data, blocksize);
  let mut out = Vec::with_capacity(data.len() + padn as usize);

  out.extend_from_slice(data);
  pkcs7_padding_extend_helper(&mut out, padn);

  return out;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_pkcs7_padding() {
    assert_eq!(super::padding(&vec![0; 4], 8), vec![0, 0, 0, 0, 4, 4, 4, 4]);

    assert_eq!(super::padding(&vec![0; 0], 8), vec![8, 8, 8, 8, 8, 8, 8, 8]);

    assert_eq!(
      super::padding(&vec![0; 8], 16),
      vec![0, 0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8]
    );

    let mut v = vec![0; 4];
    super::padding_mut(&mut v, 8);
    assert_eq!(v, vec![0, 0, 0, 0, 4, 4, 4, 4]);
  }
}
