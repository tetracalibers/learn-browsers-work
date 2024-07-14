use nom::character::complete::char;
use nom::{combinator::recognize, sequence::preceded, IResult};

use super::ident::ident_token;

pub fn at_keyword_token(input: &str) -> IResult<&str, &str> {
  recognize(preceded(char('@'), ident_token))(input)
}
