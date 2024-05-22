#[derive(Debug, PartialEq, Eq)]
pub enum Position {
  Static,
  Relative,
  Absolute,
  Fixed,
}
