pub mod token;

use std::env;

use stream::input_stream::CharInputStream;
use token::Token;

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

fn is_indent_start(ch: char) -> bool {
  ch.is_ascii_alphabetic() || ch == '_'
}

#[allow(non_camel_case_types)]
pub enum Char {
  ch(char),
  eof,
}

pub struct Tokenizer<T>
where
  T: Iterator<Item = char>,
{
  input: CharInputStream<T>,
  output: Vec<Token>,

  current_character: char,
}

impl<T> Tokenizer<T>
where
  T: Iterator<Item = char>,
{
  pub fn new(input: T) -> Self {
    Tokenizer {
      input: CharInputStream::new(input),
      output: Vec::new(),
      current_character: '\0',
    }
  }

  fn consume_next(&mut self) -> Char {
    let ch = self.input.next();

    match ch {
      Some(c) => {
        self.current_character = c;
        Char::ch(c)
      }
      None => Char::eof,
    }
  }

  fn consume_while<F>(&mut self, predicate: F)
  where
    F: Fn(char) -> bool,
  {
    while let Some(c) = self.input.peek() {
      if !predicate(c) {
        return;
      }
      self.consume_next();
    }
  }

  fn reconsume(&mut self) {
    self.input.reconsume();
  }

  /* -------------------------------------------- */

  fn consume_token(&mut self) -> Token {
    self.consume_comment();

    match self.consume_next() {
      Char::ch(c) if c.is_whitespace() => {
        self.consume_whitespace();
        Token::Whitespace
      }
      Char::ch('"') => self.consume_string(None),
      Char::ch('#') => {
        todo!("consume_token: #");
      }
      Char::ch('\'') => self.consume_string(None),
      Char::ch('(') => Token::ParentheseOpen,
      Char::ch(')') => Token::ParentheseClose,
      Char::ch('+') => {
        todo!("consume_token: +");
      }
      Char::ch(',') => Token::Comma,
      Char::ch('-') => {
        todo!("consume_token: -");
      }
      Char::ch('.') => {
        todo!("consume_token: .");
      }
      Char::ch(':') => Token::Colon,
      Char::ch(';') => Token::SemiColon,
      Char::ch('<') => {
        todo!("consume_token: <");
      }
      Char::ch('@') => {
        todo!("consume_token: @");
      }
      Char::ch('[') => Token::BracketOpen,
      Char::ch('\\') => {
        todo!("consume_token: \\");
      }
      Char::ch(']') => Token::BracketClose,
      Char::ch('{') => Token::BraceOpen,
      Char::ch('}') => Token::BraceClose,
      Char::ch(c) if c.is_ascii_digit() => {
        todo!("consume_token: digit");
      }
      Char::ch(c) if is_indent_start(c) => {
        todo!("consume_token: indent_start");
      }
      Char::eof => Token::EOF,
      _ => Token::Delim(self.current_character),
    }
  }

  fn consume_whitespace(&mut self) {
    self.consume_while(|c| c.is_whitespace());
  }

  fn consume_string(&mut self, ending: Option<char>) -> Token {
    let ending_char = ending.unwrap_or(self.current_character);

    let mut token = Token::String(String::new());

    loop {
      match self.consume_next() {
        Char::ch(c) if c == ending_char => {
          return token;
        }
        Char::eof => {
          emit_error!("Unexpected EOF");
          return token;
        }
        Char::ch('\n') => {
          emit_error!("Unexpected newline");
          self.reconsume();
          return Token::BadString;
        }
        Char::ch('\\') => {
          let next_char = self.input.peek();

          if next_char.is_none() {
            continue;
          }

          if let Some('\n') = next_char {
            self.consume_next();
            continue;
          }

          token.append_to_string_token(self.consume_escaped());
        }
        Char::ch(c) => {
          token.append_to_string_token(c);
        }
      }
    }
  }

  fn consume_escaped(&mut self) -> char {
    todo!("consume_escaped");
  }

  // 「/*」から「*/」までの文字列を読み飛ばす
  fn consume_comment(&mut self) {
    todo!("consume_comment");
  }
}
