pub mod state;
pub mod token;

use std::collections::VecDeque;
use std::env;

use state::State;

use token::Token;

use stream::input_stream::CharInputStream;

fn is_trace() -> bool {
  match env::var("TRACE_TOKENIZER") {
    Ok(s) => s == "true",
    _ => false,
  }
}

macro_rules! trace {
  ($err: expr) => {
    println!("[ParseError][Tokenizer] {}", $err);
  };
}

macro_rules! emit_error {
  ($err: expr) => {
    if is_trace() {
      trace!($err);
    }
  };
}

const REPLACEMENT_CHARACTER: char = '\u{FFFD}';

#[allow(non_camel_case_types)]
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

  tmp_buffer: String,
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

      tmp_buffer: String::new(),
    }
  }

  pub fn next_token(&mut self) -> Token {
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
              emit_error!("unexpected-null-character");
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
              emit_error!("unexpected-question-mark-instead-of-tag-name");
              self.new_token(Token::new_comment(""));
              self.reconsume_in(State::BogusComment);
            }
            Char::eof => {
              emit_error!("eof-before-tag-name");
              self.will_emit(Token::Character('<'));
              return self.emit_eof();
            }
            _ => {
              emit_error!("invalid-first-character-of-tag-name");
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
              emit_error!("missing-end-tag-name");
              self.switch_to(State::Data);
            }
            Char::eof => {
              emit_error!("eof-before-tag-name");
              self.will_emit(Token::Character('<'));
              self.will_emit(Token::Character('/'));
              return self.emit_eof();
            }
            _ => {
              emit_error!("invalid-first-character-of-tag-name");
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
              emit_error!("unexpected-null-character");
              self.append_char_to_tag_name(REPLACEMENT_CHARACTER);
            }
            Char::eof => {
              emit_error!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              self.append_char_to_tag_name(self.current_character);
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
              emit_error!("eof-in-tag");
              return self.emit_eof();
            }
            _ => {
              emit_error!("unexpected-solidus-in-tag");
              self.reconsume_in(State::BeforeAttributeName);
            }
          }
        }

        State::BeforeAttributeName => {
          todo!("State::BeforeAttributeName");
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
              emit_error!("unexpected-null-character");
              self.append_char_to_token_data(REPLACEMENT_CHARACTER);
            }
            _ => {
              self.append_char_to_token_data(self.current_character);
            }
          }
        }

        State::MarkupDeclarationOpen => {
          todo!("State::MarkupDeclarationOpen");
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
    if is_trace() {
      println!("Switch to: {:#?}", state);
    }
    self.state = state;
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

  /* token -------------------------------------- */

  fn new_token(&mut self, token: Token) {
    self.current_token = Some(token);
  }

  /* emit token --------------------------------- */

  fn will_emit(&mut self, token: Token) {
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
