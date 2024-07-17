use super::Value;
use rustc_hash::FxHashMap;

pub type Properties = FxHashMap<Property, Value>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, strum::EnumIter)]
pub enum Property {
  // physical margin
  MarginTop,
  MarginRight,
  MarginBottom,
  MarginLeft,
}

impl std::str::FromStr for Property {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "margin-top" => Ok(Property::MarginTop),
      "margin-right" => Ok(Property::MarginRight),
      "margin-bottom" => Ok(Property::MarginBottom),
      "margin-left" => Ok(Property::MarginLeft),
      _ => Err("Invalid property"),
    }
  }
}
