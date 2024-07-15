use nom::{
  branch::alt,
  character::complete::{char, digit1, one_of},
  combinator::{map, opt, recognize},
  sequence::{pair, tuple},
  IResult,
};

use super::{ident::ident_token_str, CSSToken};

fn sign(input: &str) -> IResult<&str, &str> {
  recognize(opt(one_of("+-")))(input)
}

fn integer(input: &str) -> IResult<&str, &str> {
  recognize(pair(sign, digit1))(input)
}

fn decimal(input: &str) -> IResult<&str, &str> {
  recognize(tuple((opt(pair(sign, digit1)), char('.'), digit1)))(input)
}

fn exponent(input: &str) -> IResult<&str, &str> {
  recognize(tuple((one_of("eE"), opt(one_of("+-")), digit1)))(input)
}

fn number_str(input: &str) -> IResult<&str, &str> {
  recognize(tuple((alt((decimal, integer)), opt(exponent))))(input)
}

pub fn number_token(input: &str) -> IResult<&str, CSSToken> {
  map(number_str, |num_str| {
    CSSToken::Number(num_str.parse::<f64>().expect("Failed to parse number"))
  })(input)
}

fn dimension_pair(input: &str) -> IResult<&str, (&str, &str)> {
  pair(number_str, ident_token_str)(input)
}

pub fn dimension_token(input: &str) -> IResult<&str, CSSToken> {
  map(dimension_pair, |(num_str, unit_str)| {
    CSSToken::Dimension(
      num_str.parse::<f64>().expect("Failed to parse dimension number"),
      unit_str,
    )
  })(input)
}

fn percentage_pair(input: &str) -> IResult<&str, (&str, char)> {
  pair(number_str, char('%'))(input)
}

pub fn percentage_token(input: &str) -> IResult<&str, CSSToken> {
  map(percentage_pair, |(num_str, _)| {
    CSSToken::Percentage(
      num_str.parse::<f64>().expect("Failed to parse percentage number"),
    )
  })(input)
}
