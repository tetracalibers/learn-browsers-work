#[derive(Debug, PartialEq)]
pub enum AtRule {
  Regular(RegularAtRule),
  Nested(NestedAtRule),
  ConditionalGroup(ConditionalGroupAtRule),
}

#[derive(Debug, PartialEq)]
pub struct RegularAtRule {
  pub name: String,
  pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct NestedAtRule {
  name: String,
  prelude: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub struct ConditionalGroupAtRule {
  name: String,
  prelude: Vec<String>,
  block: Vec<AtRule>,
}
