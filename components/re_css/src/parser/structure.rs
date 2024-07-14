use css::structs::selector::SelectorList;

use crate::token::{Bracket, CSSToken};

struct FunctionBlock<'a> {
  name: &'a str,
  value: Vec<ComponentValue<'a>>,
}

struct SimpleBlock<'a> {
  bracket: Bracket,
  value: Vec<ComponentValue<'a>>,
}

pub enum ComponentValue<'a> {
  PreservedToken(CSSToken<'a>),
  Function(FunctionBlock<'a>),
  Block(SimpleBlock<'a>),
}

struct Declaration<'a> {
  name: &'a str,
  value: Vec<ComponentValue<'a>>,
  important: bool,
}

struct QualifiedRule<'a> {
  selectors: SelectorList,
  declarations: Vec<Declaration<'a>>,
}

enum AtRule<'a> {
  Regular(RegularAtRule<'a>),
  Nested(NestedAtRule<'a>),
}

// @identifier RULE;
struct RegularAtRule<'a> {
  name: &'a str,
  value: &'a str,
}

// @identifier PRELUDE (RULE) {}
struct NestedAtRule<'a> {
  name: &'a str,
  prelude: Vec<&'a str>,
  block: QualifiedRule<'a>,
}

enum CSSRule<'a> {
  StyleRule(QualifiedRule<'a>),
  AtRule(AtRule<'a>),
}

struct StyleSheet<'a> {
  rules: Vec<CSSRule<'a>>,
}
