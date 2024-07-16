use nom::{
  branch::alt,
  character::complete::{char, satisfy},
  combinator::{map, recognize},
  multi::many0,
  sequence::preceded,
  IResult,
};

use super::{escape::escape_str, utils::non_ascii, CSSToken};

fn ident_char(input: &str) -> IResult<&str, char> {
  alt((
    satisfy(|c| {
      c.is_ascii_alphanumeric() || c == '_' || c == '-' || non_ascii(c)
    }),
    map(escape_str, |s: &str| s.chars().next().unwrap()),
  ))(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
  recognize(many0(ident_char))(input)
}

fn hash_str(input: &str) -> IResult<&str, &str> {
  preceded(char('#'), ident)(input)
}

pub fn hash_token(input: &str) -> IResult<&str, CSSToken> {
  map(hash_str, CSSToken::Hash)(input)
}
