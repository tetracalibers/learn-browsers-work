use std::ops::Deref;
use std::ops::DerefMut;

use ecow::EcoString;
use ecow::EcoVec;

#[derive(Debug, Clone)]
pub struct Attribute {
  pub name: EcoString,
  pub value: EcoString,
}

#[derive(Debug, Clone)]
pub enum Token {
  DOCTYPE {
    name: Option<EcoString>,
    force_quirks: bool,
  },
  Tag {
    tag_name: EcoString,
    attributes: EcoVec<Attribute>,
    self_closing: bool,
    self_closing_acknowledged: bool,
    is_end_tag: bool,
  },
  Text(EcoString),
  Comment(EcoString),
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
      attributes: EcoVec::new(),
    }
  }

  pub fn new_start_tag_of(name: &str) -> Self {
    Token::Tag {
      tag_name: EcoString::from(name),
      is_end_tag: false,
      self_closing: false,
      self_closing_acknowledged: false,
      attributes: EcoVec::new(),
    }
  }

  pub fn new_end_tag() -> Self {
    Token::Tag {
      tag_name: EcoString::new(),
      is_end_tag: true,
      self_closing: false,
      self_closing_acknowledged: false,
      attributes: EcoVec::new(),
    }
  }

  pub fn new_text(text: &str) -> Self {
    Token::Text(EcoString::from(text))
  }

  pub fn new_doctype_char_of(ch: char) -> Self {
    Token::DOCTYPE {
      name: Some(EcoString::from(ch)),
      force_quirks: false,
    }
  }

  pub fn new_doctype_with_force_quirks() -> Self {
    Token::DOCTYPE {
      name: None,
      force_quirks: true,
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

  pub fn attributes(&self) -> &EcoVec<Attribute> {
    if let Token::Tag { attributes, .. } = self {
      attributes
    } else {
      panic!("Token is not a Tag");
    }
  }

  /* setter ------------------------------------- */

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

  pub fn set_force_quirks(&mut self, value: bool) {
    if let Token::DOCTYPE {
      ref mut force_quirks,
      ..
    } = self
    {
      *force_quirks = value;
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

impl Default for Attribute {
  fn default() -> Self {
    Self::new()
  }
}

impl Attribute {
  pub fn new() -> Self {
    Attribute {
      name: EcoString::new(),
      value: EcoString::new(),
    }
  }

  pub fn new_name_of(name: &str) -> Self {
    Attribute {
      name: EcoString::from(name),
      value: EcoString::new(),
    }
  }
}

impl Deref for Attribute {
  type Target = Attribute;

  fn deref(&self) -> &Self::Target {
    self
  }
}

impl DerefMut for Attribute {
  fn deref_mut(&mut self) -> &mut Self::Target {
    self
  }
}
