use ecow::EcoString;

use std::cell::RefCell;

pub struct CharacterData {
  data: RefCell<EcoString>,
}

impl CharacterData {
  pub fn new(data: EcoString) -> Self {
    Self {
      data: RefCell::new(data),
    }
  }

  pub fn get_data(&self) -> EcoString {
    self.data.borrow().clone()
  }

  pub fn set_data(&self, data: &str) {
    self.data.replace(EcoString::from(data));
  }
}
