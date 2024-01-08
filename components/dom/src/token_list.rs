pub struct TokenList {
  items: Vec<String>,
}

impl TokenList {
  pub fn new() -> Self {
    Self { items: Vec::new() }
  }

  pub fn values(&self) -> String {
    self.items.join(" ")
  }
}
