use std::{
  ops::{Deref, DerefMut},
  rc::Rc,
};

use dom::node::NodePtr;

const SCOPE_BASE_LIST: [&str; 9] = [
  "applet", "caption", "html", "table", "td", "th", "marquee", "object",
  "template",
];

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

  /* getter ------------------------------------- */

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

  /* predicate ---------------------------------- */

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

  // tag_namesのいずれでもないnodeを持つ場合にtrueを返す
  pub fn contains_not_in(&self, tag_names: &[&str]) -> bool {
    self.any(|node| !tag_names.contains(&node.as_element().tag_name().as_str()))
  }

  pub fn contains_node(&self, node: &NodePtr) -> bool {
    self.any(|node_2| Rc::ptr_eq(node_2, node))
  }

  /* scope -------------------------------------- */

  pub fn has_element_in_specific_scope(
    &self,
    tag_name: &str,
    list: Vec<&str>,
  ) -> bool {
    for node in self.0.iter().rev() {
      let element = node.as_element();

      if element.tag_name() == tag_name {
        return true;
      }

      if list.contains(&element.tag_name().as_str()) {
        return false;
      }
    }

    false
  }

  pub fn has_element_in_scope(&self, tag_name: &str) -> bool {
    self.has_element_in_specific_scope(tag_name, SCOPE_BASE_LIST.to_vec())
  }

  pub fn has_element_in_button_scope(&self, tag_name: &str) -> bool {
    let mut list = SCOPE_BASE_LIST.to_vec();
    list.push("button");
    self.has_element_in_specific_scope(tag_name, list)
  }

  /* pop ---------------------------------------- */

  // tag_nameがpopされるまでpopする
  pub fn pop_until(&mut self, tag_name: &str) {
    while let Some(node) = self.current_node() {
      if node.as_element().tag_name() == tag_name {
        self.0.pop();
        return;
      }
      self.0.pop();
    }
  }

  /* remove ------------------------------------- */

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
