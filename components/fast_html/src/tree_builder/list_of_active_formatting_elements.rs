use std::{
  ops::{Deref, DerefMut},
  rc::Rc,
};

use ecow::EcoVec;

use fast_dom::node::NodePtr;

#[derive(Debug)]
pub struct ListOfActiveFormattingElements {
  entries: EcoVec<Entry>,
}

#[derive(Debug, Clone)]
pub enum Entry {
  Marker,
  Element(NodePtr),
}

impl Deref for ListOfActiveFormattingElements {
  type Target = EcoVec<Entry>;

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
      entries: EcoVec::new(),
    }
  }

  pub fn get_element_after_last_marker(
    &self,
    tag_name: &str,
  ) -> Option<NodePtr> {
    self.iter().rev().find_map(|entry| match entry {
      Entry::Marker => None,
      Entry::Element(node) => {
        if node.as_element().tag_name() == tag_name {
          Some(node.clone())
        } else {
          None
        }
      }
    })
  }

  pub fn get_index_of_node(&self, node: &NodePtr) -> Option<usize> {
    self.iter().rposition(|entry| match entry {
      Entry::Marker => false,
      Entry::Element(e) => Rc::ptr_eq(e, node),
    })
  }

  pub fn remove_element(&mut self, element: &NodePtr) {
    let index = self
      .iter()
      .rposition(|entry| match entry {
        Entry::Marker => false,
        Entry::Element(e) => Rc::ptr_eq(e, element),
      })
      .unwrap_or_else(|| {
        panic!("Unable to find active element: {:?}", element)
      });
    self.entries.remove(index);
  }

  pub fn contains_node(&self, node: &NodePtr) -> bool {
    self
      .iter()
      .rfind(|entry| match entry {
        Entry::Marker => false,
        Entry::Element(e) => Rc::ptr_eq(e, node),
      })
      .is_some()
  }

  pub fn add_marker(&mut self) {
    self.entries.push(Entry::Marker);
  }
}
