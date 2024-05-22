#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Position {
  Static,
  Relative,
  Absolute,
  Fixed,
}
