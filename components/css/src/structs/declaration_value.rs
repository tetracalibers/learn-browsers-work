#[derive(Debug, PartialEq)]
pub enum DeclarationValue {
  // ref: https://www.w3.org/TR/css-values-4/#keywords
  Keyword(String),
  // ref: https://www.w3.org/TR/css-values-4/#dashed-idents
  DashedIndent(String),
  // ref: https://www.w3.org/TR/css-values-4/#strings
  QuotedString(String),
  Length(f32, Unit),
  OtherToken(String),
}

#[derive(Debug, PartialEq)]
pub enum Unit {
  Px,
}
