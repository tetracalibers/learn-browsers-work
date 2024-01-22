pub mod state;
pub mod token;

use ecow::EcoString;
use ecow::EcoVec;

use std::collections::HashSet;
use std::collections::VecDeque;

use log::{debug, warn};

use state::State;

use token::Attribute;
use token::Token;

use stream::input_stream::CharInputStream;

const REPLACEMENT_CHARACTER: char = '\u{FFFD}';

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Char {
  ch(char),
  eof,
  null,
  whitespace,
}

pub struct Tokenizer<T>
where
  T: Iterator<Item = char>,
{
  input: CharInputStream<T>,
  output: VecDeque<Token>,

  state: State,
  return_state: Option<State>,

  current_character: char,
  reconsume_char: bool,

  current_token: Option<Token>,
  last_emitted_start_tag: Option<Token>,

  tmp_buffer: EcoString,
}

pub trait Tokenizing {
  fn next_token(&mut self) -> Token;
  fn switch_to(&mut self, state: State);
}

impl<T> Tokenizing for Tokenizer<T>
where
  T: Iterator<Item = char>,
{
  fn next_token(&mut self) -> Token {
    if !self.output.is_empty() {
      return self.output.pop_front().unwrap();
    }

    loop {
      match self.state {
        State::Data => {
          let ch = self.consume_next();

          match ch {
            Char::ch('&') => {
              self.return_state = Some(State::Data);
              self.switch_to(State::CharacterReference);
            }
            Char::ch('<') => {
              self.switch_to(State::TagOpen);
            }
            Char::null => {
              warn!("unexpected-null-character");
              return self.emit_current_char();
            }
            Char::eof => {
              return self.emit_eof();
            }
            _ => {
              return self.emit_current_char();
            }
          }
        }

        State::RCDATA => {
          let ch = self.consume_next();

          match ch {
            Char::ch('&') => {
              self.return_state = Some(State::RCDATA);
              self.switch_to(State::CharacterReference);
            }
            Char::ch('<') => {
              self.switch_to(State::RCDATALessThanSign);
            }
            Char::null => {
              warn!("unexpected-null-character");
              return self.emit_char(REPLACEMENT_CHARACTER);
            }
            Char::eof => {
              return self.emit_eof();
            }
            _ => {
              return self.emit_current_char();
            }
          }
        }

        State::RAWTEXT => {
          todo!("State::RAWTEXT");
        }

        State::RAWTEXTLessThanSign => {
          todo!("State::RAWTEXTLessThanSign");
        }

        State::RAWTEXTEndTagOpen => {
          todo!("State::RAWTEXTEndTagOpen");
        }

        State::RAWTEXTEndTagName => {
          todo!("State::RAWTEXTEndTagName");
        }

        State::TagOpen => {
          let ch = self.consume_next();

          match ch {
            Char::ch('!') => {
              self.switch_to(State::MarkupDeclarationOpen);
            }
            Char::ch('/') => {
              self.switch_to(State::EndTagOpen);
            }
            Char::ch(c) if c.is_ascii_alphabetic() => {
              self.new_token(Token::new_start_tag());
              self.reconsume_in(State::TagName);
            }
            Char::ch('?') => {
              warn!("unexpected-question-mark-instead-of-tag-name");
              self.new_token(Token::new_comment(""));
              self.reconsume_in(State::BogusComment);
            }
            Char::eof => {
              warn!("eof-before-tag-name");
              self.will_emit(Token::Character('<'));
              return self.emit_eof();
            }
            _ => {
              warn!("invalid-first-character-of-tag-name");
              self.will_emit(Token::Character('<'));
              self.reconsume_in(State::Data);
            }
          }
        }

        State::EndTagOpen => {
          let ch = self.consume_next();

          match ch {
            Char::ch(c) if c.is_ascii_alphabetic() => {
              self.new_token(Token::new_end_tag());
              self.reconsume_in(State::TagName);
            }
            Char::ch('>') => {
              warn!("missing-end-tag-name");
              self.switch_to(State::Data);
            }
            Char::eof => {
              warn!("eof-before-tag-name");
              self.will_emit(Token::Character('<'));
              self.will_emit(Token::Character('/'));
              return self.emit_eof();
            }
            _ => {
              warn!("invalid-first-character-of-tag-name");
              self.new_token(Token::new_comment(""));
              self.reconsume_in(State::BogusComment);
            }
          }
        }

        State::TagName => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => {
              self.switch_to(State::BeforeAttributeName);
            }
            Char::ch('/') => {
              self.switch_to(State::SelfClosingStartTag);
            }
            Char::ch('>') => {
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::ch(c) if c.is_ascii_uppercase() => {
              self.append_char_to_tag_name(c.to_ascii_lowercase());
            }
            Char::null => {
              warn!("unexpected-null-character");
              self.append_char_to_tag_name(REPLACEMENT_CHARACTER);
            }
            Char::eof => {
              warn!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              self.append_char_to_tag_name(self.current_character);
            }
          }
        }

        State::RCDATALessThanSign => {
          let ch = self.consume_next();

          match ch {
            Char::ch('/') => {
              self.tmp_buffer.clear();
              self.switch_to(State::RCDATAEndTagOpen);
            }
            _ => {
              self.will_emit(Token::Character('<'));
              self.reconsume_in(State::RCDATA);
            }
          }
        }

        State::RCDATAEndTagOpen => {
          let ch = self.consume_next();

          match ch {
            Char::ch(c) if c.is_ascii_alphabetic() => {
              self.new_token(Token::new_end_tag());
              self.reconsume_in(State::RCDATAEndTagName);
            }
            _ => {
              self.will_emit(Token::Character('<'));
              self.will_emit(Token::Character('/'));
              self.reconsume_in(State::RCDATA);
            }
          }
        }

        State::RCDATAEndTagName => {
          let ch = self.consume_next();

          fn invalid<T: Iterator<Item = char>>(this: &mut Tokenizer<T>) {
            this.will_emit(Token::Character('<'));
            this.will_emit(Token::Character('/'));
            this.emit_tmp_buffer();
            this.reconsume_in(State::RCDATA);
          }

          match ch {
            Char::whitespace => {
              if !self.is_appropriate_end_tag() {
                invalid(self);
              } else {
                self.switch_to(State::BeforeAttributeName);
              }
            }
            Char::ch('/') => {
              if !self.is_appropriate_end_tag() {
                invalid(self);
              } else {
                self.switch_to(State::SelfClosingStartTag);
              }
            }
            Char::ch('>') => {
              if !self.is_appropriate_end_tag() {
                invalid(self);
              } else {
                self.switch_to(State::Data);
                return self.emit_current_token();
              }
            }
            Char::ch(c) if c.is_ascii_uppercase() => {
              self.append_char_to_tag_name(c.to_ascii_lowercase());
              self.tmp_buffer.push(self.current_character);
            }
            Char::ch(c) if c.is_ascii_lowercase() => {
              self.append_char_to_tag_name(self.current_character);
              self.tmp_buffer.push(self.current_character);
            }
            _ => {
              invalid(self);
            }
          }
        }

        State::SelfClosingStartTag => {
          let ch = self.consume_next();

          match ch {
            Char::ch('>') => {
              let tag = self.current_token.as_mut().unwrap();

              if let Token::Tag {
                ref mut self_closing,
                ..
              } = tag
              {
                *self_closing = true;
              }
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::eof => {
              warn!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              warn!("unexpected-solidus-in-tag");
              self.reconsume_in(State::BeforeAttributeName);
            }
          }
        }

        State::BeforeAttributeName => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => continue,
            Char::ch('/') | Char::ch('>') | Char::eof => {
              self.reconsume_in(State::AfterAttributeName);
            }
            Char::ch('=') => {
              warn!("unexpected-equals-sign-before-attribute-name");
              let mut attribute = Attribute::new();
              attribute.name.push(self.current_character);
              self.new_attribute(attribute);
              self.switch_to(State::AttributeName);
            }
            _ => {
              let attribute = Attribute::new();
              self.new_attribute(attribute);
              self.reconsume_in(State::AttributeName);
            }
          }
        }

        State::AttributeName => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace | Char::ch('/') | Char::ch('>') | Char::eof => {
              self.reconsume_in(State::AfterAttributeName);
            }
            Char::ch('=') => {
              self.switch_to(State::BeforeAttributeValue);
            }
            Char::ch(c) if c.is_ascii_uppercase() => {
              self.append_char_to_attribute_name(c.to_ascii_lowercase());
            }
            Char::null => {
              warn!("unexpected-null-character");
              self.append_char_to_attribute_name(REPLACEMENT_CHARACTER);
            }
            Char::ch('"') | Char::ch('\'') | Char::ch('<') => {
              warn!("unexpected-character-in-attribute-name");
              self.append_char_to_attribute_name(self.current_character);
            }
            _ => {
              self.append_char_to_attribute_name(self.current_character);
            }
          }
        }

        State::AfterAttributeName => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => continue,
            Char::ch('/') => {
              self.switch_to(State::SelfClosingStartTag);
            }
            Char::ch('=') => {
              self.switch_to(State::BeforeAttributeValue);
            }
            Char::ch('>') => {
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::eof => {
              warn!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              let attribute = Attribute::new();
              self.new_attribute(attribute);
              self.reconsume_in(State::AttributeName);
            }
          }
        }

        State::BeforeAttributeValue => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => continue,
            Char::ch('"') => {
              self.switch_to(State::AttributeValueDoubleQuoted);
            }
            Char::ch('\'') => {
              self.switch_to(State::AttributeValueSingleQuoted);
            }
            Char::ch('>') => {
              warn!("missing-attribute-value");
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            _ => {
              self.reconsume_in(State::AttributeValueUnQuoted);
            }
          }
        }

        State::AttributeValueDoubleQuoted => {
          let ch = self.consume_next();

          match ch {
            Char::ch('"') => {
              self.switch_to(State::AfterAttributeValueQuoted);
            }
            Char::ch('&') => {
              self.return_state = Some(State::AttributeValueDoubleQuoted);
              self.switch_to(State::CharacterReference);
            }
            Char::null => {
              warn!("unexpected-null-character");
              self.append_char_to_attribute_value(REPLACEMENT_CHARACTER);
            }
            Char::eof => {
              warn!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              self.append_char_to_attribute_value(self.current_character);
            }
          }
        }

        State::AttributeValueSingleQuoted => {
          let ch = self.consume_next();

          match ch {
            Char::ch('\'') => {
              self.switch_to(State::AfterAttributeValueQuoted);
            }
            Char::ch('&') => {
              self.return_state = Some(State::AttributeValueSingleQuoted);
              self.switch_to(State::CharacterReference);
            }
            Char::null => {
              warn!("unexpected-null-character");
              self.append_char_to_attribute_value(REPLACEMENT_CHARACTER);
            }
            Char::eof => {
              warn!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              self.append_char_to_attribute_value(self.current_character);
            }
          }
        }

        State::AttributeValueUnQuoted => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => {
              self.switch_to(State::BeforeAttributeName);
            }
            Char::ch('&') => {
              self.return_state = Some(State::AttributeValueUnQuoted);
              self.switch_to(State::CharacterReference);
            }
            Char::ch('>') => {
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::null => {
              warn!("unexpected-null-character");
              self.append_char_to_attribute_value(REPLACEMENT_CHARACTER);
            }
            Char::ch('"')
            | Char::ch('\'')
            | Char::ch('<')
            | Char::ch('=')
            | Char::ch('`') => {
              warn!("unexpected-character-in-unquoted-attribute-value");
              self.append_char_to_attribute_value(self.current_character);
            }
            Char::eof => {
              warn!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              self.append_char_to_attribute_value(self.current_character);
            }
          }
        }

        State::AfterAttributeValueQuoted => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => {
              self.switch_to(State::BeforeAttributeName);
            }
            Char::ch('/') => {
              self.switch_to(State::SelfClosingStartTag);
            }
            Char::ch('>') => {
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::eof => {
              warn!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              warn!("missing-whitespace-between-attributes");
              self.reconsume_in(State::BeforeAttributeName);
            }
          }
        }

        State::BogusComment => {
          let ch = self.consume_next();

          match ch {
            Char::ch('>') => {
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::eof => {
              self.will_emit(self.current_token.clone().unwrap());
              return self.emit_eof();
            }
            Char::null => {
              warn!("unexpected-null-character");
              self.append_char_to_token_data(REPLACEMENT_CHARACTER);
            }
            _ => {
              self.append_char_to_token_data(self.current_character);
            }
          }
        }

        State::MarkupDeclarationOpen => {
          if self.consume_if_match("--", false) {
            self.new_token(Token::new_comment(""));
            self.switch_to(State::CommentStart);
          } else if self.consume_if_match("doctype", true) {
            self.switch_to(State::DOCTYPE);
          } else if self.consume_if_match("[CDATA[", false) {
            unimplemented!("unsupported: CDATA");
          } else {
            warn!("incorrectly-opened-comment");
            self.new_token(Token::new_comment(""));
            self.switch_to(State::BogusComment);
          }
        }

        State::CommentStart => {
          todo!("State::CommentStart");
        }

        State::DOCTYPE => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => {
              self.switch_to(State::BeforeDOCTYPEName);
            }
            Char::ch('>') => {
              self.reconsume_in(State::BeforeDOCTYPEName);
            }
            Char::eof => {
              warn!("eof-in-doctype");
              let mut token = Token::new_doctype();
              token.set_force_quirks(true);
              self.new_token(token);
              self.will_emit(self.current_token.clone().unwrap());
              return self.emit_eof();
            }
            _ => {
              warn!("missing-whitespace-before-doctype-name");
              self.reconsume_in(State::BeforeDOCTYPEName);
            }
          }
        }

        State::BeforeDOCTYPEName => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => continue,
            Char::ch(c) if c.is_ascii_uppercase() => {
              let mut token = Token::new_doctype();
              token.set_doctype_name_from_char(c.to_ascii_lowercase());
              self.new_token(token);
              self.switch_to(State::DOCTYPEName);
            }
            Char::null => {
              warn!("unexpected-null-character");
              let mut token = Token::new_doctype();
              token.set_doctype_name_from_char(REPLACEMENT_CHARACTER);
              self.new_token(token);
              self.switch_to(State::DOCTYPEName);
            }
            Char::ch('>') => {
              warn!("missing-doctype-name");
              let mut token = Token::new_doctype();
              token.set_force_quirks(true);
              self.new_token(token);
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::eof => {
              warn!("eof-in-doctype");
              let mut token = Token::new_doctype();
              token.set_force_quirks(true);
              self.new_token(token);
              self.will_emit(self.current_token.clone().unwrap());
              return self.emit_eof();
            }
            _ => {
              let mut token = Token::new_doctype();
              token.set_doctype_name_from_char(self.current_character);
              self.new_token(token);
              self.switch_to(State::DOCTYPEName);
            }
          }
        }

        State::DOCTYPEName => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => {
              self.switch_to(State::AfterDOCTYPEName);
            }
            Char::ch('>') => {
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::ch(c) if c.is_ascii_uppercase() => {
              self.append_char_to_doctype_name(c.to_ascii_lowercase());
            }
            Char::null => {
              warn!("unexpected-null-character");
              self.append_char_to_doctype_name(REPLACEMENT_CHARACTER);
            }
            Char::eof => {
              warn!("eof-in-doctype");
              let token = self.current_token.as_mut().unwrap();
              token.set_force_quirks(true);
              self.will_emit(self.current_token.clone().unwrap());
              return self.emit_eof();
            }
            _ => {
              self.append_char_to_doctype_name(self.current_character);
            }
          }
        }

        State::AfterDOCTYPEName => {
          let ch = self.consume_next();

          match ch {
            Char::whitespace => continue,
            Char::ch('>') => {
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::eof => {
              warn!("eof-in-doctype");
              let token = self.current_token.as_mut().unwrap();
              token.set_force_quirks(true);
              self.will_emit(self.current_token.clone().unwrap());
              return self.emit_eof();
            }
            _ => {
              // 本来ならここで PUBLIC や SYSTEM などの識別子を読み取り、適切な状態に遷移する
              // が、このパーサーでは対応しない

              warn!("invalid-character-sequence-after-doctype-name");
              let token = self.current_token.as_mut().unwrap();
              token.set_force_quirks(true);
              self.reconsume_in(State::BogusDOCTYPE);
            }
          }
        }

        State::BogusDOCTYPE => {
          let ch = self.consume_next();

          match ch {
            Char::ch('>') => {
              self.switch_to(State::Data);
              return self.emit_current_token();
            }
            Char::null => {
              warn!("unexpected-null-character");
              continue;
            }
            Char::eof => {
              self.will_emit(self.current_token.clone().unwrap());
              return self.emit_eof();
            }
            _ => continue,
          }
        }

        State::CharacterReference => {
          let ch = self.consume_next();

          self.tmp_buffer.clear();
          self.tmp_buffer.push('&');

          match ch {
            Char::ch(c) if c.is_ascii_alphabetic() => {
              self.reconsume_in(State::NamedCharacterReference);
            }
            Char::ch('#') => {
              self.tmp_buffer.push(self.current_character);
              self.switch_to(State::NumericCharacterReference);
            }
            _ => {
              self.flush_code_points_consumed_as_a_character_reference();
              self.reconsume_in_return_state();
            }
          }
        }

        State::NamedCharacterReference => {
          todo!("State::NamedCharacterReference")
        }

        State::NumericCharacterReference => {
          todo!("State::NumericCharacterReference")
        }
      }
    }
  }

  /* state -------------------------------------- */

  fn switch_to(&mut self, state: State) {
    debug!("Tokenizer State: switch to {:#?}", state);
    self.state = state;
  }
}

impl<T> Tokenizer<T>
where
  T: Iterator<Item = char>,
{
  pub fn new(input: T) -> Self {
    Self {
      input: CharInputStream::new(input),
      output: VecDeque::new(),

      state: State::Data,
      return_state: None,

      current_character: '\0',
      reconsume_char: false,

      current_token: None,
      last_emitted_start_tag: None,

      tmp_buffer: EcoString::new(),
    }
  }

  /* consume char ------------------------------- */

  fn consume_next(&mut self) -> Char {
    let ch = if self.reconsume_char {
      self.reconsume_char = false;
      Some(self.current_character)
    } else {
      self.input.next()
    };

    match ch {
      Some(ch) => {
        self.current_character = ch;

        match ch {
          '\0' => Char::null,
          c if c.is_whitespace() => Char::whitespace,
          _ => Char::ch(ch),
        }
      }
      None => Char::eof,
    }
  }

  fn consume_if_match(
    &mut self,
    pattern: &str,
    case_insensitive: bool,
  ) -> bool {
    let mut current_str = self.input.peek_max().iter().collect::<String>();
    let mut pattern = pattern.to_owned();

    if case_insensitive {
      current_str = current_str.to_ascii_lowercase();
      pattern = pattern.to_ascii_lowercase();
    }

    if current_str.starts_with(&pattern) {
      for _ in 0..pattern.len() {
        self.consume_next();
      }
      return true;
    }

    false
  }

  fn reconsume_in(&mut self, state: State) {
    self.reconsume_char = true;
    self.switch_to(state);
  }

  fn reconsume_in_return_state(&mut self) {
    self.reconsume_in(self.return_state.clone().unwrap());
  }

  /* string utilities --------------------------- */

  fn append_char_to_tag_name(&mut self, ch: char) {
    let current_tag = self.current_token.as_mut().unwrap();
    match current_tag {
      Token::Tag { tag_name, .. } => {
        tag_name.push(ch);
      }
      _ => unreachable!("No tag found"),
    }
  }

  fn append_char_to_token_data(&mut self, ch: char) {
    let current_tag = self.current_token.as_mut().unwrap();
    match current_tag {
      Token::Comment(data) => {
        data.push(ch);
      }
      _ => unreachable!("No comment found"),
    }
  }

  fn append_char_to_attribute_name(&mut self, ch: char) {
    let current_tag = self.current_token.as_mut().unwrap();
    if let Token::Tag {
      ref mut attributes, ..
    } = current_tag
    {
      let attribute = attributes.pop();
      if let Some(attribute) = attribute {
        let new_name = attribute.name + EcoString::from(ch);
        attributes.push(Attribute {
          name: new_name,
          ..attribute
        });
      }
    }
  }

  fn append_char_to_attribute_value(&mut self, ch: char) {
    let current_tag = self.current_token.as_mut().unwrap();
    if let Token::Tag {
      ref mut attributes, ..
    } = current_tag
    {
      let attribute = attributes.pop();
      if let Some(attribute) = attribute {
        let new_value = attribute.value + EcoString::from(ch);
        attributes.push(Attribute {
          value: new_value,
          ..attribute
        });
      }
    }
  }

  fn append_char_to_doctype_name(&mut self, ch: char) {
    let token = self.current_token.as_mut().unwrap();
    if let Token::DOCTYPE { ref mut name, .. } = token {
      name.as_mut().unwrap().push(ch);
    }
  }

  /* checker ------------------------------------ */

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

  /* token -------------------------------------- */

  fn new_token(&mut self, token: Token) {
    self.current_token = Some(token);
  }

  fn new_attribute(&mut self, attr: Attribute) {
    let token = self.current_token.as_mut().unwrap();
    if let Token::Tag {
      ref mut attributes, ..
    } = token
    {
      attributes.push(attr);
    }
  }

  /* emit token --------------------------------- */

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
      for index in self.get_duplicate_attribute_index(attributes) {
        attributes.remove(index);
      }

      if !is_end_tag {
        self.last_emitted_start_tag = Some(token.clone());
      }
    }

    self.output.push_back(token);
  }

  fn emit_current_token(&mut self) -> Token {
    self.will_emit(self.current_token.clone().unwrap());
    self.output.pop_front().unwrap()
  }

  fn emit_char(&mut self, ch: char) -> Token {
    self.new_token(Token::Character(ch));
    self.emit_current_token()
  }

  fn emit_current_char(&mut self) -> Token {
    self.emit_char(self.current_character)
  }

  fn emit_eof(&mut self) -> Token {
    self.new_token(Token::EOF);
    self.emit_current_token()
  }

  /* tmp buffer --------------------------------- */

  fn flush_code_points_consumed_as_a_character_reference(&mut self) {
    self.emit_tmp_buffer();
  }

  fn emit_tmp_buffer(&mut self) {
    for c in self.tmp_buffer.chars() {
      self.output.push_back(Token::Character(c));
    }
  }
}
