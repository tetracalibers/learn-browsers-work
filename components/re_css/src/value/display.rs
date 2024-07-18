// spec: CSS Display Module Level 4

use crate::token::CSSToken;

use super::{Value, ValueParser};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Display {
  Full(DisplayOutside, DisplayInside),
  Box(DisplayBox),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DisplayOutside {
  Block,
  Inline,
  RunIn,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DisplayInside {
  Flow,
  FlowRoot,
  Table,
  Flex,
  Grid,
  Ruby,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DisplayBox {
  Contents,
  None,
}

impl std::str::FromStr for Display {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "none" => Ok(Display::Box(DisplayBox::None)),
      "contents" => Ok(Display::Box(DisplayBox::Contents)),
      "block" => Ok(Self::new_block()),
      "inline" => Ok(Self::new_inline()),
      "inline-block" => Ok(Display::Full(
        DisplayOutside::Inline,
        DisplayInside::FlowRoot,
      )),
      _ => Err("Invalid display value"),
    }
  }
}

impl ValueParser for Display {
  fn parse_token(token: &CSSToken) -> Option<Value> {
    match token {
      CSSToken::Ident(value) => {
        if let Ok(display) = value.parse::<Display>() {
          Some(Value::Display(display))
        } else {
          None
        }
      }
      _ => None,
    }
  }
}

impl Display {
  pub fn new_block() -> Self {
    Display::Full(DisplayOutside::Block, DisplayInside::Flow)
  }

  pub fn new_inline() -> Self {
    Display::Full(DisplayOutside::Inline, DisplayInside::Flow)
  }
}
