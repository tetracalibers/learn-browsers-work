use crate::token::CSSToken;

use super::{Value, ValueParser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LengthUnit {
  Px,
  Em,
  Rem,
}

impl std::str::FromStr for LengthUnit {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "px" => Ok(LengthUnit::Px),
      "em" => Ok(LengthUnit::Em),
      "rem" => Ok(LengthUnit::Rem),
      _ => Err("Invalid length unit"),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Length {
  pub value: f64,
  pub unit: LengthUnit,
}

impl Eq for Length {}

impl ValueParser for Length {
  fn parse_token(token: &CSSToken) -> Option<Value> {
    match token {
      CSSToken::Dimension(value, unit_str) => {
        let parsed_unit = unit_str.parse().ok();
        if let Some(unit) = parsed_unit {
          Some(Value::Length(Length {
            value: *value,
            unit,
          }))
        } else {
          None
        }
      }
      _ => None,
    }
  }
}

impl Length {
  pub fn new(value: f64, unit: LengthUnit) -> Self {
    Self { value, unit }
  }

  pub fn new_px(value: f64) -> Self {
    Self::new(value, LengthUnit::Px)
  }
}
