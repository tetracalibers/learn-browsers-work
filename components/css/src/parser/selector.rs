use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{alpha1, space0, space1},
  combinator::{opt, peek, value},
  multi::many0,
  sequence::{delimited, tuple},
  IResult,
};

use crate::structs::selector::AttributeOperator;
use crate::structs::selector::AttributeSelector;
use crate::structs::selector::Combinator;
use crate::structs::selector::ComplexSelector;
use crate::structs::selector::ComplexSelectorSequence;
use crate::structs::selector::CompoundSelector;
use crate::structs::selector::PseudoClassSelector;
use crate::structs::selector::PseudoElementSelector;
use crate::structs::selector::Selector;
use crate::structs::selector::SelectorList;
use crate::structs::selector::SimpleSelector;

use super::utility::double_quoted;

fn parenthesized<'a, F, O>(
  parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
  F: FnMut(&'a str) -> IResult<&'a str, O>,
{
  delimited(tag("("), parser, tag(")"))
}

fn identifier(input: &str) -> IResult<&str, String> {
  map(
    tuple((
      alpha1::<&str, _>,
      opt(tuple((many0(alt((tag("-"), tag("_")))), alphanumeric1))),
    )),
    |(s1, o1)| {
      if let Some((v1, s2)) = o1 {
        format!("{}{}{}", s1, v1.join(""), s2)
      } else {
        s1.to_string()
      }
    },
  )(input)
}

fn type_selector(input: &str) -> IResult<&str, SimpleSelector> {
  map(identifier, |name| SimpleSelector::Type(name))(input)
}

fn universal_selector(input: &str) -> IResult<&str, SimpleSelector> {
  map(tag("*"), |_| SimpleSelector::Universal)(input)
}

fn id_selector(input: &str) -> IResult<&str, SimpleSelector> {
  map(tuple((tag("#"), identifier)), |(_, name)| {
    SimpleSelector::Id(name)
  })(input)
}

fn class_selector(input: &str) -> IResult<&str, SimpleSelector> {
  map(tuple((tag("."), identifier)), |(_, name)| {
    SimpleSelector::Class(name)
  })(input)
}

fn attribute_operator(input: &str) -> IResult<&str, AttributeOperator> {
  alt((
    value(AttributeOperator::Equal, tag("=")),
    value(AttributeOperator::DashMatch, tag("|=")),
    value(AttributeOperator::Contains, tag("~=")),
    value(AttributeOperator::Substring, tag("*=")),
    value(AttributeOperator::StartsWith, tag("^=")),
    value(AttributeOperator::EndsWith, tag("$=")),
  ))(input)
}

fn attribute_selector(input: &str) -> IResult<&str, SimpleSelector> {
  map(
    tuple((
      tag("["),
      identifier,
      opt(attribute_operator),
      opt(double_quoted),
      tag("]"),
    )),
    |(_, name, operator, value, _)| {
      SimpleSelector::Attribute(AttributeSelector {
        name,
        operator,
        value: value.map(|v| v.to_string()),
      })
    },
  )(input)
}

fn pseudo_class_selector(input: &str) -> IResult<&str, SimpleSelector> {
  map(
    tuple((
      tag(":"),
      identifier,
      opt(parenthesized(alphanumeric1)),
      opt(parenthesized(selector_list)),
    )),
    |(_, name, argument, subtree)| {
      SimpleSelector::PseudoClass(PseudoClassSelector {
        name,
        argument: argument.map(|v| v.to_string()),
        subtree,
      })
    },
  )(input)
}

fn pseudo_element_selector(input: &str) -> IResult<&str, SimpleSelector> {
  map(tuple((tag("::"), identifier)), |(_, name)| {
    SimpleSelector::PseudoElement(PseudoElementSelector { name })
  })(input)
}

fn simple_selector(input: &str) -> IResult<&str, SimpleSelector> {
  alt((
    id_selector,
    class_selector,
    attribute_selector,
    pseudo_element_selector,
    pseudo_class_selector,
    universal_selector,
    type_selector,
  ))(input)
}

fn compound_selector(input: &str) -> IResult<&str, CompoundSelector> {
  map(many1(simple_selector), |selectors| {
    CompoundSelector(selectors)
  })(input)
}

fn combinator(input: &str) -> IResult<&str, Combinator> {
  alt((
    value(
      Combinator::Child,
      tuple((space0, tag(">"), space0, peek(compound_selector))),
    ),
    value(
      Combinator::NextSibling,
      tuple((space0, tag("+"), space0, peek(compound_selector))),
    ),
    value(
      Combinator::SubsequentSibling,
      tuple((space0, tag("~"), space0, peek(compound_selector))),
    ),
    value(
      Combinator::Descendant,
      tuple((space1, peek(compound_selector))),
    ), // 他の記号がスペースで囲まれている場合にマッチしないよう最後に置く
  ))(input)
}

// cobminatorで繋がれた2つのcompound_selectorをparseする
fn complex_selector(input: &str) -> IResult<&str, ComplexSelector> {
  map(
    tuple((compound_selector, combinator)),
    |(selector, combinator)| (selector, Some(combinator)),
  )(input)
}

fn complex_selector_sequence(
  input: &str,
) -> IResult<&str, ComplexSelectorSequence> {
  map(
    tuple((many0(complex_selector), compound_selector)),
    |(selectors, selector)| {
      let mut selectors = selectors
        .into_iter()
        .filter(|(s, _)| !s.is_empty())
        .collect::<Vec<_>>();
      selectors.push((selector, None));
      selectors
    },
  )(input)
}

pub fn selector(input: &str) -> IResult<&str, Selector> {
  map(complex_selector_sequence, |selectors| Selector(selectors))(input)
}

pub fn selector_list(input: &str) -> IResult<&str, SelectorList> {
  map(
    tuple((
      separated_list1(tuple((space0, char(','), space0)), selector),
      opt(tuple((space0, char(','), space0))),
    )),
    |(selectors, _)| selectors,
  )(input)
}

pub fn main() {
  let input =
    r#"#foo > .bar + div.k1.k2 [id="baz"]:hello(2):not(:where(#yolo))::before"#;

  let (rest, result) = selector_list(input).unwrap();

  println!("rest: {:?}", rest);
  println!("result: {:?}", result);
}

#[cfg(test)]
mod tests {
  use std::vec;

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
  fn test_selector() {
    assert_eq!(
      complex_selector_sequence("div.class #id"),
      Ok((
        "",
        vec![
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
        ]
      ))
    );
    assert_eq!(
      complex_selector_sequence("div.class > #id"),
      Ok((
        "",
        vec![
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
        ]
      ))
    );
    assert_eq!(
      complex_selector_sequence(":where(#yoro)"),
      Ok((
        "",
        vec![(
          CompoundSelector(vec![SimpleSelector::PseudoClass(
            PseudoClassSelector {
              name: "where".to_string(),
              argument: None,
              subtree: Some(vec![Selector(vec![(
                CompoundSelector(vec![SimpleSelector::Id("yoro".to_string())]),
                None
              )])]),
            }
          )]),
          None
        )]
      ))
    );
    assert_eq!(
      complex_selector_sequence("input:not(:where(#yolo))"),
      Ok((
        "",
        vec![(
          CompoundSelector(vec![
            SimpleSelector::Type("input".to_string()),
            SimpleSelector::PseudoClass(PseudoClassSelector {
              name: "not".to_string(),
              argument: None,
              subtree: Some(vec![Selector(vec![(
                CompoundSelector(vec![SimpleSelector::PseudoClass(
                  PseudoClassSelector {
                    name: "where".to_string(),
                    argument: None,
                    subtree: Some(vec![Selector(vec![(
                      CompoundSelector(vec![SimpleSelector::Id(
                        "yolo".to_string()
                      )]),
                      None
                    )])])
                  }
                )]),
                None
              )])])
            })
          ]),
          None
        )]
      ))
    );
    assert_eq!(
      complex_selector_sequence("a:hover::before"),
      Ok((
        "",
        vec![(
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
        )]
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
          subtree: Some(vec![Selector(vec![(
            CompoundSelector(vec![SimpleSelector::Class("class".to_string())]),
            None
          ),])]),
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
  fn test_selector_list() {
    assert_eq!(
      selector_list("div, a, .class, #id"),
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
      selector_list("div.class"),
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
      selector_list("div , a"),
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
