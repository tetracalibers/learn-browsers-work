use length::Length;

use crate::token::CSSToken;

pub mod length;

pub enum Value {
  Length(Length),
  // css wide keywords
  Inherit,
  Initial,
  Unset,
}

pub trait ValueParser {
  fn parse(token: CSSToken) -> Option<Value>;
}
