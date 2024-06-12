use super::declaration_value::DeclarationValue;

#[derive(Debug, PartialEq)]
pub struct Declaration {
  pub name: String,
  pub value: Vec<DeclarationValue>,
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
