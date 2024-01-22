mod byte_string;
pub mod state;
mod stream;
pub mod token;

use std::collections::HashSet;
use std::str::from_utf8;

use self::byte_string::*;
use self::state::State;
use self::stream::Stream;
use self::token::Attribute;
use self::token::Token;

use log::{debug, trace, warn};

use ecow::EcoString;
use ecow::EcoVec;

const REPLACEMENT_CHARACTER: char = '\u{FFFD}';

pub struct Tokenizer<'a> {
  state: State,
  return_state: Option<State>,
  stream: Stream<'a, u8>,
  output: EcoVec<Token>,
  current_token: Option<Token>,
  last_emitted_start_tag: Option<Token>,
  tmp_buffer: EcoVec<u8>,
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
      tmp_buffer: EcoVec::new(),
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
        State::SelfClosingStartTag => {
          self.process_self_closing_start_tag_state()
        }
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
        State::MarkupDeclarationOpen => {
          self.process_markup_declaration_open_state()
        }
        State::DOCTYPE => self.process_doctype_state(),
        State::BeforeDOCTYPEName => self.process_before_doctype_name_state(),
        State::DOCTYPEName => self.process_doctype_name_state(),
        State::AfterDOCTYPEName => self.process_after_doctype_name_state(),
        State::BogusDOCTYPE => self.process_bogus_doctype_state(),
        State::RAWTEXT => self.process_rawtext_state(),
        State::RAWTEXTLessThanSign => {
          self.process_rawtext_less_than_sign_state()
        }
        State::RAWTEXTEndTagOpen => self.process_rawtext_end_tag_open_state(),
        State::RAWTEXTEndTagName => self.process_rawtext_end_tag_name_state(),
        State::RCDATA => self.process_rcdata_state(),
        State::RCDATALessThanSign => self.process_rcdata_less_than_sign_state(),
        State::RCDATAEndTagOpen => self.process_rcdata_end_tag_open_state(),
        State::RCDATAEndTagName => self.process_rcdata_end_tag_name_state(),
      };

      if let Some(token) = token {
        return token;
      }
    }
  }

  /* -------------------------------------------- */

  pub fn switch_to(&mut self, state: State) {
    debug!("Tokenizer State: switch to {:#?}", state);
    self.state = state;
    self.stream.advance();
  }

  pub fn reconsume_in(&mut self, state: State) {
    debug!("Tokenizer State: reconsume in {:#?}", state);
    self.state = state;
  }

  /* -------------------------------------------- */

  fn process_data_state(&mut self) -> Option<Token> {
    let bytes = self.read_to_oneof(&[b'<', b'&', b'\0']);

    trace!("-- Data: {}", bytes_to_string(bytes));

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
        // noop
      }
    }

    None
  }

  fn process_tag_open_state(&mut self) -> Option<Token> {
    let c = self.read_current();

    trace!("-- TagOpen: {}", c as char);

    if c.is_ascii_alphanumeric() {
      self.new_token(Token::new_start_tag());
      self.reconsume_in(State::TagName);
      return None;
    }

    match c {
      b'!' => {
        self.switch_to(State::MarkupDeclarationOpen);
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
        self.reconsume_in(State::Data);
      }
    }

    None
  }

  fn process_tag_name_state(&mut self) -> Option<Token> {
    let bytes =
      self.read_to_oneof(&[b'/', b'>', b'\0', b'\t', b'\n', b' ', b'\x0C']);

    trace!("-- TagName: {}", bytes_to_string(bytes));

    if bytes.iter().all(|&b| b.is_ascii_alphanumeric()) {
      self.set_tag_name(bytes);
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      warn!("eof-in-tag");
      return Some(self.emit_eof());
    }

    let c = self.read_current();

    match c {
      b'/' => {
        self.switch_to(State::SelfClosingStartTag);
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
        // noop
      }
    }

    None
  }

  fn process_end_tag_open_state(&mut self) -> Option<Token> {
    let c = self.read_current();

    trace!("-- EndTagOpen: {}", c as char);

    if c.is_ascii_alphanumeric() {
      self.new_token(Token::new_end_tag());
      self.reconsume_in(State::TagName);
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

  fn process_self_closing_start_tag_state(&mut self) -> Option<Token> {
    let b = self.read_current();

    trace!("-- SelfClosingStartTag: {}", b as char);

    match b {
      b'>' => {
        let current_token = self.current_token.as_mut().unwrap();
        if let Token::Tag {
          ref mut self_closing,
          ..
        } = current_token
        {
          *self_closing = true;
        }
        self.switch_to(State::Data);
        return Some(self.emit_current_token());
      }
      _ if self.stream.is_eof() => {
        warn!("eof-in-tag");
        return Some(self.emit_eof());
      }
      _ => {
        warn!("unexpected-solidus-in-tag");
        self.reconsume_in(State::BeforeAttributeName);
      }
    }

    None
  }

  fn process_before_attribute_name_state(&mut self) -> Option<Token> {
    let c = self.read_current_skipped_whitespace();

    trace!("-- BeforeAttributeName: {}", c as char);

    match c {
      b'/' | b'>' => {
        self.reconsume_in(State::AfterAttributeName);
      }
      _ if self.stream.is_eof() => {
        self.reconsume_in(State::AfterAttributeName);
      }
      b'=' => {
        warn!("unexpected-equals-sign-before-attribute-name");
        self.switch_to(State::AttributeName);
      }
      _ => {
        self.reconsume_in(State::AttributeName);
      }
    }

    None
  }

  fn process_attribute_name_state(&mut self) -> Option<Token> {
    let bytes = self.read_to_whitespace_or_oneof(&[
      b'/', b'>', b'=', b'\0', b'"', b'\'', b'<',
    ]);

    trace!("-- AttributeName: {}", bytes_to_string(bytes));

    if !bytes.is_empty() {
      self.set_attribute_name(bytes);
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      self.switch_to(State::AfterAttributeName);
      return None;
    }

    let c = self.read_current();

    match c {
      _ if c.is_ascii_whitespace() => {
        self.reconsume_in(State::AfterAttributeName);
      }
      b'/' | b'>' => {
        self.reconsume_in(State::AfterAttributeName);
      }
      b'=' => {
        self.switch_to(State::BeforeAttributeValue);
      }
      b'\0' => {
        warn!("unexpected-null-character");
        self.append_char_to_attribute_name(REPLACEMENT_CHARACTER);
      }
      b'"' | b'\'' | b'<' => {
        warn!("unexpected-character-in-attribute-name");
        self.append_char_to_attribute_name(c as char);
      }
      _ => {
        // noop
      }
    }

    None
  }

  fn process_after_attribute_name_state(&mut self) -> Option<Token> {
    let b = self.read_current_skipped_whitespace();

    trace!("-- AfterAttributeName: {}", b as char);

    match b {
      b'/' => {
        self.switch_to(State::SelfClosingStartTag);
      }
      b'=' => {
        self.switch_to(State::BeforeAttributeValue);
      }
      b'>' => {
        self.switch_to(State::Data);
        return Some(self.emit_current_token());
      }
      _ if self.stream.is_eof() => {
        warn!("eof-in-tag");
        return Some(self.emit_eof());
      }
      _ => {
        self.reconsume_in(State::AttributeName);
      }
    }

    None
  }

  fn process_before_attribute_value_state(&mut self) -> Option<Token> {
    let c = self.read_current_skipped_whitespace();

    trace!("-- BeforeAttributeValue: {}", c as char);

    match c {
      b'"' => {
        self.switch_to(State::AttributeValueDoubleQuoted);
      }
      b'\'' => {
        self.switch_to(State::AttributeValueSingleQuoted);
      }
      b'>' => {
        warn!("missing-attribute-value");
        self.switch_to(State::Data);
        return Some(self.emit_current_token());
      }
      _ => {
        self.reconsume_in(State::AttributeValueUnQuoted);
      }
    }

    None
  }

  fn process_attribute_value_double_quoted_state(&mut self) -> Option<Token> {
    let bytes = self.read_to_oneof(&[b'"', b'&', b'\0']);

    trace!("-- AttributeValueDoubleQuoted: {}", bytes_to_string(bytes));

    if !bytes.is_empty() {
      self.set_attribute_value(bytes);
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      warn!("eof-in-tag");
      return Some(self.emit_eof());
    }

    let c = self.read_current();

    match c {
      b'"' => {
        self.switch_to(State::AfterAttributeValueQuoted);
      }
      b'&' => {
        self.return_state = Some(State::AttributeValueDoubleQuoted);
        unimplemented!("self.switch_to(State::CharacterReference);");
      }
      b'\0' => {
        warn!("unexpected-null-character");
        self.append_char_to_attribute_value(REPLACEMENT_CHARACTER);
      }
      _ => {
        // noop
      }
    }

    None
  }

  fn process_attribute_value_single_quoted_state(&mut self) -> Option<Token> {
    todo!("process_attribute_value_single_quoted_state");
  }

  fn process_attribute_value_unquoted_state(&mut self) -> Option<Token> {
    todo!("process_attribute_value_unquoted_state");
  }

  fn process_after_attribute_value_quoted_state(&mut self) -> Option<Token> {
    let c = self.read_current();

    trace!("-- AfterAttributeValueQuoted: {}", c as char);

    match c {
      _ if c.is_ascii_whitespace() => {
        self.switch_to(State::BeforeAttributeName);
      }
      b'/' => {
        self.switch_to(State::SelfClosingStartTag);
      }
      b'>' => {
        self.switch_to(State::Data);
        return Some(self.emit_current_token());
      }
      _ if self.stream.is_eof() => {
        warn!("eof-in-tag");
        return Some(self.emit_eof());
      }
      _ => {
        warn!("missing-whitespace-between-attributes");
        self.reconsume_in(State::BeforeAttributeName);
      }
    }

    None
  }

  fn process_markup_declaration_open_state(&mut self) -> Option<Token> {
    if self.read_if_match(b"--", false) {
      unimplemented!("self.switch_to(State::CommentStart);");
    }

    if self.read_if_match(b"DOCTYPE", true) {
      self.switch_to(State::DOCTYPE);
      return None;
    }

    if self.read_if_match(b"[CDATA[", false) {
      unimplemented!("self.switch_to(State::CDATASection);");
    }

    warn!("incorrectly-opened-comment");
    unimplemented!("self.switch_to(State::BogusComment);");
  }

  fn process_doctype_state(&mut self) -> Option<Token> {
    let c = self.read_current();

    trace!("-- DOCTYPE: {}", c as char);

    match c {
      _ if c.is_ascii_whitespace() => {
        self.switch_to(State::BeforeDOCTYPEName);
      }
      b'>' => {
        self.reconsume_in(State::BeforeDOCTYPEName);
      }
      _ if self.stream.is_eof() => {
        warn!("eof-in-doctype");
        let token = Token::new_doctype_with_force_quirks();
        self.new_token(token);
        self.will_emit(self.current_token.clone().unwrap());
        return Some(self.emit_eof());
      }
      _ => {
        warn!("missing-whitespace-before-doctype-name");
        self.reconsume_in(State::BeforeDOCTYPEName);
      }
    }

    None
  }

  fn process_before_doctype_name_state(&mut self) -> Option<Token> {
    let b = self.read_current_skipped_whitespace();

    trace!("-- BeforeDOCTYPEName: {}", b as char);

    match b {
      b'>' => {
        warn!("missing-doctype-name");
        let token = Token::new_doctype_with_force_quirks();
        self.new_token(token);
        self.switch_to(State::Data);
        return Some(self.emit_current_token());
      }
      b'\0' => {
        warn!("unexpected-null-character");
        let token = Token::new_doctype_char_of(REPLACEMENT_CHARACTER);
        self.new_token(token);
        self.switch_to(State::DOCTYPEName);
      }
      _ if self.stream.is_eof() => {
        warn!("eof-in-doctype");
        let token = Token::new_doctype_with_force_quirks();
        self.new_token(token);
        self.will_emit(self.current_token.clone().unwrap());
        return Some(self.emit_eof());
      }
      _ => {
        let token = Token::new_doctype_char_of(b as char);
        self.new_token(token);
        self.switch_to(State::DOCTYPEName);
      }
    }

    None
  }

  fn process_doctype_name_state(&mut self) -> Option<Token> {
    let bytes = self.read_to_whitespace_or_oneof(&[b'>', b'\0']);

    trace!("-- DOCTYPEName: {}", bytes_to_string(bytes));

    if !bytes.is_empty() {
      self.concat_to_doctype_name(bytes);
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      warn!("eof-in-doctype");
      let mut token = self.current_token.clone().unwrap();
      token.set_force_quirks(true);
      self.will_emit(token);
      return Some(self.emit_eof());
    }

    let b = self.read_current();

    match b {
      _ if b.is_ascii_whitespace() => {
        self.switch_to(State::AfterDOCTYPEName);
      }
      b'>' => {
        self.switch_to(State::Data);
        return Some(self.emit_current_token());
      }
      b'\0' => {
        warn!("unexpected-null-character");
        self.append_char_to_doctype_name(REPLACEMENT_CHARACTER);
      }
      _ => {
        // noop
      }
    }

    None
  }

  fn process_after_doctype_name_state(&mut self) -> Option<Token> {
    todo!("process_after_doctype_name_state");
  }

  fn process_bogus_doctype_state(&mut self) -> Option<Token> {
    todo!("process_bogus_doctype_state");
  }

  fn process_rawtext_state(&mut self) -> Option<Token> {
    let bytes = self.read_to_oneof(&[b'<', b'\0']);

    trace!("-- RAWTEXT: {}", bytes_to_string(bytes));

    if !bytes.is_empty() {
      return Some(self.emit_text(bytes));
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      return Some(self.emit_eof());
    }

    let b = self.read_current();

    match b {
      b'<' => {
        self.switch_to(State::RAWTEXTLessThanSign);
      }
      b'\0' => {
        warn!("unexpected-null-character");
        return Some(self.emit_char(REPLACEMENT_CHARACTER));
      }
      _ => {
        // noop
      }
    }

    None
  }

  fn process_rawtext_less_than_sign_state(&mut self) -> Option<Token> {
    todo!("process_rawtext_less_than_sign_state");
  }

  fn process_rawtext_end_tag_open_state(&mut self) -> Option<Token> {
    todo!("process_rawtext_end_tag_open_state");
  }

  fn process_rawtext_end_tag_name_state(&mut self) -> Option<Token> {
    todo!("process_rawtext_end_tag_name_state");
  }

  fn process_rcdata_state(&mut self) -> Option<Token> {
    let bytes = self.read_to_oneof(&[b'&', b'<', b'\0']);

    trace!("-- RCDATA: {}", bytes_to_string(bytes));

    if !bytes.is_empty() {
      return Some(self.emit_text(bytes));
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      return Some(self.emit_eof());
    }

    let b = self.read_current();

    match b {
      b'&' => {
        self.return_state = Some(State::RCDATA);
        unimplemented!("self.switch_to(State::CharacterReference);");
      }
      b'<' => {
        self.switch_to(State::RCDATALessThanSign);
      }
      b'\0' => {
        warn!("unexpected-null-character");
        return Some(self.emit_char(REPLACEMENT_CHARACTER));
      }
      _ => {
        // noop
      }
    }

    None
  }

  fn process_rcdata_less_than_sign_state(&mut self) -> Option<Token> {
    let b = self.read_current();

    trace!("-- RCDATALessThanSign: {}", b as char);

    match b {
      b'/' => {
        self.clear_tmp_buffer();
        self.switch_to(State::RCDATAEndTagOpen);
      }
      _ => {
        self.will_emit(Token::new_text("<"));
        self.reconsume_in(State::RCDATA);
      }
    }

    None
  }

  fn process_rcdata_end_tag_open_state(&mut self) -> Option<Token> {
    let b = self.read_current();

    trace!("-- RCDATAEndTagOpen: {}", b as char);

    match b {
      _ if b.is_ascii_alphabetic() => {
        self.new_token(Token::new_end_tag());
        self.reconsume_in(State::RCDATAEndTagName);
      }
      _ => {
        self.will_emit(Token::new_text("</"));
        self.reconsume_in(State::RCDATA);
      }
    }

    None
  }

  fn process_rcdata_end_tag_name_state(&mut self) -> Option<Token> {
    let bytes = self.read_to_whitespace_or_oneof(&[b'/', b'>', b'\0']);

    trace!("-- RCDATAEndTagName: {}", bytes_to_string(bytes));

    if !bytes.is_empty() {
      self.concat_to_tag_name(bytes);
      self.push_many_to_tmp_buffer(bytes);
    }

    fn invalid(this: &mut Tokenizer<'_>) {
      this.will_emit(Token::new_text("</"));
      this.emit_tmp_buffer();
      this.reconsume_in(State::RCDATA);
    }

    // read_currentに進む前にEOFチェック
    if self.stream.is_eof() {
      invalid(self);
      return None;
    }

    let b = self.read_current();

    match b {
      b'/' => {
        if !self.is_appropriate_end_tag() {
          invalid(self);
        } else {
          self.switch_to(State::SelfClosingStartTag);
        }
      }
      b'>' => {
        if !self.is_appropriate_end_tag() {
          invalid(self);
        } else {
          self.switch_to(State::Data);
          return Some(self.emit_current_token());
        }
      }
      _ if b.is_ascii_whitespace() => {
        if !self.is_appropriate_end_tag() {
          invalid(self);
        } else {
          self.switch_to(State::BeforeAttributeName);
        }
      }
      _ => {
        invalid(self);
      }
    }

    None
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

  fn append_char_to_attribute_name(&mut self, c: char) {
    let current_tag = self.current_token.as_mut().unwrap();
    if let Token::Tag { attributes, .. } = current_tag {
      if let Some(mut last) = attributes.pop() {
        last.name.push(c);
        attributes.push(last);
      }
    }
  }

  fn append_char_to_attribute_value(&mut self, c: char) {
    let current_tag = self.current_token.as_mut().unwrap();
    if let Token::Tag { attributes, .. } = current_tag {
      if let Some(mut last) = attributes.pop() {
        last.value.push(c);
        attributes.push(last);
      }
    }
  }

  fn append_char_to_doctype_name(&mut self, c: char) {
    let current_tag = self.current_token.as_mut().unwrap();
    if let Token::DOCTYPE {
      name: ref mut old_name,
      ..
    } = current_tag
    {
      if let Some(ref mut old_name) = old_name {
        old_name.push(c);
      }
    }
  }

  fn concat_to_tag_name(&mut self, suffix: &[u8]) {
    let suffix = bytes_to_string(suffix);
    let current_tag = self.current_token.as_mut().unwrap();
    match current_tag {
      Token::Tag { tag_name, .. } => {
        tag_name.push_str(&suffix);
      }
      _ => unreachable!("No tag found"),
    }
  }

  fn concat_to_doctype_name(&mut self, name: &[u8]) {
    let name = bytes_to_string(name);
    let current_tag = self.current_token.as_mut().unwrap();
    if let Token::DOCTYPE {
      name: ref mut old_name,
      ..
    } = current_tag
    {
      if let Some(ref mut old_name) = old_name {
        old_name.push_str(&name);
      }
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

  fn set_attribute_name(&mut self, name: &[u8]) {
    let name = from_utf8(name).unwrap();
    let current_tag = self.current_token.as_mut().unwrap();
    match current_tag {
      Token::Tag { attributes, .. } => {
        attributes.push(Attribute::new_name_of(name));
      }
      _ => unreachable!("No tag found"),
    }
  }

  fn set_attribute_value(&mut self, value: &[u8]) {
    let value = bytes_to_string(value);
    let current_tag = self.current_token.as_mut().unwrap();
    match current_tag {
      Token::Tag { attributes, .. } => {
        if let Some(mut last) = attributes.pop() {
          last.value = value;
          attributes.push(last);
        }
      }
      _ => unreachable!("No tag found"),
    }
  }

  /* -------------------------------------------- */

  fn is_appropriate_end_tag(&mut self) -> bool {
    if self.last_emitted_start_tag.is_none() {
      return false;
    }

    let current_tag = self.current_token.as_ref().unwrap();
    let last_start_tag = self.last_emitted_start_tag.as_ref().unwrap();

    if let Token::Tag { tag_name, .. } = current_tag {
      let current_tag_name = tag_name;

      if let Token::Tag { tag_name, .. } = last_start_tag {
        let last_tag_name = tag_name;

        return current_tag_name == last_tag_name;
      }
    }

    false
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

  fn emit_char(&mut self, c: char) -> Token {
    self.new_token(Token::Text(EcoString::from(c)));
    self.emit_current_token()
  }

  fn emit_eof(&mut self) -> Token {
    self.new_token(Token::EOF);
    self.emit_current_token()
  }

  fn emit_tmp_buffer(&mut self) {
    self.output.push(Token::Text(bytes_to_string(&self.tmp_buffer)));
  }

  /* tmp buffer --------------------------------- */

  fn clear_tmp_buffer(&mut self) {
    self.tmp_buffer.clear();
    trace!("-- tmp_buffer_clear");
  }

  fn push_many_to_tmp_buffer(&mut self, bytes: &[u8]) {
    self.tmp_buffer.extend_from_slice(bytes);
    trace!(
      "-- tmp_buffer: {:?}",
      bytes_to_string(self.tmp_buffer.as_slice())
    );
  }

  /* -------------------------------------------- */

  fn read_current(&mut self) -> u8 {
    self.stream.current_cpy().unwrap()
  }

  fn read_current_skipped_whitespace(&mut self) -> u8 {
    let start = self.stream.idx;
    let rest = &self.stream.data()[start..];

    let end = rest
      .iter()
      .position(|&b| !b.is_ascii_whitespace())
      .unwrap_or(self.stream.len() - start);

    self.stream.advance_by(end);
    self.stream.current_cpy().unwrap()
  }

  // patternに合致するなら、進める
  // 合致して進めた場合、trueを返す
  fn read_if_match(&mut self, pattern: &[u8], ignore_case: bool) -> bool {
    let pattern_len = pattern.len();

    let peeked = self.stream.slice_len(pattern_len);

    if ignore_case {
      let peeked =
        peeked.iter().map(|&b| b.to_ascii_lowercase()).collect::<Vec<_>>();

      let pattern =
        pattern.iter().map(|&b| b.to_ascii_lowercase()).collect::<Vec<_>>();

      if peeked == pattern {
        // for _ in 0..pattern.len() { self.stream.advance(); } のイメージ
        self.stream.advance_by(pattern_len - 1);
        return true;
      }
    } else if peeked == pattern {
      // for _ in 0..pattern.len() { self.stream.advance(); } のイメージ
      self.stream.advance_by(pattern_len - 1);
      return true;
    }

    false
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
  fn read_to_oneof(&mut self, c: &[u8]) -> &'a [u8] {
    let start = self.stream.idx;
    let bytes = &self.stream.data()[start..];

    let end = bytes
      .iter()
      .position(|&b| c.contains(&b))
      .unwrap_or(self.stream.len() - start);

    self.stream.advance_by(end);
    self.stream.slice(start, start + end)
  }

  fn read_to_whitespace_or_oneof(&mut self, c: &[u8]) -> &'a [u8] {
    let start = self.stream.idx;
    let bytes = &self.stream.data()[start..];

    let end = bytes
      .iter()
      .position(|&b| c.contains(&b) || b.is_ascii_whitespace())
      .unwrap_or(self.stream.len() - start);

    self.stream.advance_by(end);
    self.stream.slice(start, start + end)
  }
}
