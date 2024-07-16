use nom::{
  branch::alt,
  bytes::complete::take_while_m_n,
  character::complete::{char, multispace0, satisfy},
  combinator::{map, opt, recognize},
  sequence::{pair, tuple},
  IResult,
};

use super::{utils::is_not_newline_or_hex_digit, CSSToken};

pub fn escape_str(input: &str) -> IResult<&str, &str> {
  alt((
    recognize(pair(char('\\'), satisfy(is_not_newline_or_hex_digit))),
    recognize(tuple((
      char('\\'),
      take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
      opt(multispace0),
    ))),
  ))(input)
}

pub fn escape_token(input: &str) -> IResult<&str, CSSToken> {
  map(escape_str, CSSToken::Escape)(input)
}
