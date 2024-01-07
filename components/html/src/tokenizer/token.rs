#[derive(Debug, Clone)]
pub struct Attribute {
  pub name: String,
  pub value: String,
}

#[derive(Debug, Clone)]
pub enum Token {
  Tag {
    tag_name: String,
    attributes: Vec<Attribute>,
    self_closing: bool,
    is_end_tag: bool,
  },
  Comment(String),
  Character(char),
  DOCTYPE {
    name: Option<String>,
    force_quirks: bool,
  },
  EOF,
}

impl Token {
  pub fn new_start_tag() -> Self {
    Token::Tag {
      tag_name: String::new(),
      is_end_tag: false,
      self_closing: false,
      attributes: Vec::new(),
    }
  }

  pub fn new_end_tag() -> Self {
    Token::Tag {
      tag_name: String::new(),
      is_end_tag: true,
      self_closing: false,
      attributes: Vec::new(),
    }
  }

  pub fn new_comment(data: &str) -> Self {
    Token::Comment(data.to_owned())
  }

  pub fn new_doctype() -> Self {
    Token::DOCTYPE {
      name: None,
      force_quirks: false,
    }
  }

  pub fn set_force_quirks(&mut self, value: bool) {
    if let Token::DOCTYPE {
      ref mut force_quirks,
      ..
    } = self
    {
      *force_quirks = value;
    }
  }

  pub fn is_eof(&self) -> bool {
    match self {
      Token::EOF => true,
      _ => false,
    }
  }
}

impl Attribute {
  pub fn new() -> Self {
    Attribute {
      name: String::new(),
      value: String::new(),
    }
  }
}
