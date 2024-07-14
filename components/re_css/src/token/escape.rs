use nom::{
  branch::alt,
  bytes::complete::take_while_m_n,
  character::complete::{char, multispace0, satisfy},
  combinator::{opt, recognize},
  sequence::{pair, tuple},
  IResult,
};

use super::utils::is_not_newline_or_hex_digit;

pub fn escape_token(input: &str) -> IResult<&str, &str> {
  alt((
    recognize(pair(char('\\'), satisfy(is_not_newline_or_hex_digit))),
    recognize(tuple((
      char('\\'),
      take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
      opt(multispace0),
    ))),
  ))(input)
}
