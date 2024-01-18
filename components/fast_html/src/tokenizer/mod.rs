mod byte_string;
mod state;
mod stream;
pub mod token;

use self::byte_string::*;
use self::state::State;
use self::stream::Stream;
use self::token::Token;

use log::{debug, warn};

use ecow::{EcoString, EcoVec};

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

    if self.stream.is_eof() {
      return Some(self.emit_eof());
    }

    if !bytes.is_empty() {
      return Some(self.emit_text(&bytes));
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
        return Some(self.emit_text(&bytes));
      }
      _ => {
        noop!();
      }
    }

    return None;
  }

  fn process_tag_open_state(&mut self) -> Option<Token> {
    let c = self.read_next();

    if self.stream.is_eof() {
      warn!("eof-before-tag-name");
      self.will_emit(Token::Text(EcoString::from("<")));
      return Some(self.emit_eof());
    }

    if is_ascii_alphanumeric(c) {
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
      _ => {
        warn!("invalid-first-character-of-tag-name");
        self.will_emit(Token::new_text("<"));
        self.reconsume_in_state(State::Data);
      }
    }

    return None;
  }

  fn process_tag_name_state(&mut self) -> Option<Token> {
    let bytes =
      self.read_to_many(&[b'/', b'>', b'\0', b'\t', b'\n', b' ', b'\x0C']);

    if bytes.iter().all(|&b| is_ascii_alphanumeric(b)) {
      self.set_tag_name(&bytes);
    }

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
      b if is_whitespace(b) => {
        unimplemented!("undefined State::BeforeAttributeName");
      }
      _ => {
        noop!();
      }
    }

    None
  }

  fn process_end_tag_open_state(&mut self) -> Option<Token> {
    let c = self.read_next();

    if self.stream.is_eof() {
      warn!("eof-before-tag-name");
      self.will_emit(Token::new_text("</"));
      return Some(self.emit_eof());
    }

    if is_ascii_alphanumeric(c) {
      self.new_token(Token::new_end_tag());
      self.switch_to(State::TagName);
      return None;
    }

    match c {
      b'>' => {
        warn!("missing-end-tag-name");
        self.switch_to(State::Data);
      }
      _ => {
        warn!("invalid-first-character-of-tag-name");
        unimplemented!("undefined State::BogusComment");
      }
    }

    None
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

  fn will_emit(&mut self, token: Token) {
    let token = token;
    // todo: remove duplicate attribute
    if let Token::Tag { is_end_tag, .. } = token {
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
