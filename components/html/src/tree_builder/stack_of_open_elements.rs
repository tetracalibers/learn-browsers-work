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

  pub fn current_node(&self) -> Option<NodePtr> {
    if let Some(node) = self.0.last() {
      return Some(node.clone());
    }
    None
  }

  pub fn remove_first_matching_node<F>(&mut self, test: F)
  where
    F: Fn(&NodePtr) -> bool,
  {
    for (i, node) in self.0.iter().rev().enumerate() {
      if test(node) {
        self.0.remove(i);
        return;
      }
    }
  }
}
