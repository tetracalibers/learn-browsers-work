use std::{
  cmp::Ordering,
  ops::{Deref, DerefMut},
};

pub type SelectorList = Vec<Selector>;

#[derive(Debug, PartialEq, Clone)]
pub struct Selector(pub ComplexSelectorSequence);

impl Selector {
  pub fn values(&self) -> &ComplexSelectorSequence {
    &self.0
  }

  pub fn specificity(&self) -> Specificity {
    let (a, b, c) =
      self.values().iter().fold((0, 0, 0), |acc, (selector, _)| {
        let specificity = selector.specificity();
        (
          acc.0 + specificity.0,
          acc.1 + specificity.1,
          acc.2 + specificity.2,
        )
      });
    Specificity(a, b, c)
  }
}

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

impl CompoundSelector {
  pub fn specificity(&self) -> Specificity {
    let (a, b, c) =
      self.values().iter().fold((0, 0, 0), |acc, curr| match curr {
        SimpleSelector::Id(_) => (acc.0 + 1, acc.1, acc.2),
        SimpleSelector::Class(_) => (acc.0, acc.1 + 1, acc.2),
        SimpleSelector::Type(_) => (acc.0, acc.1, acc.2 + 1),
        _ => acc,
      });
    Specificity(a, b, c)
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AttributeSelector {
  pub name: String,
  pub operator: Option<AttributeOperator>, // Noneの場合は存在チェック
  pub value: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AttributeOperator {
  Equal,      // [attr="value"] ...完全一致
  DashMatch,  // [attr|="value"] ... valueに完全一致 or value-で始まる
  Contains,   // [attr~="value"] ...単語として含む
  Substring,  // [attr*="value"] ...部分文字列として含む
  StartsWith, // [attr^="value"] ...先頭が一致
  EndsWith,   // [attr$="value"] ...末尾が一致
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

/* -------------------------------------------- */

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/Specificity
#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Specificity(u32, u32, u32); // (ID, CLASS, TYPE)

impl Ord for Specificity {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.0.cmp(&other.0) {
      // ID列の数字が同じであれば、次のCLASS列を比較
      Ordering::Equal => match self.1.cmp(&other.1) {
        // ID列の数字も同じであれば、TYPE列を比較
        Ordering::Equal => self.2.cmp(&other.2),
        // TYPE列の値にかかわらず、 CLASS列の値が大きいセレクターが勝つ
        other => other,
      },
      // 他の列の値がどうであれ、ID列の値がより大きいセレクターが勝つ
      other => other,
    }
  }
}
