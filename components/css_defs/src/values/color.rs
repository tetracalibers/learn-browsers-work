use css::structs::declaration_value::DeclarationValue;

use super::number::Number;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Color {
  CurrentColor,
  Rgba(Number, Number, Number, Number),
  Transparent,
}

impl Color {
  pub fn parse(values: &[DeclarationValue]) -> Option<Self> {
    match values.iter().next() {
      Some(DeclarationValue::Keyword(keyword)) => {
        match keyword.to_lowercase().as_str() {
          "currentcolor" => Some(Color::CurrentColor),
          "transparent" => Some(Color::Transparent),
          _ => None,
        }
      }
      Some(DeclarationValue::OtherToken(token)) => {
        let result = csscolorparser::parse(token);
        match result {
          Ok(color) => Some(Color::Rgba(
            Number(color.r as f32),
            Number(color.g as f32),
            Number(color.b as f32),
            Number(color.a as f32),
          )),
          Err(_) => None,
        }
      }
      _ => None,
    }
  }
}
