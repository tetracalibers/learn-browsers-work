#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Overflow {
  Visible,
  Hidden,
  Clip,
  Scroll,
}
