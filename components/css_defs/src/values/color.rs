use super::number::Number;

#[derive(Debug, PartialEq, Eq)]
pub enum Color {
  CurrentColor,
  Rgba(Number, Number, Number, Number),
  Transparent,
}
