use super::number::Number;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Color {
  CurrentColor,
  Rgba(Number, Number, Number, Number),
  Transparent,
}

impl Color {
  pub fn parse(value: String) -> Option<Self> {
    match value.to_lowercase().as_str() {
      "currentcolor" => Some(Color::CurrentColor),
      "transparent" => Some(Color::Transparent),
      _ => {
        let result = csscolorparser::parse(value.as_str());
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
    }
  }
}
