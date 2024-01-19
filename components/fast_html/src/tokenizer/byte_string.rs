use std::str::from_utf8;

use ecow::EcoString;

pub fn bytes_to_string(bytes: &[u8]) -> EcoString {
  let s = from_utf8(bytes).unwrap();
  EcoString::from(s)
}

pub fn is_whitespace(c: u8) -> bool {
  c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == b'\x0C'
}
