use std::{cell::RefCell, collections::HashMap};

use super::elements::ElementData;
use super::token_list;

pub struct AttributeMap(HashMap<String, String>);

pub struct Element {
  data: ElementData,
  id: RefCell<Option<String>>,
  attributes: RefCell<AttributeMap>,
  class_list: RefCell<token_list::TokenList>,
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
