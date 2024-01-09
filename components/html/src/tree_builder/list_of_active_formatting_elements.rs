use std::ops::{Deref, DerefMut};

use dom::node::NodePtr;

#[derive(Debug)]
pub struct ListOfActiveFormattingElements {
  entries: Vec<Entry>,
}

#[derive(Debug)]
pub enum Entry {
  Marker,
  Element(NodePtr),
}

impl Deref for ListOfActiveFormattingElements {
  type Target = Vec<Entry>;

  fn deref(&self) -> &Self::Target {
    &self.entries
  }
}

impl DerefMut for ListOfActiveFormattingElements {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.entries
  }
}

impl ListOfActiveFormattingElements {
  pub fn new() -> Self {
    Self {
      entries: Vec::new(),
    }
  }
}
