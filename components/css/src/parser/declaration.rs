use super::structs::ComponentValue;

#[derive(Debug)]
pub struct Declaration {
  pub name: String,
  pub value: Vec<ComponentValue>,
  pub important: bool,
}

impl Declaration {
  pub fn new(name: String) -> Self {
    Self {
      name,
      value: Vec::new(),
      important: false,
    }
  }
}
