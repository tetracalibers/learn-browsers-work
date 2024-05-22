#[derive(Debug, PartialEq, Eq)]
pub enum Overflow {
  Visible,
  Hidden,
  Clip,
  Scroll,
}
