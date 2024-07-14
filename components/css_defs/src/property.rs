use crate::value::Value;
use rustc_hash::FxHashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, strum::EnumIter)]
pub enum Property {
  BorderTopColor,
  BorderRightColor,
  BorderBottomColor,
  BorderLeftColor,
  FontSize,
}

impl Property {
  pub fn parse(name: &str) -> Option<Self> {
    match name {
      "border-top-color" => Some(Self::BorderTopColor),
      "border-right-color" => Some(Self::BorderRightColor),
      "border-bottom-color" => Some(Self::BorderBottomColor),
      "border-left-color" => Some(Self::BorderLeftColor),
      "font-size" => Some(Property::FontSize),
      _ => {
        log::debug!("Unknown property: {}", name);
        None
      }
    }
  }

  pub fn inheritable(&self) -> bool {
    match self {
      Self::BorderTopColor
      | Self::BorderRightColor
      | Self::BorderBottomColor
      | Self::BorderLeftColor
      | Self::FontSize => true,
      _ => false,
    }
  }
}

pub type Properties = FxHashMap<Property, Value>;
