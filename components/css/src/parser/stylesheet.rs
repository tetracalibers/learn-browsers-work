use super::at_rule::AtRule;
use super::style_rule::StyleRule;

pub struct StyleSheet {
  pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub enum Rule {
  StyleRule(StyleRule),
  AtRule(AtRule),
}
