pub mod sample;
pub mod selector;
mod structs;

use std::env;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::sequence::delimited;
use nom::IResult;

use stream::input_stream::CharInputStream;

use super::tokenizer::token::Token;

fn is_trace() -> bool {
  match env::var("TRACE_CSS_PARSER") {
    Ok(s) => s == "true",
    _ => false,
  }
}

macro_rules! trace {
  ($err: expr) => {
    println!("[ParseError][CSS Parser] {}", $err);
  };
}

macro_rules! emit_error {
  ($err: expr) => {
    if is_trace() {
      trace!($err);
    }
  };
}

pub struct Parser<T>
where
  T: Iterator<Item = char>,
{
  input: CharInputStream<T>,
  output: Vec<Token>,

  current_character: char,
}

impl<T> Parser<T>
where
  T: Iterator<Item = char>,
{
  pub fn new(input: T) -> Self {
    Parser {
      input: CharInputStream::new(input),
      output: Vec::new(),
      current_character: '\0',
    }
  }

  // fn consume_next(&mut self) -> Char {
  //   let ch = self.input.next();

  //   match ch {
  //     Some(c) => {
  //       self.current_character = c;
  //       Char::ch(c)
  //     }
  //     None => Char::eof,
  //   }
  // }

  // fn skip_comment(&mut self) {
  //   let parse_open = tag("/*");
  //   let parse_close = tag("*/");

  //   if Some(maybe_open) = self.input.peek_next_repeat(2) {
  //     if !parse_open(maybe_open).is_ok() {
  //       return;
  //     }
  //     self.consume_next(); // consume '/'
  //     self.consume_next(); // consume '*'

  //     loop {
  //       let maybe_close = self.input.peek_next_repeat::<String>(2);
  //       if let Some(close) = maybe_close {
  //         if parse_close(close).is_ok() {
  //           self.consume_next(); // consume '*'
  //           self.consume_next(); // consume '/'
  //           break;
  //         } else {
  //           self.consume_next();
  //         }
  //       } else {
  //         emit_error!("Unexpected EOF while consume_comments");
  //         return;
  //       }
  //     }
  //   }
  // }
}
