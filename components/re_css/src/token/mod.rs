mod at_keyword;
mod escape;
mod function;
mod hash;
mod ident;
mod number;
mod string;
mod unicode;
mod url;
mod utils;

pub mod prelude {
  pub use crate::token::at_keyword::{at_keyword_str, at_keyword_token};
  pub use crate::token::escape::escape_token;
  pub use crate::token::function::function_token_str;
  pub use crate::token::hash::hash_token;
  pub use crate::token::ident::{ident_token, ident_token_str};
  pub use crate::token::number::{
    dimension_token, number_token, percentage_token,
  };
  pub use crate::token::string::string_token;
  pub use crate::token::unicode::unicode_range_token;
  pub use crate::token::url::url_token;
}

#[derive(Debug)]
pub enum CSSToken<'src> {
  Comment(&'src str),
  Escape(&'src str),
  Ident(&'src str),
  Function(&'src str),
  AtKeyword(&'src str),
  Hash(&'src str),
  String(&'src str),
  Url(&'src str),
  Number(f64),
  Dimension(f64, &'src str), // (value, unit)
  Percentage(f64),
  UnicodeRange(&'src str),
}

#[derive(Debug)]
pub enum Bracket {
  Square, // [
  Curly,  // {
  Round,  // (
}
