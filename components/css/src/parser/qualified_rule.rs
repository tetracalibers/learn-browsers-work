use super::atomic::ComponentValue;
use super::atomic::SimpleBlock;

#[derive(Debug)]
pub struct QualifiedRule {
  pub prelude: Vec<ComponentValue>,
  pub block: Option<SimpleBlock>,
}

impl QualifiedRule {
  pub fn new() -> Self {
    Self {
      prelude: Vec::new(),
      block: None,
    }
  }
}
