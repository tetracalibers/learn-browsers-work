pub mod parser;
pub mod tokenizer;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::multi::many0;
use nom::sequence::delimited;
use nom::IResult;

macro_rules! emit_error {
  ($err: expr) => {
    println!("[ParseError][CSS] {}", $err);
  };
}

// pub fn comment(s: &str) {
//   let mut parse = delimited(tag("/*"), take_until("*/"), tag("*/"));
//   let result = parse(s);

//   match result {
//     Ok((out, matched)) => {}
//     Err(e) => {
//       emit_error!("Unexpected EOF while consume_comments");
//     }
//   }
// }
