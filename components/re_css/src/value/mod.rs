use length::Length;
use percentage::Percentage;
use property::Property;

use crate::{parser::structure::ComponentValue, token::CSSToken};

pub mod length;
pub mod percentage;
mod property;

pub enum Value {
  Length(Length),
  Percentage(Percentage),
  // css wide keywords
  Inherit,
  Initial,
  Unset,
  Auto,
}

pub trait ValueParser {
  fn parse(token: &CSSToken) -> Option<Value>;

  fn parse_first(values: &Vec<ComponentValue>) -> Option<Value> {
    match values.iter().next() {
      Some(ComponentValue::PreservedToken(token)) => Self::parse(token),
      _ => None,
    }
  }
}

fn has_keyword(values: &Vec<ComponentValue>, target: &str) -> bool {
  match values.iter().next() {
    Some(ComponentValue::PreservedToken(token)) => {
      if let CSSToken::Ident(keyword) = token {
        keyword.eq_ignore_ascii_case(target)
      } else {
        false
      }
    }
    _ => false,
  }
}

macro_rules! parse_value {
  (Auto; $tokens:ident) => {{
      if has_keyword(&$tokens, "auto") {
          Some(Value::Auto)
      } else {
          None
      }
  }};
  (Inherit; $tokens:ident) => {{
      if has_keyword(&$tokens, "inherit") {
          Some(Value::Inherit)
      } else {
          None
      }
  }};
  (Initial; $tokens:ident) => {{
      if has_keyword(&$tokens, "initial") {
          Some(Value::Initial)
      } else {
          None
      }
  }};
  (Unset; $tokens:ident) => {{
      if has_keyword(&$tokens, "unset") {
          Some(Value::Unset)
      } else {
          None
      }
  }};
  ($value:ident; $tokens:ident) => {{
      $value::parse_first(&$tokens)
  }};
  ($value:ident | $($remain:ident)|+; $tokens:ident) => {{
      let value = parse_value!($value; $tokens);
      if value.is_some() {
          return value;
      }
      parse_value!($($remain)|+; $tokens)
  }};
}

impl Value {
  pub fn parse(
    property: Property,
    values: Vec<ComponentValue>,
  ) -> Option<Self> {
    match property {
      Property::MarginTop => {
        parse_value!(
          Length | Percentage | Auto | Inherit | Initial | Unset;
          values
        )
      }
    }
  }
}
