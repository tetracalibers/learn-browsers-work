use std::{
  borrow::Borrow,
  ops::{Deref, DerefMut},
};

use nom::{
  branch::alt,
  bytes::complete::{tag, take_till, take_while1},
  character::complete::{one_of, space0, space1},
  combinator::{eof, opt, peek, value},
  multi::{fold_many0, fold_many1, many0, many_till},
  sequence::{delimited, tuple},
  IResult,
};

#[derive(Debug, PartialEq, Clone)]
struct Selector(Vec<SelectorData>);

type SelectorData = (CompoundSelector, Option<Combinator>);

#[derive(Debug, PartialEq, Clone)]
enum SimpleSelector {
  Id(String),                           // #id
  Class(String),                        // .class
  Type(String),                         // div
  Attribute(AttributeSelector),         // [attr] とか [attr="value"] とか
  PseudoClass(PseudoClassSelector),     // :hover とか
  PseudoElement(PseudoElementSelector), // ::before とか
}

// p.class#id とか p:not(.class) とか
#[derive(Debug, PartialEq, Clone)]
struct CompoundSelector(Vec<SimpleSelector>);

#[derive(Debug, PartialEq, Clone)]
struct AttributeSelector {
  name: String,
  operator: Option<AttributeOperator>,
  value: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
enum AttributeOperator {
  Equal,
  Contains,
  StartsWith,
  EndsWith,
}

#[derive(Debug, PartialEq, Clone)]
enum Combinator {
  Descendant,
  Child,
  NextSibling,
  SubsequentSibling,
}

#[derive(Debug, PartialEq, Clone)]
struct PseudoClassSelector {
  name: String,
  argument: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
struct PseudoElementSelector {
  name: String,
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

/* -------------------------------------------- */

// 最終的に以下をparseできるようなものを作りたい
// #foo > .bar + div.k1.k2 [id='baz']:hello(2):not(:where(#yolo))::before

// クラス名やID名として有効なもの
fn identifier(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_')(input)
}

// 擬似クラスの名称などとして有効なもの
fn keyword(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric())(input)
}

fn till_next_start(input: &str) -> IResult<&str, &str> {
  take_till(|c: char| {
    ['#', '.', '(', '{', '[', ':', '>', '+', '~'].contains(&c)
      || c.is_whitespace()
  })(input)
}

fn double_quoted_string(input: &str) -> IResult<&str, &str> {
  let (input, _) = tag("\"")(input)?;
  let (input, name) = take_while1(|c: char| c != '"')(input)?;
  let (input, _) = tag("\"")(input)?;
  Ok((input, name))
}

fn type_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, name) = till_next_start(input)?;
  Ok((input, SimpleSelector::Type(name.to_string())))
}

fn id_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, _) = tag("#")(input)?;
  let (input, name) = till_next_start(input)?; // ID名はクラス名や擬似クラス名などとして有効なもので終わる
  Ok((input, SimpleSelector::Id(name.to_string())))
}

fn class_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, _) = tag(".")(input)?;
  let (input, name) = till_next_start(input)?;
  Ok((input, SimpleSelector::Class(name.to_string())))
}

fn attribute_operator(input: &str) -> IResult<&str, AttributeOperator> {
  alt((
    value(AttributeOperator::Equal, tag("=")),
    value(AttributeOperator::Contains, tag("~=")),
    value(AttributeOperator::StartsWith, tag("^=")),
    value(AttributeOperator::EndsWith, tag("$=")),
  ))(input)
}

fn attribute_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, _) = tag("[")(input)?;
  let (input, name) = identifier(input)?;
  let (input, operator) = opt(attribute_operator)(input)?;
  let (input, value) = opt(double_quoted_string)(input)?;
  let (input, _) = tag("]")(input)?;

  Ok((
    input,
    SimpleSelector::Attribute(AttributeSelector {
      name: name.to_string(),
      operator,
      value: value.map(|v| v.to_string()),
    }),
  ))
}

fn pseudo_class_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, _) = tag(":")(input)?;
  let (input, name) = till_next_start(input)?;
  let (input, argument) = opt(delimited(tag("("), keyword, tag(")")))(input)?;

  Ok((
    input,
    SimpleSelector::PseudoClass(PseudoClassSelector {
      name: name.to_string(),
      argument: argument.map(|v| v.to_string()),
    }),
  ))
}

fn pseudo_element_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, _) = tag("::")(input)?;
  let (input, name) = till_next_start(input)?;

  Ok((
    input,
    SimpleSelector::PseudoElement(PseudoElementSelector {
      name: name.to_string(),
    }),
  ))
}

fn delimited_space0<'a, F, O>(
  f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
  F: Fn(&'a str) -> IResult<&'a str, O>,
{
  delimited(space0, f, space0)
}

fn compound_selector(input: &str) -> IResult<&str, CompoundSelector> {
  let input = input.trim();
  // altは上から順にマッチするので、並べる順序が重要
  let (input, (selectors, _)) = many_till(
    alt((
      id_selector,
      class_selector,
      attribute_selector,
      pseudo_element_selector,
      pseudo_class_selector,
      type_selector,
    )),
    alt((peek(combinator_as_string), eof)),
  )(input)?;

  Ok((input, CompoundSelector(selectors)))
}

fn combinator(input: &str) -> IResult<&str, Combinator> {
  alt((
    value(
      Combinator::Child,
      delimited(space0, tag(">"), take_till(|c| c == ' ')),
    ),
    value(
      Combinator::NextSibling,
      delimited(space0, tag("+"), take_till(|c| c == ' ')),
    ),
    value(
      Combinator::SubsequentSibling,
      delimited(space0, tag("~"), take_till(|c| c == ' ')),
    ),
    value(Combinator::Descendant, tuple((space1, till_next_start))), // 他の記号がスペースで囲まれている場合にマッチしないよう最後に置く
  ))(input)
}

fn child_combinator_as_string(input: &str) -> IResult<&str, &str> {
  delimited(space0, tag(">"), take_till(|c| c == ' '))(input)
}

fn next_sibling_combinator_as_string(input: &str) -> IResult<&str, &str> {
  delimited(space0, tag("+"), take_till(|c| c == ' '))(input)
}

fn subsequent_sibling_combinator_as_string(input: &str) -> IResult<&str, &str> {
  delimited(space0, tag("~"), take_till(|c| c == ' '))(input)
}

fn descendant_combinator_as_string(input: &str) -> IResult<&str, &str> {
  space1(input)
}

fn combinator_as_string(input: &str) -> IResult<&str, &str> {
  alt((
    child_combinator_as_string,
    next_sibling_combinator_as_string,
    subsequent_sibling_combinator_as_string,
    descendant_combinator_as_string,
  ))(input)
}

// cobminatorで繋がれた2つのcompound_selectorをparseする
fn complex_selector(input: &str) -> IResult<&str, SelectorData> {
  let (input, selector) = compound_selector(input)?;
  let (input, combinator) = opt(combinator)(input)?;

  Ok((input, (selector, combinator)))
}

fn selector(input: &str) -> IResult<&str, Selector> {
  let (input, (selectors, _)) = many_till(complex_selector, peek(eof))(input)?;

  // 空のcompound_selectorがある場合は削除する
  let selectors = selectors
    .into_iter()
    .filter(|(selector, _)| !selector.is_empty())
    .collect();

  Ok((input, Selector(selectors)))
}

// #foo > .bar + div.k1.k2 [id='baz']:hello(2):not(:where(#yolo))::before
pub fn main() {
  let input = "div.class + ~ #id";

  let result = selector(input);

  println!("result: {:?}", result);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_type_selector() {
    assert_eq!(
      type_selector("div"),
      Ok(("", SimpleSelector::Type("div".to_string())))
    );
  }

  #[test]
  fn test_id_selector() {
    assert_eq!(
      id_selector("#foo"),
      Ok(("", SimpleSelector::Id("foo".to_string())))
    );
  }

  #[test]
  fn test_class_selector() {
    assert_eq!(
      class_selector(".bar"),
      Ok(("", SimpleSelector::Class("bar".to_string())))
    );
  }

  #[test]
  fn test_attribute_selector() {
    assert_eq!(
      attribute_selector("[href]"),
      Ok((
        "",
        SimpleSelector::Attribute(AttributeSelector {
          name: "href".to_string(),
          operator: None,
          value: None,
        })
      ))
    );
    assert_eq!(
      attribute_selector("[href=\"https://example.com\"]"),
      Ok((
        "",
        SimpleSelector::Attribute(AttributeSelector {
          name: "href".to_string(),
          operator: Some(AttributeOperator::Equal),
          value: Some("https://example.com".to_string()),
        })
      ))
    )
  }

  #[test]
  fn test_compound_selector() {
    assert_eq!(
      compound_selector("div#foo.bar"),
      Ok((
        "",
        CompoundSelector(vec![
          SimpleSelector::Type("div".to_string()),
          SimpleSelector::Id("foo".to_string()),
          SimpleSelector::Class("bar".to_string()),
        ])
      ))
    );
  }

  #[test]
  fn test_combinator() {
    assert_eq!(combinator(" "), Ok(("", Combinator::Descendant)));
    assert_eq!(combinator(">"), Ok(("", Combinator::Child)));
    assert_eq!(combinator("+"), Ok(("", Combinator::NextSibling)));
    assert_eq!(combinator("~"), Ok(("", Combinator::SubsequentSibling)));
    assert_eq!(combinator(" > .bar"), Ok(("", Combinator::Child)));
  }

  #[test]
  fn test_selector() {
    assert_eq!(
      selector("div.class #id"),
      Ok((
        "",
        Selector(vec![
          (
            CompoundSelector(vec![
              SimpleSelector::Type("div".to_string()),
              SimpleSelector::Class("class".to_string()),
            ]),
            Some(Combinator::Descendant)
          ),
          (
            CompoundSelector(vec![SimpleSelector::Id("id".to_string())]),
            None
          ),
        ])
      ))
    );
    assert_eq!(
      selector("div.class > #id"),
      Ok((
        "",
        Selector(vec![
          (
            CompoundSelector(vec![
              SimpleSelector::Type("div".to_string()),
              SimpleSelector::Class("class".to_string()),
            ]),
            Some(Combinator::Child)
          ),
          (
            CompoundSelector(vec![SimpleSelector::Id("id".to_string())]),
            None
          ),
        ])
      ))
    );
    assert_eq!(
      selector("div.class > > #id"),
      Ok((
        "",
        Selector(vec![
          (
            CompoundSelector(vec![
              SimpleSelector::Type("div".to_string()),
              SimpleSelector::Class("class".to_string()),
            ]),
            Some(Combinator::Child)
          ),
          (
            CompoundSelector(vec![SimpleSelector::Id("id".to_string())]),
            None
          ),
        ])
      ))
    );
  }
}
