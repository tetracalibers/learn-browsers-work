use display::Display;
use length::{Length, LengthUnit};
use percentage::Percentage;
use property::Property;
use property::Property::*;

use crate::{parser::structure::ComponentValue, token::CSSToken};

mod display;
pub mod length;
pub mod percentage;
pub mod property;

pub const BASE_FONT_SIZE: f64 = 16.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
  Length(Length),
  Percentage(Percentage),
  Display(Display),
  Inherit,
  Initial,
  Unset,
  Auto,
}

pub trait ValueParser {
  fn parse_token(token: &CSSToken) -> Option<Value>;

  fn parse(values: &[ComponentValue]) -> Option<Value> {
    match values.iter().next() {
      Some(ComponentValue::PreservedToken(token)) => Self::parse_token(token),
      _ => None,
    }
  }
}

fn parse_keyword(values: &[ComponentValue], target: &str) -> bool {
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
      if parse_keyword(&$tokens, "auto") {
          Some(Value::Auto)
      } else {
          None
      }
  }};
  (Inherit; $tokens:ident) => {{
      if parse_keyword(&$tokens, "inherit") {
          Some(Value::Inherit)
      } else {
          None
      }
  }};
  (Initial; $tokens:ident) => {{
      if parse_keyword(&$tokens, "initial") {
          Some(Value::Initial)
      } else {
          None
      }
  }};
  (Unset; $tokens:ident) => {{
      if parse_keyword(&$tokens, "unset") {
          Some(Value::Unset)
      } else {
          None
      }
  }};
  ($value:ident; $tokens:ident) => {{
      $value::parse(&$tokens)
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
  pub fn parse(property: &Property, values: &[ComponentValue]) -> Option<Self> {
    match property {
      Display => {
        parse_value!(
          Display | Inherit | Initial | Unset;
          values
        )
      }
      MarginTop | MarginRight | MarginBottom | MarginLeft => {
        parse_value!(
          Length | Percentage | Auto | Inherit | Initial | Unset;
          values
        )
      }
      FontSize => {
        parse_value!(
          Length | Percentage | Inherit | Initial | Unset;
          values
        )
      }
    }
  }

  pub fn initial(property: &Property) -> Self {
    match property {
      Display => Value::Display(Display::new_inline()),
      MarginTop | MarginRight | MarginBottom | MarginLeft => {
        Value::Length(Length::new_px(0.0))
      }
      FontSize => Value::Length(Length::new_px(BASE_FONT_SIZE)),
    }
  }

  pub fn to_absolute_px(&self) -> f64 {
    match self {
      Value::Length(Length {
        value,
        unit: LengthUnit::Px,
      }) => *value,
      _ => unreachable!("Calling to_absolute_px for unsupported value"),
    }
  }
}
