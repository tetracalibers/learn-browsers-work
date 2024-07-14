pub fn is_not_newline_or_hex_digit(c: char) -> bool {
  !(c.is_ascii_hexdigit() || c == '\n')
}

pub fn non_ascii(c: char) -> bool {
  c as u32 > 127
}
