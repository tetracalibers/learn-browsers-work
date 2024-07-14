use nom::{
  branch::alt,
  character::complete::{char, satisfy},
  combinator::{map, recognize},
  multi::many0,
  sequence::preceded,
  IResult,
};

use super::{escape::escape_token, utils::non_ascii};

fn ident_char(input: &str) -> IResult<&str, char> {
  alt((
    satisfy(|c| {
      c.is_ascii_alphanumeric() || c == '_' || c == '-' || non_ascii(c)
    }),
    map(escape_token, |s: &str| s.chars().next().unwrap()),
  ))(input)
}

fn ident(input: &str) -> IResult<&str, &str> {
  recognize(many0(ident_char))(input)
}

pub fn hash_token(input: &str) -> IResult<&str, &str> {
  recognize(preceded(char('#'), ident))(input)
}
