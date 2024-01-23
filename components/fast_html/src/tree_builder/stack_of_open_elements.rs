use std::{
  ops::{Deref, DerefMut},
  rc::Rc,
};

use fast_dom::node::NodePtr;

use ecow::EcoVec;

const SCOPE_BASE_LIST: [&[u8]; 9] = [
  b"applet",
  b"caption",
  b"html",
  b"table",
  b"td",
  b"th",
  b"marquee",
  b"object",
  b"template",
];

#[derive(Debug)]
pub struct StackOfOpenElements(pub EcoVec<NodePtr>);

impl Deref for StackOfOpenElements {
  type Target = EcoVec<NodePtr>;
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
    Self(EcoVec::new())
  }

  /* getter ------------------------------------- */

  pub fn get(&self, index: usize) -> NodePtr {
    self.0[index].clone()
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

  pub fn contains(&self, tag_name: &[u8]) -> bool {
    self.any(|node| node.as_element().tag_name_as_bytes() == tag_name)
  }

  pub fn contains_in(&self, tag_names: &[&[u8]]) -> bool {
    self.any(|node| tag_names.contains(&node.as_element().tag_name_as_bytes()))
  }

  // tag_namesのいずれでもないnodeを持つ場合にtrueを返す
  pub fn contains_not_in(&self, tag_names: &[&[u8]]) -> bool {
    self.any(|node| !tag_names.contains(&node.as_element().tag_name_as_bytes()))
  }

  pub fn contains_node(&self, node: &NodePtr) -> bool {
    self.any(|node_2| Rc::ptr_eq(node_2, node))
  }

  /* scope -------------------------------------- */

  pub fn has_element_in_specific_scope(
    &self,
    target_node: &NodePtr,
    list: EcoVec<&[u8]>,
  ) -> bool {
    for node in self.0.iter().rev() {
      if Rc::ptr_eq(node, target_node) {
        return true;
      }

      if list.contains(&node.as_element().tag_name_as_bytes()) {
        return false;
      }
    }

    false
  }

  pub fn has_element_name_in_specific_scope(
    &self,
    tag_name: &[u8],
    list: EcoVec<&[u8]>,
  ) -> bool {
    for node in self.0.iter().rev() {
      let element = node.as_element();

      if element.tag_name_as_bytes() == tag_name {
        return true;
      }

      if list.contains(&element.tag_name_as_bytes()) {
        return false;
      }
    }

    false
  }

  pub fn has_oneof_element_names_in_specific_scope(
    &self,
    tag_names: &[&[u8]],
    list: EcoVec<&[u8]>,
  ) -> bool {
    for node in self.0.iter().rev() {
      let element = node.as_element();

      if tag_names.contains(&element.tag_name_as_bytes()) {
        return true;
      }

      if list.contains(&element.tag_name_as_bytes()) {
        return false;
      }
    }

    false
  }

  pub fn has_element_in_scope(&self, target_node: &NodePtr) -> bool {
    self
      .has_element_in_specific_scope(target_node, EcoVec::from(SCOPE_BASE_LIST))
  }

  pub fn has_element_name_in_scope(&self, tag_name: &[u8]) -> bool {
    self.has_element_name_in_specific_scope(
      tag_name,
      EcoVec::from(SCOPE_BASE_LIST),
    )
  }

  // すべてのtag_nameがscope内にない場合にtrueを返す
  pub fn has_not_all_element_names_in_scope(
    &self,
    tag_names: &[&[u8]],
  ) -> bool {
    for tag_name in tag_names {
      if self.has_element_name_in_scope(tag_name) {
        return false;
      }
    }

    true
  }

  pub fn has_element_name_in_button_scope(&self, tag_name: &[u8]) -> bool {
    let mut list = EcoVec::from(SCOPE_BASE_LIST);
    list.push(b"button");
    self.has_element_name_in_specific_scope(tag_name, list)
  }

  pub fn has_element_name_in_list_item_scope(&self, tag_name: &[u8]) -> bool {
    let mut list = EcoVec::from(SCOPE_BASE_LIST);
    list.push(b"ol");
    list.push(b"ul");
    self.has_element_name_in_specific_scope(tag_name, list)
  }

  pub fn has_element_name_in_table_scope(&self, tag_name: &[u8]) -> bool {
    let mut list = EcoVec::from(SCOPE_BASE_LIST);
    list.push(b"html");
    list.push(b"table");
    list.push(b"template");
    self.has_element_name_in_specific_scope(tag_name, list)
  }

  pub fn has_oneof_element_names_in_table_scope(
    &self,
    tag_names: &[&[u8]],
  ) -> bool {
    let mut list = EcoVec::from(SCOPE_BASE_LIST);
    list.push(b"html");
    list.push(b"table");
    list.push(b"template");
    self.has_oneof_element_names_in_specific_scope(tag_names, list)
  }

  /* pop ---------------------------------------- */

  // tag_nameがpopされるまでpopする
  pub fn pop_until(&mut self, tag_name: &[u8]) {
    while let Some(node) = self.current_node() {
      if node.as_element().tag_name_as_bytes() == tag_name {
        self.0.pop();
        break;
      }
      self.0.pop();
    }
  }

  // indexがpopsされる直前までpopする
  pub fn pop_before_index(&mut self, index: usize) {
    while self.0.len() > index {
      self.0.pop();
    }
  }

  // tag_namesのいずれかがpopされるまでpopする
  pub fn pop_until_some_in(&mut self, tag_names: &[&[u8]]) {
    while let Some(node) = self.current_node() {
      if tag_names.contains(&node.as_element().tag_name_as_bytes()) {
        self.0.pop();
        break;
      }
      self.0.pop();
    }
  }

  pub fn pop_while_not_in(&mut self, tag_names: &[&[u8]]) {
    while let Some(node) = self.current_node() {
      if tag_names.contains(&node.as_element().tag_name_as_bytes()) {
        break;
      }
      self.0.pop();
    }
  }

  pub fn clear_back_to_table_context(&mut self) {
    self.pop_while_not_in(&[b"table", b"template", b"html"]);
  }

  pub fn clear_back_to_table_body_context(&mut self) {
    self.pop_while_not_in(&[
      b"table",
      b"tbody",
      b"tfoot",
      b"thead",
      b"template",
      b"html",
    ]);
  }

  pub fn clear_back_to_table_row_context(&mut self) {
    self.pop_while_not_in(&[b"tr", b"template", b"html"]);
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
