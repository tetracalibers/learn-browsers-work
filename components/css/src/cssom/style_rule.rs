use crate::structs::{
  declaration::Declaration,
  selector::{SelectorList, Specificity},
};

#[derive(Debug, PartialEq)]
pub struct StyleRule {
  pub selectors: SelectorList,
  pub declarations: Vec<Declaration>,
}

impl StyleRule {
  pub fn specificity(&self) -> Specificity {
    let specificities = self
      .selectors
      .iter()
      .map(|selector| selector.specificity())
      .collect::<Vec<Specificity>>();

    specificities.into_iter().max().unwrap()
  }
}
