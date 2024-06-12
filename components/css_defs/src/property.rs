#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Property {
  BorderTopColor,
  BorderRightColor,
  BorderBottomColor,
  BorderLeftColor,
}

impl Property {
  pub fn parse(name: &str) -> Option<Self> {
    match name {
      "border-top-color" => Some(Self::BorderTopColor),
      "border-right-color" => Some(Self::BorderRightColor),
      "border-bottom-color" => Some(Self::BorderBottomColor),
      "border-left-color" => Some(Self::BorderLeftColor),
      _ => {
        log::debug!("Unknown property: {}", name);
        None
      }
    }
  }
}
