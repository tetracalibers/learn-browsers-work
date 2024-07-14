use css::structs::declaration_value::DeclarationValue;

use super::number::Number;

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/length
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Length {
  pub value: Number,
  pub unit: LengthUnit,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LengthUnit {
  // Relative length units based on font
  Cap,
  Ch,
  Em,
  Ex,
  Lh,
  // Relative length units based on root element's font
  Rem,
  // Relative length units based on viewport
  Vh,
  Vw,
  Vmax,
  Vmin,
  // Absolute length units
  Px,
}

impl Length {
  pub fn new_px(value: f32) -> Self {
    todo!("Length::new_px");
  }
  pub fn parse(values: &[DeclarationValue]) -> Option<Self> {
    todo!("Length::parse");
  }
}
