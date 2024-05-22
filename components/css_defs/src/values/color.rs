use super::number::Number;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Color {
  CurrentColor,
  Rgba(Number, Number, Number, Number),
  Transparent,
}
