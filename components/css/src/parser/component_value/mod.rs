pub mod color;

use self::color::Color;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

use super::utility::alpha1_with_hyphen;
use super::utility::quoted_within_esceped_quote;

#[derive(Debug, PartialEq)]
pub enum ComponentValue {
  // ref: https://www.w3.org/TR/css-values-4/#keywords
  Keyword(String),
  // ref: https://www.w3.org/TR/css-values-4/#dashed-idents
  DashedIndent(String),
  // ref: https://www.w3.org/TR/css-values-4/#strings
  QuotedString(String),
  Length(f32, Unit),
  ColorValue(Color),
}

#[derive(Debug, PartialEq)]
pub enum Unit {
  Px,
}

pub fn component_value(input: &str) -> IResult<&str, ComponentValue> {
  alt((keyword, dashed_ident, quoted_string))(input)
}

fn color(input: &str) -> IResult<&str, ComponentValue> {
  todo!("parse_color");
}

fn keyword(input: &str) -> IResult<&str, ComponentValue> {
  // TODO
  map(alpha1, |s: &str| ComponentValue::Keyword(s.to_string()))(input)
}

fn dashed_ident(input: &str) -> IResult<&str, ComponentValue> {
  map(tuple((tag("--"), alpha1_with_hyphen)), |(_, s)| {
    ComponentValue::DashedIndent(s)
  })(input)
}

fn quoted_string(input: &str) -> IResult<&str, ComponentValue> {
  map(quoted_within_esceped_quote, |s: String| {
    ComponentValue::QuotedString(s)
  })(input)
}

fn length(input: &str) -> IResult<&str, ComponentValue> {
  todo!("parse_length");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_keyword() {
    assert_eq!(
      keyword("separate"),
      Ok(("", ComponentValue::Keyword("separate".to_string())))
    );
  }

  #[test]
  fn test_dashed_ident() {
    assert_eq!(
      dashed_ident("--fg-color"),
      Ok(("", ComponentValue::DashedIndent("fg-color".to_string())))
    );
  }
}
