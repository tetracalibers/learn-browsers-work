use super::Value;
use rustc_hash::FxHashMap;

pub type Properties = FxHashMap<Property, Value>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, strum::EnumIter)]
pub enum Property {
  // display
  Display,
  // physical margin
  MarginTop,
  MarginRight,
  MarginBottom,
  MarginLeft,
  // font
  FontSize,
}

impl std::str::FromStr for Property {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "margin-top" => Ok(Property::MarginTop),
      "margin-right" => Ok(Property::MarginRight),
      "margin-bottom" => Ok(Property::MarginBottom),
      "margin-left" => Ok(Property::MarginLeft),
      "font-size" => Ok(Property::FontSize),
      _ => Err("Invalid property"),
    }
  }
}

impl Property {
  pub fn inheritable(&self) -> bool {
    match self {
      _ => false,
    }
  }
}
