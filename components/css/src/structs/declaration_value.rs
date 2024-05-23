#[derive(Debug, PartialEq)]
pub enum DeclarationValue {
  // ref: https://www.w3.org/TR/css-values-4/#keywords
  Keyword(String),
  // ref: https://www.w3.org/TR/css-values-4/#dashed-idents
  DashedIndent(String),
  // ref: https://www.w3.org/TR/css-values-4/#strings
  QuotedString(String),
  Length(f32, Unit),
  ColorValue(Color),
}

#[derive(Debug, PartialEq)]
pub enum Unit {
  Px,
}

#[derive(Debug, PartialEq)]
pub enum Color {
  Rgb(f32, f32, f32),
  Rgba(f32, f32, f32, f32),
  Hsl(f32, f32, f32),
  Hsla(f32, f32, f32, f32),
  Hex(String),
}
