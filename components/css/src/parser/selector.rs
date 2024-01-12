use std::{
  ops::{Deref, DerefMut},
  vec,
};

use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  character::complete::{char, space0, space1},
  combinator::{map, opt, value},
  multi::separated_list0,
  sequence::{delimited, preceded, terminated, tuple},
  IResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
enum Selector {
  Simple(SimpleSelector),
  Complex(ComplexSelector),
  Compound(CompoundSelector),
}

#[derive(Debug, PartialEq, Clone)]
enum SimpleSelector {
  Id(String),                           // #id
  Class(String),                        // .class
  Type(String),                         // div
  Attribute(AttributeSelector),         // [attr] とか [attr="value"] とか
  PseudoClass(PseudoClassSelector),     // :hover とか
  PseudoElement(PseudoElementSelector), // ::before とか
}

// p > a とか p + a とか p ~ a とか p a とか
#[derive(Debug, PartialEq, Clone)]
struct ComplexSelector {
  combinator: Combinator,
  left: Box<Selector>,
  right: Box<Selector>,
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

fn double_quoted_string(input: &str) -> IResult<&str, &str> {
  let (input, _) = tag("\"")(input)?;
  let (input, name) = take_while1(|c: char| c != '"')(input)?;
  let (input, _) = tag("\"")(input)?;
  Ok((input, name))
}

fn type_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, name) = identifier(input)?;
  Ok((input, SimpleSelector::Type(name.to_string())))
}

fn id_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, _) = tag("#")(input)?;
  let (input, name) = identifier(input)?;
  Ok((input, SimpleSelector::Id(name.to_string())))
}

fn class_selector(input: &str) -> IResult<&str, SimpleSelector> {
  let (input, _) = tag(".")(input)?;
  let (input, name) = identifier(input)?;
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
  todo!("pseudo_class_selector");
}

fn simple_selector(input: &str) -> IResult<&str, SimpleSelector> {
  alt((
    id_selector,
    class_selector,
    attribute_selector,
    pseudo_class_selector,
    type_selector,
  ))(input)
}

fn compound_selector(input: &str) -> IResult<&str, CompoundSelector> {
  let mut rest = input;
  let mut list = vec![];

  loop {
    if let Some(next) = rest.chars().next() {
      match next {
        '.' => {
          let (input, selector) = class_selector(rest)?;
          rest = input;
          list.push(selector);
        }
        '#' => {
          let (input, selector) = id_selector(rest)?;
          rest = input;
          list.push(selector);
        }
        '[' => {
          let (input, selector) = attribute_selector(rest)?;
          rest = input;
          list.push(selector);
        }
        ':' => {
          let (input, selector) = pseudo_class_selector(rest)?;
          rest = input;
          list.push(selector);
        }
        _ => {
          let (input, selector) = type_selector(rest)?;
          rest = input;
          list.push(selector);
        }
      }
    } else {
      break;
    }
  }

  Ok((rest, CompoundSelector(list)))
}

fn flatten_compound_selector(selector: &CompoundSelector) -> Selector {
  if selector.len() == 1 {
    Selector::Simple(selector[0].clone())
  } else {
    Selector::Compound(selector.clone())
  }
}

fn delimited_space0<'a, F, O>(
  f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
  F: Fn(&'a str) -> IResult<&'a str, O>,
{
  delimited(space0, f, space0)
}

fn combinator(input: &str) -> IResult<&str, Combinator> {
  alt((
    value(Combinator::Child, delimited_space0(tag(">"))),
    value(Combinator::NextSibling, delimited_space0(tag("+"))),
    value(Combinator::SubsequentSibling, delimited_space0(tag("~"))),
    value(Combinator::Descendant, space1), // 他の記号がスペースで囲まれている場合にマッチしないよう最後に置く
  ))(input)
}

fn complex_selector(input: &str) -> IResult<&str, ComplexSelector> {
  let (input, (left, combinator, right)) = tuple((
    terminated(compound_selector, space1),
    combinator,
    preceded(space1, compound_selector),
  ))(input)?;

  Ok((
    input,
    ComplexSelector {
      combinator,
      left: Box::new(flatten_compound_selector(&left)),
      right: Box::new(flatten_compound_selector(&right)),
    },
  ))
}

// #foo > .bar + div.k1.k2 [id='baz']:hello(2):not(:where(#yolo))::before
fn selector(input: &str) {
  let mut rest = input;

  loop {
    if let Some(c) = compound_selector(rest).ok() {
      println!("complex_selector: {:?}", c);
      rest = c.0;
    } else if let Some(r) = combinator(rest).ok() {
      println!("combinator: {:?}", r);
      rest = r.0;
    } else {
      break;
    }
  }
}

pub fn main() {
  let (rest, result) = compound_selector("div#foo.bar").unwrap();
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
  }
}
