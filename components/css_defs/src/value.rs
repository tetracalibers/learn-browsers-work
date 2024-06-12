use super::values::prelude::*;
use crate::property::Property;
use css::structs::declaration_value::DeclarationValue;

fn has_keyword(values: &[DeclarationValue], target: &str) -> bool {
  match values.iter().next() {
    Some(DeclarationValue::Keyword(keyword)) => {
      keyword.eq_ignore_ascii_case(target)
    }
    _ => false,
  }
}

macro_rules! parse_value {
  (Auto; $tokens:ident) => {{
      if has_keyword($tokens, "auto") {
          Some(Value::Auto)
      } else {
          None
      }
  }};
  (Inherit; $tokens:ident) => {{
      if has_keyword($tokens, "inherit") {
          Some(Value::Inherit)
      } else {
          None
      }
  }};
  (Initial; $tokens:ident) => {{
      if has_keyword($tokens, "initial") {
          Some(Value::Initial)
      } else {
          None
      }
  }};
  (Unset; $tokens:ident) => {{
      if has_keyword($tokens, "unset") {
          Some(Value::Unset)
      } else {
          None
      }
  }};
  ($value:ident; $tokens:ident) => {{
      if let Some(value) = $value::parse($tokens) {
          Some(Value::$value(value))
      } else {
          None
      }
  }};
  ($value:ident | $($remain:ident)|+; $tokens:ident) => {{
      let value = parse_value!($value; $tokens);
      if value.is_some() {
          return value;
      }
      parse_value!($($remain)|+; $tokens)
  }};
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Value {
  Color(Color),
  Display(Display),
  Length(Length),
  Percentage(Percentage),
  BorderStyle(BorderStyle),
  BorderWidth(BorderWidth),
  // todo: BorderRadius(BorderRadius),
  Float(Float),
  Position(Position),
  Direction(Direction),
  TextAlign(TextAlign),
  Overflow(Overflow),
  FontWeight(FontWeight),
  Auto,
  Inherit,
  Initial,
  Unset,
}

impl Value {
  pub fn parse(
    property: &Property,
    values: &[DeclarationValue],
  ) -> Option<Self> {
    match property {
      Property::BorderTopColor => parse_value!(
          Color | Inherit | Initial | Unset;
          values
      ),
      Property::BorderRightColor => parse_value!(
          Color | Inherit | Initial | Unset;
          values
      ),
      Property::BorderBottomColor => parse_value!(
          Color | Inherit | Initial | Unset;
          values
      ),
      Property::BorderLeftColor => parse_value!(
          Color | Inherit | Initial | Unset;
          values
      ),
    }
  }
}
