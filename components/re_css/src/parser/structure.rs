use css::structs::selector::SelectorList;

use crate::token::{Bracket, CSSToken};

#[derive(Debug)]
pub struct Function<'a> {
  pub name: &'a str,
  pub value: Vec<ComponentValue<'a>>,
}

#[derive(Debug)]
pub struct SimpleBlock<'a> {
  pub associated: Bracket,
  pub value: Vec<ComponentValue<'a>>,
}

#[derive(Debug)]
pub enum ComponentValue<'a> {
  PreservedToken(CSSToken<'a>),
  Function(Function<'a>),
  SimpleBlock(SimpleBlock<'a>),
}

#[derive(Debug)]
pub struct Declaration<'a> {
  pub name: &'a str,
  pub value: Vec<ComponentValue<'a>>,
  pub important: bool,
}

#[derive(Debug)]
pub enum BlockContent<'a> {
  Declaration(Declaration<'a>),
  AtRule(AtRule<'a>),
  QualifiedRule(QualifiedRule<'a>),
}

#[derive(Debug)]
pub struct QualifiedRule<'a> {
  pub prelude: Vec<ComponentValue<'a>>,
  pub block: Vec<BlockContent<'a>>,
}

#[derive(Debug)]
pub struct StyleRule<'a> {
  pub selectors: SelectorList,
  pub declarations: Vec<Declaration<'a>>,
}

#[derive(Debug)]
pub struct AtRule<'a> {
  pub name: &'a str,
  pub prelude: Vec<ComponentValue<'a>>,
  pub block: Option<SimpleBlock<'a>>,
}

#[derive(Debug)]
pub enum CSSRule<'a> {
  StyleRule(StyleRule<'a>),
  AtRule(AtRule<'a>),
}

struct StyleSheet<'a> {
  rules: Vec<CSSRule<'a>>,
}
