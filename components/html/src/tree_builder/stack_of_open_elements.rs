use std::{
  ops::{Deref, DerefMut},
  rc::Rc,
};

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

  pub fn get(&self, index: usize) -> NodePtr {
    return self.0[index].clone();
  }

  pub fn current_node(&self) -> Option<NodePtr> {
    if let Some(node) = self.0.last() {
      return Some(node.clone());
    }
    None
  }

  pub fn len(&self) -> usize {
    self.0.len()
  }

  pub fn any<F>(&self, test: F) -> bool
  where
    F: Fn(&NodePtr) -> bool,
  {
    self.0.iter().any(test)
  }

  pub fn contains(&self, tag_name: &str) -> bool {
    self.any(|node| node.as_element().tag_name() == tag_name)
  }

  pub fn contains_in(&self, tag_names: &[&str]) -> bool {
    self.any(|node| tag_names.contains(&node.as_element().tag_name().as_str()))
  }

  pub fn contains_node(&self, node: &NodePtr) -> bool {
    self.any(|node_2| Rc::ptr_eq(node_2, node))
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
