use super::style_rule::StyleRule;

#[derive(Debug, PartialEq)]
pub struct StyleSheet {
  pub rules: Vec<CSSRule>,
}

#[derive(Debug, PartialEq)]
pub enum CSSRule {
  Style(StyleRule),
  //AtRule(AtRule),
}
