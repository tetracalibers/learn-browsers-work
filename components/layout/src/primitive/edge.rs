#[derive(Debug)]
pub enum Edge {
  Top,
  Right,
  Bottom,
  Left,
}

#[derive(Debug)]
pub struct EdgeSizes {
  top: f32,
  right: f32,
  bottom: f32,
  left: f32,
}

impl Default for EdgeSizes {
  fn default() -> Self {
    Self {
      top: 0.0,
      right: 0.0,
      bottom: 0.0,
      left: 0.0,
    }
  }
}
