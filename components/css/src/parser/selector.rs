use std::ops::{Deref, DerefMut};

use nom::{
  branch::alt,
  bytes::complete::{tag, take_till, take_while1},
  character::complete::{alpha1, alphanumeric1, space0, space1},
  combinator::{eof, opt, peek, value},
  multi::{many0, many_till},
  sequence::{delimited, tuple},
  IResult,
};

type SelectorSequence = Vec<Selector>;

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
  subtree: Option<Selector>,
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

// クラス名やID名として有効なもの
fn identifier(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_')(input)
}

fn till_next_start(input: &str) -> IResult<&str, &str> {
  take_till(|c: char| {
    [
      '#', '.', '(', '{', '[', ':', '>', '+', '~', ')', '}', ']', ',',
    ]
    .contains(&c)
      || c.is_whitespace()
  })(input)
}

fn double_quoted_string(input: &str) -> IResult<&str, &str> {
  let (input, _) = tag("\"")(input)?;
  let (input, name) = take_while1(|c: char| c != '"')(input)?;
  let (input, _) = tag("\"")(input)?;
  Ok((input, name))
}

fn parenthesized<'a, F, O>(
  parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
  F: FnMut(&'a str) -> IResult<&'a str, O>,
{
  delimited(tag("("), parser, tag(")"))
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
  let (input, name) = many0(alt((alpha1, tag("-"))))(input)?;
  let (input, argument) = opt(parenthesized(alphanumeric1))(input)?;
  let (input, subtree) = opt(parenthesized(selector))(input)?;

  Ok((
    input,
    SimpleSelector::PseudoClass(PseudoClassSelector {
      name: name.join("").to_string(),
      argument: argument.map(|v| v.to_string()),
      subtree,
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
    alt((peek(delimiter_string), eof)),
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

fn delimiter_string(input: &str) -> IResult<&str, &str> {
  alt((
    tag(">"),
    tag("+"),
    tag("~"),
    tag(")"),
    tag("("),
    tag(","),
    space1,
  ))(input)
}

// cobminatorで繋がれた2つのcompound_selectorをparseする
fn complex_selector(input: &str) -> IResult<&str, SelectorData> {
  let (input, selector) = compound_selector(input)?;
  let (input, combinator) = opt(combinator)(input)?;

  Ok((input, (selector, combinator)))
}

fn selector(input: &str) -> IResult<&str, Selector> {
  let (input, (selectors, _)) =
    many_till(complex_selector, alt((eof, tag(","), peek(tag(")")))))(input)?;

  // 空のcompound_selectorがある場合は削除する
  let selectors = selectors
    .into_iter()
    .filter(|(selector, _)| !selector.is_empty())
    .collect();

  Ok((input, Selector(selectors)))
}

fn selector_sequence(input: &str) -> IResult<&str, SelectorSequence> {
  let splitted = input.split(',').collect::<Vec<&str>>();

  let mut rest = input;
  let mut selectors = Vec::new();

  for s in splitted {
    let (input, selector) = selector(s)?;
    selectors.push(selector);
    rest = input;
  }

  Ok((rest, selectors))
}

pub fn main() {
  let input =
    r#"#foo > .bar + div.k1.k2 [id="baz"]:hello(2):not(:where(#yolo))::before"#;

  let result = selector_sequence(input);

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
    assert_eq!(combinator(" > .bar"), Ok((" .bar", Combinator::Child)));
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
    assert_eq!(
      selector(":where(#yoro)"),
      Ok((
        "",
        Selector(vec![(
          CompoundSelector(vec![SimpleSelector::PseudoClass(
            PseudoClassSelector {
              name: "where".to_string(),
              argument: None,
              subtree: Some(Selector(vec![(
                CompoundSelector(vec![SimpleSelector::Id("yoro".to_string())]),
                None
              )]))
            }
          )]),
          None
        )])
      ))
    );
    assert_eq!(
      selector("input:not(:where(#yolo))"),
      Ok((
        "",
        Selector(vec![(
          CompoundSelector(vec![
            SimpleSelector::Type("input".to_string()),
            SimpleSelector::PseudoClass(PseudoClassSelector {
              name: "not".to_string(),
              argument: None,
              subtree: Some(Selector(vec![(
                CompoundSelector(vec![SimpleSelector::PseudoClass(
                  PseudoClassSelector {
                    name: "where".to_string(),
                    argument: None,
                    subtree: Some(Selector(vec![(
                      CompoundSelector(vec![SimpleSelector::Id(
                        "yolo".to_string()
                      )]),
                      None
                    )]))
                  }
                )]),
                None
              )]))
            })
          ]),
          None
        )])
      ))
    );
    assert_eq!(
      selector("a:hover::before"),
      Ok((
        "",
        Selector(vec![(
          CompoundSelector(vec![
            SimpleSelector::Type("a".to_string()),
            SimpleSelector::PseudoClass(PseudoClassSelector {
              name: "hover".to_string(),
              argument: None,
              subtree: None,
            }),
            SimpleSelector::PseudoElement(PseudoElementSelector {
              name: "before".to_string(),
            })
          ]),
          None
        )])
      ))
    )
  }

  #[test]
  fn test_pseudo_class_selector() {
    assert_eq!(
      pseudo_class_selector(":hover"),
      Ok((
        "",
        SimpleSelector::PseudoClass(PseudoClassSelector {
          name: "hover".to_string(),
          argument: None,
          subtree: None,
        })
      ))
    );
    assert_eq!(
      pseudo_class_selector(":nth-child(2)"),
      Ok((
        "",
        SimpleSelector::PseudoClass(PseudoClassSelector {
          name: "nth-child".to_string(),
          argument: Some("2".to_string()),
          subtree: None,
        })
      ))
    );
    assert_eq!(
      pseudo_class_selector(":not(.class)"),
      Ok((
        "",
        SimpleSelector::PseudoClass(PseudoClassSelector {
          name: "not".to_string(),
          argument: None,
          subtree: Some(Selector(vec![(
            CompoundSelector(vec![SimpleSelector::Class("class".to_string())]),
            None
          ),])),
        })
      ))
    );
  }

  #[test]
  fn test_pseudo_element_selector() {
    assert_eq!(
      pseudo_element_selector("::before"),
      Ok((
        "",
        SimpleSelector::PseudoElement(PseudoElementSelector {
          name: "before".to_string(),
        })
      ))
    );
  }

  #[test]
  fn test_selector_sequence() {
    assert_eq!(
      selector_sequence("div, a, .class, #id"),
      Ok((
        "",
        vec![
          Selector(vec![(
            CompoundSelector(vec![SimpleSelector::Type("div".to_string())]),
            None
          )]),
          Selector(vec![(
            CompoundSelector(vec![SimpleSelector::Type("a".to_string())]),
            None
          )]),
          Selector(vec![(
            CompoundSelector(vec![SimpleSelector::Class("class".to_string())]),
            None
          )]),
          Selector(vec![(
            CompoundSelector(vec![SimpleSelector::Id("id".to_string())]),
            None
          )]),
        ]
      ))
    );
    assert_eq!(
      selector_sequence("div.class"),
      Ok((
        "",
        vec![Selector(vec![(
          CompoundSelector(vec![
            SimpleSelector::Type("div".to_string()),
            SimpleSelector::Class("class".to_string()),
          ]),
          None
        )])]
      ))
    );
    assert_eq!(
      selector_sequence("div , a"),
      Ok((
        "",
        vec![
          Selector(vec![(
            CompoundSelector(vec![SimpleSelector::Type("div".to_string())]),
            None
          )]),
          Selector(vec![(
            CompoundSelector(vec![SimpleSelector::Type("a".to_string())]),
            None
          )]),
        ]
      ))
    )
  }
}
