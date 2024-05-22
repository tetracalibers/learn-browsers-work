#[derive(Debug)]
pub struct Number(pub f32);

impl PartialEq for Number {
  fn eq(&self, other: &Number) -> bool {
    self.0 == other.0
  }
}

impl Eq for Number {}
