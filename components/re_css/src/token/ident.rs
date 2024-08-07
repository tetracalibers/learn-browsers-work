use nom::{
  branch::alt,
  character::complete::{char, satisfy},
  combinator::{map, recognize},
  multi::many0,
  sequence::pair,
  IResult,
};

use super::{escape::escape_str, utils::non_ascii, CSSToken};

fn initial_ident_char(input: &str) -> IResult<&str, char> {
  alt((
    satisfy(|c| c.is_ascii_alphabetic() || c == '_' || non_ascii(c)),
    map(escape_str, |s: &str| s.chars().next().unwrap()),
    char('-'),
  ))(input)
}

fn subsequent_ident_char(input: &str) -> IResult<&str, char> {
  alt((
    satisfy(|c| {
      c.is_ascii_alphanumeric() || c == '_' || c == '-' || non_ascii(c)
    }),
    map(escape_str, |s: &str| s.chars().next().unwrap()),
  ))(input)
}

pub fn ident_token_str(input: &str) -> IResult<&str, &str> {
  recognize(pair(initial_ident_char, many0(subsequent_ident_char)))(input)
}

pub fn ident_token(input: &str) -> IResult<&str, CSSToken> {
  map(ident_token_str, CSSToken::Ident)(input)
}
