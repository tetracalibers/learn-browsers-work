use nom::character::complete::char;
use nom::combinator::map;
use nom::{sequence::preceded, IResult};

use super::ident::ident_token_str;
use super::CSSToken;

pub fn at_keyword_str(input: &str) -> IResult<&str, &str> {
  preceded(char('@'), ident_token_str)(input)
}

pub fn at_keyword_token(input: &str) -> IResult<&str, CSSToken> {
  map(at_keyword_str, CSSToken::AtKeyword)(input)
}
