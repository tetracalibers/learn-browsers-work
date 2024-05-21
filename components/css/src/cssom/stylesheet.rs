use super::style_rule::StyleRule;

#[derive(Debug, PartialEq)]
pub struct StyleSheet {
  pub rules: Vec<Rule>,
}

#[derive(Debug, PartialEq)]
pub enum Rule {
  StyleRule(StyleRule),
  //AtRule(AtRule),
}
