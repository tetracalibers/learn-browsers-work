use std::ops::{Deref, DerefMut};

use dom::node::NodePtr;

pub struct StackOfOpenElements(pub Vec<NodePtr>);

impl Deref for StackOfOpenElements {
  type Target = Vec<NodePtr>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for StackOfOpenElements {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl StackOfOpenElements {
  pub fn new() -> Self {
    Self(Vec::new())
  }
}
