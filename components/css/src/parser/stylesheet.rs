use super::at_rule::AtRule;
use super::qualified_rule::QualifiedRule;

pub struct StyleSheet {
  pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub enum Rule {
  QualifiedRule(QualifiedRule),
  AtRule(AtRule),
}
