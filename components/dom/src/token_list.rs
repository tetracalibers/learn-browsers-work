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
        .filter(|class| !class.is_empty())
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

impl Default for TokenList {
  fn default() -> Self {
    Self::new()
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
