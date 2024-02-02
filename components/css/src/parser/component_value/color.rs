#[derive(Debug, PartialEq)]
pub enum Color {
  Rgb(f32, f32, f32),
  Rgba(f32, f32, f32, f32),
  Hsl(f32, f32, f32),
  Hsla(f32, f32, f32, f32),
  Hex(String),
}
