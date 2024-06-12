use crate::structs::declaration_value::DeclarationValue;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

use super::utility::alpha1_with_hyphen;
use super::utility::quoted_within_esceped_quote;

pub fn component_value(input: &str) -> IResult<&str, DeclarationValue> {
  alt((keyword, dashed_ident, quoted_string))(input)
}

fn color(_input: &str) -> IResult<&str, DeclarationValue> {
  todo!("parse_color");
}

fn keyword(input: &str) -> IResult<&str, DeclarationValue> {
  // TODO
  map(alpha1, |s: &str| DeclarationValue::Keyword(s.to_string()))(input)
}

fn dashed_ident(input: &str) -> IResult<&str, DeclarationValue> {
  map(tuple((tag("--"), alpha1_with_hyphen)), |(_, s)| {
    DeclarationValue::DashedIndent(s)
  })(input)
}

fn quoted_string(input: &str) -> IResult<&str, DeclarationValue> {
  map(quoted_within_esceped_quote, |s: String| {
    DeclarationValue::QuotedString(s)
  })(input)
}

fn length(_input: &str) -> IResult<&str, DeclarationValue> {
  todo!("parse_length");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_keyword() {
    assert_eq!(
      keyword("separate"),
      Ok(("", DeclarationValue::Keyword("separate".to_string())))
    );
  }

  #[test]
  fn test_dashed_ident() {
    assert_eq!(
      dashed_ident("--fg-color"),
      Ok(("", DeclarationValue::DashedIndent("fg-color".to_string())))
    );
  }
}
