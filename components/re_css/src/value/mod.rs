use length::Length;
use percentage::Percentage;

use crate::token::CSSToken;

pub mod length;
pub mod percentage;

pub enum Value {
  Length(Length),
  Percentage(Percentage),
  // css wide keywords
  Inherit,
  Initial,
  Unset,
}

pub trait ValueParser {
  fn parse(token: CSSToken) -> Option<Value>;
}
