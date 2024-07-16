use crate::token::CSSToken;

use super::{Value, ValueParser};

#[derive(Debug)]
pub struct Percentage(f64);

impl ValueParser for Percentage {
  fn parse(token: &CSSToken) -> Option<Value> {
    match token {
      CSSToken::Percentage(value) => {
        Some(Value::Percentage(Percentage(*value)))
      }
      _ => None,
    }
  }
}
