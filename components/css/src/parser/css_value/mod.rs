pub mod color;

use self::color::Color;

use nom::branch::alt;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum CssValue {
  Keyword(String),
  Length(f32, Unit),
  ColorValue(Color),
}

#[derive(Debug, PartialEq)]
pub enum Unit {
  Px,
}

pub fn css_value(input: &str) -> IResult<&str, CssValue> {
  // TODO: alt((keyword, length, color))(input)
  keyword(input)
}

fn color(input: &str) -> IResult<&str, CssValue> {
  todo!("parse_color");
}

fn keyword(input: &str) -> IResult<&str, CssValue> {
  // TODO
  map(alpha1, |s: &str| CssValue::Keyword(s.to_string()))(input)
}

fn length(input: &str) -> IResult<&str, CssValue> {
  todo!("parse_length");
}
