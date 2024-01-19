mod byte_string;
mod state;
mod stream;
pub mod token;

use std::collections::HashSet;

use self::byte_string::*;
use self::state::State;
use self::stream::Stream;
use self::token::Attribute;
use self::token::Token;

use log::{debug, warn};

use ecow::EcoVec;

macro_rules! noop {
  () => {};
}

const REPLACEMENT_CHARACTER: char = '\u{FFFD}';

pub struct Tokenizer<'a> {
  state: State,
  return_state: Option<State>,
  stream: Stream<'a, u8>,
  output: EcoVec<Token>,
  current_token: Option<Token>,
  last_emitted_start_tag: Option<Token>,
}

impl<'a> Tokenizer<'a> {
  pub fn new(data: &'a [u8]) -> Self {
    Self {
      state: State::Data,
      return_state: None,
      stream: Stream::new(data),
      output: EcoVec::new(),
      current_token: None,
      last_emitted_start_tag: None,
    }
  }

  pub fn next_token(&mut self) -> Token {
    if !self.output.is_empty() {
      return self.output.pop().unwrap();
    }

    loop {
      let token = match self.state {
        State::Data => self.process_data_state(),
        State::TagOpen => self.process_tag_open_state(),
        State::TagName => self.process_tag_name_state(),
        State::EndTagOpen => self.process_end_tag_open_state(),
        State::BeforeAttributeName => {
          self.process_before_attribute_name_state()
        }
        State::AttributeName => self.process_attribute_name_state(),
        State::AfterAttributeName => self.process_after_attribute_name_state(),
        State::BeforeAttributeValue => {
          self.process_before_attribute_value_state()
        }
        State::AttributeValueDoubleQuoted => {
          self.process_attribute_value_double_quoted_state()
        }
        State::AttributeValueSingleQuoted => {
          self.process_attribute_value_single_quoted_state()
        }
        State::AttributeValueUnQuoted => {
          self.process_attribute_value_unquoted_state()
        }
        State::AfterAttributeValueQuoted => {
          self.process_after_attribute_value_quoted_state()
        }
      };

      if let Some(token) = token {
        return token;
      }
    }
  }

  fn switch_to(&mut self, state: State) {
    debug!("Tokenizer State: switch to {:#?}", state);
    self.state = state;
  }

  /* -------------------------------------------- */

  fn process_data_state(&mut self) -> Option<Token> {
    let bytes = self.read_to_many(&[b'<', b'&', b'\0']);

    if !bytes.is_empty() {
      return Some(self.emit_text(bytes));
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      return Some(self.emit_eof());
    }

    let c = self.read_current();

    match c {
      b'<' => {
        self.return_state = Some(State::Data);
        self.switch_to(State::TagOpen);
      }
      b'&' => {
        unimplemented!("undefined State::CharacterReference");
      }
      b'\0' => {
        warn!("unexpected-null-character");
        return Some(self.emit_text(bytes));
      }
      _ => {
        noop!();
      }
    }

    None
  }

  fn process_tag_open_state(&mut self) -> Option<Token> {
    let c = self.read_next();

    if c.is_ascii_alphanumeric() {
      self.new_token(Token::new_start_tag());
      self.switch_to(State::TagName);
      return None;
    }

    match c {
      b'!' => {
        unimplemented!("undefined State::MarkupDeclarationOpen");
      }
      b'/' => {
        self.switch_to(State::EndTagOpen);
      }
      b'?' => {
        warn!("unexpected-question-mark-instead-of-tag-name");
        unimplemented!("undefined Token::Comment and State::BogusComment");
      }
      _ if self.stream.is_eof() => {
        warn!("eof-before-tag-name");
        self.will_emit(Token::new_text("<"));
        return Some(self.emit_eof());
      }
      _ => {
        warn!("invalid-first-character-of-tag-name");
        self.will_emit(Token::new_text("<"));
        self.reconsume_in_state(State::Data);
      }
    }

    None
  }

  fn process_tag_name_state(&mut self) -> Option<Token> {
    let bytes =
      self.read_to_many(&[b'/', b'>', b'\0', b'\t', b'\n', b' ', b'\x0C']);

    if bytes.iter().all(|&b| b.is_ascii_alphanumeric()) {
      self.set_tag_name(bytes);
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      warn!("eof-in-tag");
      return Some(self.emit_eof());
    }

    let c = self.read_current();
    self.stream.advance();

    match c {
      b'/' => {
        unimplemented!("undefined State::SelfClosingStartTag");
      }
      b'>' => {
        self.switch_to(State::Data);
        return Some(self.emit_current_token());
      }
      b'\0' => {
        warn!("unexpected-null-character");
        self.append_replacement_char_to_tag_name();
      }
      b if b.is_ascii_whitespace() => {
        self.switch_to(State::BeforeAttributeName);
      }
      _ => {
        noop!();
      }
    }

    None
  }

  fn process_end_tag_open_state(&mut self) -> Option<Token> {
    let c = self.read_next();

    if c.is_ascii_alphanumeric() {
      self.new_token(Token::new_end_tag());
      self.switch_to(State::TagName);
      return None;
    }

    match c {
      b'>' => {
        warn!("missing-end-tag-name");
        self.switch_to(State::Data);
      }
      _ if self.stream.is_eof() => {
        warn!("eof-before-tag-name");
        self.will_emit(Token::new_text("</"));
        return Some(self.emit_eof());
      }
      _ => {
        warn!("invalid-first-character-of-tag-name");
        unimplemented!("undefined State::BogusComment");
      }
    }

    None
  }

  fn process_before_attribute_name_state(&mut self) -> Option<Token> {
    let c = self.read_next_skipped_whitespace();

    match c {
      b'/' | b'>' => {
        self.reconsume_in_state(State::AfterAttributeName);
      }
      _ if self.stream.is_eof() => {
        self.reconsume_in_state(State::AfterAttributeName);
      }
      b'=' => {
        warn!("unexpected-equals-sign-before-attribute-name");
        self.switch_to(State::AttributeName);
      }
      _ => {
        self.reconsume_in_state(State::AttributeName);
      }
    }

    None
  }

  fn process_attribute_name_state(&mut self) -> Option<Token> {
    todo!("process_attribute_name_state");
  }

  fn process_after_attribute_name_state(&mut self) -> Option<Token> {
    todo!("process_after_attribute_name_state");
  }

  fn process_before_attribute_value_state(&mut self) -> Option<Token> {
    todo!("process_before_attribute_value_state");
  }

  fn process_attribute_value_double_quoted_state(&mut self) -> Option<Token> {
    todo!("process_attribute_value_double_quoted_state");
  }

  fn process_attribute_value_single_quoted_state(&mut self) -> Option<Token> {
    todo!("process_attribute_value_single_quoted_state");
  }

  fn process_attribute_value_unquoted_state(&mut self) -> Option<Token> {
    todo!("process_attribute_value_unquoted_state");
  }

  fn process_after_attribute_value_quoted_state(&mut self) -> Option<Token> {
    todo!("process_after_attribute_value_quoted_state");
  }

  /* -------------------------------------------- */

  fn reconsume_in_state(&mut self, state: State) {
    self.stream.reconsume();
    self.switch_to(state);
  }

  /* -------------------------------------------- */

  fn new_token(&mut self, token: Token) {
    self.current_token = Some(token);
  }

  /* -------------------------------------------- */

  fn append_replacement_char_to_tag_name(&mut self) {
    let current_tag = self.current_token.as_mut().unwrap();
    match current_tag {
      Token::Tag { tag_name, .. } => {
        tag_name.push(REPLACEMENT_CHARACTER);
      }
      _ => unreachable!("No tag found"),
    }
  }

  fn set_tag_name(&mut self, name: &[u8]) {
    let name = bytes_to_string(name);
    let current_tag = self.current_token.as_mut().unwrap();
    match current_tag {
      Token::Tag { tag_name, .. } => {
        *tag_name = name;
      }
      _ => unreachable!("No tag found"),
    }
  }

  /* -------------------------------------------- */

  fn get_duplicate_attribute_index(
    &self,
    attributes: &EcoVec<Attribute>,
  ) -> EcoVec<usize> {
    let mut seen = HashSet::new();
    let mut remove_indexes = EcoVec::new();

    for (index, attribute) in attributes.iter().enumerate() {
      if seen.contains(&attribute.name) {
        warn!("duplicate-attribute");
        remove_indexes.push(index);
      } else {
        seen.insert(attribute.name.clone());
      }
    }

    remove_indexes
  }

  fn will_emit(&mut self, token: Token) {
    let mut token = token;
    if let Token::Tag {
      is_end_tag,
      ref mut attributes,
      ..
    } = token
    {
      if !attributes.is_empty() {
        for index in self.get_duplicate_attribute_index(attributes) {
          attributes.remove(index);
        }
      }

      if !is_end_tag {
        self.last_emitted_start_tag = Some(token.clone());
      }
    }
    self.output.push(token);
  }

  fn emit_current_token(&mut self) -> Token {
    self.will_emit(self.current_token.clone().unwrap());
    self.output.pop().unwrap()
  }

  fn emit_text(&mut self, s: &[u8]) -> Token {
    let text = bytes_to_string(s);
    self.new_token(Token::Text(text));
    self.emit_current_token()
  }

  fn emit_eof(&mut self) -> Token {
    self.new_token(Token::EOF);
    self.emit_current_token()
  }

  /* -------------------------------------------- */

  fn read_current(&mut self) -> u8 {
    self.stream.current_cpy().unwrap()
  }

  fn read_next(&mut self) -> u8 {
    self.stream.advance();
    self.stream.current_cpy().unwrap()
  }

  fn read_next_skipped_whitespace(&mut self) -> u8 {
    self.stream.advance();
    self.stream.skip_while(|b| b.is_ascii_whitespace());
    self.stream.current_cpy().unwrap()
  }

  fn read_to(&mut self, c: u8) -> &'a [u8] {
    let start = self.stream.idx;
    let bytes = &self.stream.data()[start..];

    let end =
      bytes.iter().position(|&b| b == c).unwrap_or(self.stream.len() - start);

    self.stream.advance_by(end);
    self.stream.slice(start, start + end)
  }

  // cに遭遇するまで読み進める
  // cを含む位置を返すので注意
  fn read_to_many(&mut self, c: &[u8]) -> &'a [u8] {
    let start = self.stream.idx;
    let bytes = &self.stream.data()[start..];

    let end = bytes
      .iter()
      .position(|&b| c.contains(&b))
      .unwrap_or(self.stream.len() - start);

    self.stream.advance_by(end);
    self.stream.slice(start, start + end)
  }
}
