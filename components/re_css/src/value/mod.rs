use length::Length;
use percentage::Percentage;
use property::Property;
use property::Property::*;

use crate::{parser::structure::ComponentValue, token::CSSToken};

pub mod length;
pub mod percentage;
pub mod property;

#[derive(Debug, Clone)]
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
  fn parse_token(token: &CSSToken) -> Option<Value>;

  fn parse_component_value(value: &ComponentValue) -> Option<Value> {
    if let ComponentValue::PreservedToken(token) = value {
      Self::parse_token(token)
    } else {
      None
    }
  }
}

fn match_keyword(value: &ComponentValue, target: &str) -> bool {
  if let ComponentValue::PreservedToken(CSSToken::Ident(keyword)) = value {
    keyword.eq_ignore_ascii_case(target)
  } else {
    false
  }
}

macro_rules! parse_value {
  (Auto; $tokens:ident) => {{
      if match_keyword(&$tokens, "auto") {
          Some(Value::Auto)
      } else {
          None
      }
  }};
  (Inherit; $tokens:ident) => {{
      if match_keyword(&$tokens, "inherit") {
          Some(Value::Inherit)
      } else {
          None
      }
  }};
  (Initial; $tokens:ident) => {{
      if match_keyword(&$tokens, "initial") {
          Some(Value::Initial)
      } else {
          None
      }
  }};
  (Unset; $tokens:ident) => {{
      if match_keyword(&$tokens, "unset") {
          Some(Value::Unset)
      } else {
          None
      }
  }};
  ($value:ident; $tokens:ident) => {{
      $value::parse_component_value(&$tokens)
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
  pub fn parse(property: &Property, values: &ComponentValue) -> Option<Self> {
    match property {
      MarginTop | MarginRight | MarginBottom | MarginLeft => {
        parse_value!(
          Length | Percentage | Auto | Inherit | Initial | Unset;
          values
        )
      }
    }
  }
}
