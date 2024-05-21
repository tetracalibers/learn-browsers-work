use super::number::Number;

#[derive(Debug)]
pub enum Color {
  CurrentColor,
  Rgba(Number, Number, Number, Number),
  Transparent,
}
