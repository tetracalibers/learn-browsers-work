use crate::token::CSSToken;

use super::{Value, ValueParser};

enum LengthUnit {
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

pub struct Length {
  value: f64,
  unit: LengthUnit,
}

impl ValueParser for Length {
  fn parse(token: &CSSToken) -> Option<Value> {
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
