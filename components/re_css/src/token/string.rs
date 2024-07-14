use nom::{
  branch::alt,
  bytes::complete::take_while,
  character::complete::{char, none_of},
  combinator::recognize,
  multi::many0,
  sequence::{delimited, pair},
  IResult,
};

fn is_not_newline_or_quote(c: char) -> bool {
  c != '"' && c != '\'' && c != '\n'
}

fn escape(input: &str) -> IResult<&str, &str> {
  recognize(pair(char('\\'), none_of("\n")))(input)
}

fn string_content(input: &str) -> IResult<&str, &str> {
  recognize(many0(alt((take_while(is_not_newline_or_quote), escape))))(input)
}

fn double_quoted_string(input: &str) -> IResult<&str, &str> {
  delimited(char('"'), string_content, char('"'))(input)
}

fn single_quoted_string(input: &str) -> IResult<&str, &str> {
  delimited(char('\''), string_content, char('\''))(input)
}

pub fn string_token(input: &str) -> IResult<&str, &str> {
  alt((double_quoted_string, single_quoted_string))(input)
}
