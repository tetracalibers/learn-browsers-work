use super::ident::ident_token;
use nom::character::complete::char;
use nom::{combinator::recognize, sequence::pair, IResult};

pub fn function_token(input: &str) -> IResult<&str, &str> {
  recognize(pair(ident_token, char('(')))(input)
}
