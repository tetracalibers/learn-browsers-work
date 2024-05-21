use super::number::Number;

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/length
#[derive(Debug)]
pub struct Length {
  pub value: Number,
  pub unit: LengthUnit,
}

#[derive(Debug)]
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
