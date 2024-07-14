use nom::{
  branch::alt,
  bytes::complete::{tag, take_while},
  character::complete::{char, multispace0},
  combinator::recognize,
  multi::many0,
  sequence::{delimited, tuple},
  IResult,
};

use super::escape::escape_token;

fn non_printable(c: char) -> bool {
  c.is_control() && !c.is_whitespace()
}

fn url_char(c: char) -> bool {
  !c.is_whitespace()
    && c != '"'
    && c != '\''
    && c != '('
    && c != ')'
    && !non_printable(c)
}

fn url_content(input: &str) -> IResult<&str, &str> {
  recognize(many0(alt((take_while(url_char), escape_token))))(input)
}

pub fn url_token(input: &str) -> IResult<&str, &str> {
  delimited(
    tuple((tag("url"), multispace0, char('('), multispace0)),
    url_content,
    tuple((multispace0, char(')'))),
  )(input)
}
