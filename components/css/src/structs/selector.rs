use std::ops::{Deref, DerefMut};

pub type SelectorList = Vec<ComplexSelectorSequence>;

pub type ComplexSelectorSequence = Vec<ComplexSelector>;

pub type ComplexSelector = (CompoundSelector, Option<Combinator>);

#[derive(Debug, PartialEq, Clone)]
pub enum SimpleSelector {
  Universal,                            // *
  Id(String),                           // #id
  Class(String),                        // .class
  Type(String),                         // div
  Attribute(AttributeSelector),         // [attr] とか [attr="value"] とか
  PseudoClass(PseudoClassSelector),     // :hover とか
  PseudoElement(PseudoElementSelector), // ::before とか
}

// p.class#id とか p:not(.class) とか
#[derive(Debug, PartialEq, Clone)]
pub struct CompoundSelector(pub Vec<SimpleSelector>);

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeSelector {
  pub name: String,
  pub operator: Option<AttributeOperator>,
  pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttributeOperator {
  Equal,
  Contains,
  StartsWith,
  EndsWith,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Combinator {
  Descendant,
  Child,
  NextSibling,
  SubsequentSibling,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PseudoClassSelector {
  pub name: String,
  pub argument: Option<String>,
  pub subtree: Option<SelectorList>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PseudoElementSelector {
  pub name: String,
}

/* -------------------------------------------- */

impl Deref for CompoundSelector {
  type Target = Vec<SimpleSelector>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for CompoundSelector {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl CompoundSelector {
  pub fn values(&self) -> &Vec<SimpleSelector> {
    &self.0
  }
}
