pub enum Property {
  MarginTop,
}

impl std::str::FromStr for Property {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "margin-top" => Ok(Property::MarginTop),
      _ => Err("Invalid property"),
    }
  }
}
