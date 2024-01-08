pub struct TokenList {
  items: Vec<String>,
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

impl TokenList {
  pub fn new() -> Self {
    Self { items: Vec::new() }
  }

  pub fn values(&self) -> String {
    self.items.join(" ")
  }
}
