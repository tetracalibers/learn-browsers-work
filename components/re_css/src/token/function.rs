use super::ident::ident_token_str;
use super::CSSToken;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::terminated;
use nom::IResult;

pub fn function_token_str(input: &str) -> IResult<&str, &str> {
  terminated(ident_token_str, char('('))(input)
}

pub fn function_token(input: &str) -> IResult<&str, CSSToken> {
  map(function_token_str, CSSToken::Function)(input)
}
