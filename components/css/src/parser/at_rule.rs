use super::atomic::ComponentValue;
use super::atomic::SimpleBlock;

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
