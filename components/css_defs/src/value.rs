use super::values::prelude::*;

#[derive(Debug)]
pub enum Value {
  Color(Color),
  Display(Display),
  Length(Length),
  Percentage(Percentage),
  BorderStyle(BorderStyle),
  BorderWidth(BorderWidth),
  // todo: BorderRadius(BorderRadius),
  Float(Float),
  Position(Position),
  Direction(Direction),
  TextAlign(TextAlign),
  Overflow(Overflow),
  FontWeight(FontWeight),
  Auto,
  Inherit,
  Initial,
  Unset,
}
