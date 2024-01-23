use std::cell::RefCell;
use std::collections::HashMap;

use ecow::EcoString;
use ecow::EcoVec;

type AttributeMap = HashMap<EcoString, EcoString>;
type ClassList = EcoVec<EcoString>;

pub struct Element {
  tag_name: EcoString,
  id: RefCell<Option<EcoString>>,
  attributes: RefCell<AttributeMap>,
  class_list: RefCell<ClassList>,
}

impl Element {
  pub fn new(tag_name: &str) -> Self {
    Self {
      tag_name: EcoString::from(tag_name),
      id: RefCell::new(None),
      attributes: RefCell::new(AttributeMap::new()),
      class_list: RefCell::new(ClassList::new()),
    }
  }

  pub fn tag_name(&self) -> &EcoString {
    &self.tag_name
  }

  pub fn match_tag_name_in(&self, names: &[&str]) -> bool {
    names.iter().any(|name| self.tag_name() == *name)
  }

  pub fn attributes(&self) -> RefCell<AttributeMap> {
    self.attributes.clone()
  }

  pub fn id(&self) -> RefCell<Option<EcoString>> {
    self.id.clone()
  }

  pub fn class_list(&self) -> RefCell<ClassList> {
    self.class_list.clone()
  }

  pub fn has_attribute(&self, name: &str) -> bool {
    self.attributes.borrow().contains_key(name)
  }

  pub fn set_attribute(&self, name: &str, value: &str) {
    match name {
      "id" => {
        *self.id.borrow_mut() = Some(EcoString::from(value));
      }
      "class" => {
        self.class_list.borrow_mut().push(EcoString::from(value));
      }
      _ => {
        self
          .attributes
          .borrow_mut()
          .insert(EcoString::from(name), EcoString::from(value));
      }
    }
  }
}

impl core::fmt::Debug for Element {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    // id, attribute, class_list は空の場合は表示しない
    let mut debug_struct = f.debug_struct("Element");
    debug_struct.field("tag_name", &self.tag_name);
    if let Some(id) = &*self.id.borrow() {
      debug_struct.field("id", id);
    }
    if !self.attributes.borrow().is_empty() {
      debug_struct.field("attributes", &self.attributes.borrow());
    }
    if !self.class_list.borrow().is_empty() {
      debug_struct.field("class_list", &self.class_list.borrow());
    }
    debug_struct.finish()
  }
}
