use nom::{
  branch::alt,
  character::complete::{char, digit1, one_of},
  combinator::{opt, recognize},
  sequence::{pair, tuple},
  IResult,
};

use super::ident::ident_token;

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

pub fn number_token(input: &str) -> IResult<&str, &str> {
  recognize(tuple((alt((decimal, integer)), opt(exponent))))(input)
}

pub fn dimension_token(input: &str) -> IResult<&str, &str> {
  recognize(pair(number_token, ident_token))(input)
}

pub fn percentage_token(input: &str) -> IResult<&str, &str> {
  recognize(pair(number_token, char('%')))(input)
}
