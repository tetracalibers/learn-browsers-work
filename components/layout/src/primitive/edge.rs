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
