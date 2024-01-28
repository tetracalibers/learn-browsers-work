use super::declaration::declaration_list;
use super::declaration::Declaration;

use super::selector::selector_list;
use super::selector::SelectorList;

use nom::character::complete::char;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct StyleRule {
  selector: SelectorList,
  declarations: Vec<Declaration>,
}

pub fn style_rule(input: &str) -> IResult<&str, StyleRule> {
  map(
    tuple((
      selector_list,
      tuple((space0, char('{'), space0)),
      declaration_list,
      tuple((space0, char('}'), space0)),
    )),
    |(selector, _, declarations, _)| StyleRule {
      selector,
      declarations,
    },
  )(input)
}

#[cfg(test)]
mod tests {
  use std::vec;

  use super::super::css_value::CssValue;
  use super::super::selector::CompoundSelector;
  use super::super::selector::SimpleSelector;
  use super::*;

  #[test]
  fn test_style_rule() {
    assert_eq!(
      style_rule("h1 { font-weight: bold; }"),
      Ok((
        "",
        StyleRule {
          selector: vec![vec![(
            CompoundSelector(vec![SimpleSelector::Type("h1".to_string())]),
            None
          )],],
          declarations: vec![Declaration {
            name: "font-weight".to_string(),
            value: vec![CssValue::Keyword("bold".to_string())],
            important: false,
          }],
        }
      ))
    );
  }
}
