use ecow::EcoString;

#[derive(Debug, Clone)]
pub struct Attribute {
  pub name: EcoString,
  pub value: EcoString,
}

#[derive(Debug, Clone)]
pub enum Token {
  Tag {
    tag_name: EcoString,
    attributes: Vec<Attribute>,
    self_closing: bool,
    self_closing_acknowledged: bool,
    is_end_tag: bool,
  },
  Comment(EcoString),
  Character(char),
  DOCTYPE {
    name: Option<EcoString>,
    force_quirks: bool,
  },
  EOF,
}

impl Token {
  /* constructor -------------------------------- */

  pub fn new_start_tag() -> Self {
    Token::Tag {
      tag_name: EcoString::new(),
      is_end_tag: false,
      self_closing: false,
      self_closing_acknowledged: false,
      attributes: Vec::new(),
    }
  }

  pub fn new_start_tag_of(name: &str) -> Self {
    Token::Tag {
      tag_name: EcoString::from(name),
      is_end_tag: false,
      self_closing: false,
      self_closing_acknowledged: false,
      attributes: Vec::new(),
    }
  }

  pub fn new_end_tag() -> Self {
    Token::Tag {
      tag_name: EcoString::new(),
      is_end_tag: true,
      self_closing: false,
      self_closing_acknowledged: false,
      attributes: Vec::new(),
    }
  }

  pub fn new_comment(data: &str) -> Self {
    Token::Comment(EcoString::from(data))
  }

  pub fn new_doctype() -> Self {
    Token::DOCTYPE {
      name: None,
      force_quirks: false,
    }
  }

  /* getter ------------------------------------- */

  pub fn tag_name(&self) -> &EcoString {
    if let Token::Tag { tag_name, .. } = self {
      tag_name
    } else {
      panic!("Token is not a Tag");
    }
  }

  pub fn attributes(&self) -> &Vec<Attribute> {
    if let Token::Tag { attributes, .. } = self {
      attributes
    } else {
      panic!("Token is not a Tag");
    }
  }

  /* setter ------------------------------------- */

  pub fn set_force_quirks(&mut self, value: bool) {
    if let Token::DOCTYPE {
      ref mut force_quirks,
      ..
    } = self
    {
      *force_quirks = value;
    }
  }

  pub fn set_doctype_name_from_char(&mut self, ch: char) {
    if let Token::DOCTYPE { ref mut name, .. } = self {
      let mut new_name = EcoString::new();
      new_name.push(ch);
      *name = Some(new_name);
    }
  }

  pub fn acknowledge_self_closing_if_set(&mut self) {
    if let Token::Tag {
      ref mut self_closing_acknowledged,
      self_closing,
      ..
    } = self
    {
      if *self_closing {
        *self_closing_acknowledged = true;
      }
    }
  }

  /* checker ------------------------------------ */

  pub fn is_start_tag(&self) -> bool {
    match self {
      Token::Tag { is_end_tag, .. } => !is_end_tag,
      _ => false,
    }
  }

  pub fn is_end_tag(&self) -> bool {
    match self {
      Token::Tag { is_end_tag, .. } => *is_end_tag,
      _ => false,
    }
  }

  pub fn is_eof(&self) -> bool {
    match self {
      Token::EOF => true,
      _ => false,
    }
  }

  pub fn match_tag_name_in(&self, names: &[&str]) -> bool {
    if let Token::Tag { tag_name, .. } = self {
      names.contains(&tag_name.as_str())
    } else {
      false
    }
  }
}

impl Attribute {
  pub fn new() -> Self {
    Attribute {
      name: EcoString::new(),
      value: EcoString::new(),
    }
  }
}
