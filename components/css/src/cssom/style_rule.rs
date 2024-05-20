use crate::structs::{declaration::Declaration, selector::SelectorList};

#[derive(Debug, PartialEq)]
pub struct StyleRule {
  pub selector: SelectorList,
  pub declarations: Vec<Declaration>,
}
