use super::structs::ComponentValue;
use super::structs::SimpleBlock;

#[derive(Debug)]
pub struct AtRule {
  pub name: String,
  pub prelude: Vec<ComponentValue>,
  pub block: Option<SimpleBlock>,
}

impl AtRule {
  pub fn new(name: String) -> Self {
    Self {
      name,
      prelude: Vec::new(),
      block: None,
    }
  }
}
