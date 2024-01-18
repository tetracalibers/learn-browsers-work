use std::cell::RefCell;

use ecow::EcoString;

pub struct Text {
  pub value: RefCell<EcoString>,
}

impl Text {
  pub fn new(value: EcoString) -> Self {
    Self {
      value: RefCell::new(value),
    }
  }

  pub fn get_data(&self) -> EcoString {
    self.value.borrow().clone()
  }

  pub fn set_data(&self, value: EcoString) {
    *self.value.borrow_mut() = value;
  }
}
