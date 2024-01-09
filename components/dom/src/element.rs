use std::ops::{Deref, DerefMut};
use std::{cell::RefCell, collections::HashMap};

use super::elements::ElementData;
use super::token_list::TokenList;

pub struct AttributeMap(HashMap<String, String>);

pub struct Element {
  data: ElementData,
  id: RefCell<Option<String>>,
  attributes: RefCell<AttributeMap>,
  class_list: RefCell<TokenList>,
}

impl Deref for AttributeMap {
  type Target = HashMap<String, String>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for AttributeMap {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl core::fmt::Debug for Element {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "Element({:?}", self.data)?;
    let class_name = self.class_list.borrow().values();
    if !class_name.is_empty() {
      write!(f, "| Class: {}", class_name)?;
    }
    write!(f, ")")
  }
}

impl AttributeMap {
  pub fn new() -> Self {
    Self(HashMap::new())
  }
}

impl Element {
  pub fn new(data: ElementData) -> Self {
    Self {
      data,
      id: RefCell::new(None),
      attributes: RefCell::new(AttributeMap::new()),
      class_list: RefCell::new(TokenList::new()),
    }
  }

  pub fn tag_name(&self) -> String {
    self.data.tag_name()
  }

  pub fn match_tag_name_in(&self, names: &[&str]) -> bool {
    names.iter().any(|name| self.tag_name() == *name)
  }

  pub fn has_attribute(&self, name: &str) -> bool {
    self.attributes.borrow().contains_key(name)
  }

  pub fn set_attribute(&self, name: &str, value: &str) {
    match name {
      "id" => {
        *self.id.borrow_mut() = Some(value.to_string());
      }
      "class" => {
        *self.class_list.borrow_mut() = TokenList::from(value);
      }
      _ => {
        self
          .attributes
          .borrow_mut()
          .insert(name.to_string(), value.to_string());

        self.data.handle_attribute_change(name, value);
      }
    }
  }
}
