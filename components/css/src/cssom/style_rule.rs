use crate::structs::{declaration::Declaration, selector::SelectorList};

#[derive(Debug, PartialEq)]
pub struct StyleRule {
  pub selectors: SelectorList,
  pub declarations: Vec<Declaration>,
}
