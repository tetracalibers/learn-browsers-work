#[derive(Debug)]
pub struct SimpleBlock {
  pub associated_token: BracketLeftToken,
  pub value: Vec<ComponentValue>,
}

#[derive(Debug)]
enum BracketLeftToken {
  // [
  SquareBracketLeft,
  // {
  CurlyBracketLeft,
  // (
  RoundBracketLeft,
}

#[derive(Debug)]
pub struct Function {
  pub name: String,
  pub value: Vec<ComponentValue>,
}

#[derive(Debug)]
pub enum ComponentValue {
  PreservedToken(BracketLeftToken),
  Function(Function),
  SimpleBlock(SimpleBlock),
}

impl SimpleBlock {
  pub fn new(associated_token: BracketLeftToken) -> Self {
    Self {
      associated_token,
      value: Vec::new(),
    }
  }
}

impl Function {
  pub fn new(name: String) -> Self {
    Self {
      name,
      value: Vec::new(),
    }
  }
}
