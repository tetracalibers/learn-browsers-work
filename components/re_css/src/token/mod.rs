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

pub enum Bracket {
  Square, // [
  Curly,  // {
  Round,  // (
}
