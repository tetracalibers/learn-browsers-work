use crate::cssom::style_rule::StyleRule;

use super::declaration::declaration_list;
use super::selector::selector_list;
use super::utility::space_with_newline;

use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

pub fn style_rule(input: &str) -> IResult<&str, StyleRule> {
  map(
    tuple((
      space_with_newline,
      selector_list,
      tuple((space_with_newline, char('{'), space_with_newline)),
      declaration_list,
      tuple((space_with_newline, char('}'), space_with_newline)),
    )),
    |(_, selectors, _, declarations, _)| StyleRule {
      selectors,
      declarations,
    },
  )(input)
}

#[cfg(test)]
mod tests {
  use std::vec;

  use super::*;
  use crate::structs::declaration_value::DeclarationValue;
  use crate::structs::{
    declaration::Declaration,
    selector::{CompoundSelector, SimpleSelector},
  };

  #[test]
  fn test_style_rule() {
    assert_eq!(
      style_rule("h1 { font-weight: bold; }"),
      Ok((
        "",
        StyleRule {
          selectors: vec![vec![(
            CompoundSelector(vec![SimpleSelector::Type("h1".to_string())]),
            None
          )],],
          declarations: vec![Declaration {
            name: "font-weight".to_string(),
            value: vec![DeclarationValue::Keyword("bold".to_string())],
            important: false,
          }],
        }
      ))
    );

    assert_eq!(
      style_rule(
        r#"
        h1 {
          font-weight: bold;
        }
        "#
      ),
      Ok((
        "",
        StyleRule {
          selectors: vec![vec![(
            CompoundSelector(vec![SimpleSelector::Type("h1".to_string())]),
            None
          )],],
          declarations: vec![Declaration {
            name: "font-weight".to_string(),
            value: vec![DeclarationValue::Keyword("bold".to_string())],
            important: false,
          }],
        }
      ))
    )
  }
}
