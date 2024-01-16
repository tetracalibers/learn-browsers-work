use std::ops::Deref;

use ecow::EcoVec;

#[derive(Debug, Clone)]
pub struct TokenList {
  items: EcoVec<String>,
}

impl From<&str> for TokenList {
  fn from(data: &str) -> Self {
    Self {
      items: data
        .split(' ')
        .filter(|class| class.len() > 0)
        .map(String::from)
        .collect(),
    }
  }
}

impl Deref for TokenList {
  type Target = EcoVec<String>;
  fn deref(&self) -> &Self::Target {
    &self.items
  }
}

impl TokenList {
  pub fn new() -> Self {
    Self {
      items: EcoVec::new(),
    }
  }

  pub fn values(&self) -> String {
    self.items.join(" ")
  }
}
