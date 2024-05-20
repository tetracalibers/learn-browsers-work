use super::component_value::ComponentValue;

#[derive(Debug, PartialEq)]
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
