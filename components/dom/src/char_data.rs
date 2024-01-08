use std::cell::RefCell;

pub struct CharacterData {
  data: RefCell<String>,
}

impl CharacterData {
  pub fn new(data: String) -> Self {
    Self {
      data: RefCell::new(data),
    }
  }

  pub fn get_data(&self) -> String {
    self.data.borrow().clone()
  }

  pub fn set_data(&self, data: &str) {
    self.data.replace(data.to_string());
  }
}
