#[derive(Debug)]
pub enum Rule {
  QualifiedRule(QualifiedRule),
  AtRule(AtRule),
}

pub type RuleList = Vec<Rule>;

#[derive(Debug)]
pub struct QualifiedRule {
  pub prelude: Vec<ComponentValue>,
  pub block: Option<SimpleBlock>,
}

#[derive(Debug)]
pub struct AtRule {
  pub name: String,
  pub prelude: Vec<ComponentValue>,
  pub block: Option<SimpleBlock>,
}

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

#[derive(Debug)]
pub struct Declaration {
  pub name: String,
  pub value: Vec<ComponentValue>,
  pub important: bool,
}

impl QualifiedRule {
  pub fn new() -> Self {
    Self {
      prelude: Vec::new(),
      block: None,
    }
  }
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

impl Declaration {
  pub fn new(name: String) -> Self {
    Self {
      name,
      value: Vec::new(),
      important: false,
    }
  }
}
