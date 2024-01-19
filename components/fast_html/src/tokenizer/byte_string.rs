use std::str::from_utf8;

use ecow::EcoString;

pub fn bytes_to_string(bytes: &[u8]) -> EcoString {
  let s = from_utf8(bytes).unwrap();
  EcoString::from(s)
}

pub fn is_ascii(c: u8) -> bool {
  c <= 0x7F
}

pub fn is_ascii_alphabetic(c: u8) -> bool {
  is_ascii_upper_alpha(c) || is_ascii_lower_alpha(c)
}

pub fn is_ascii_alphanumeric(c: u8) -> bool {
  is_ascii_alphabetic(c) || is_ascii_digit(c)
}

pub fn is_ascii_digit(c: u8) -> bool {
  c >= b'0' && c <= b'9'
}

pub fn is_ascii_upper_alpha(c: u8) -> bool {
  c >= b'A' && c <= b'Z'
}

pub fn is_ascii_lower_alpha(c: u8) -> bool {
  c >= b'a' && c <= b'z'
}

pub fn is_whitespace(c: u8) -> bool {
  c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' || c == b'\x0C'
}

pub fn to_ascii_lowercase(c: u8) -> u8 {
  match c {
    b'A'..=b'Z' => c + 32,
    _ => c,
  }
}
