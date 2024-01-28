use nom::bytes::complete::tag;
use nom::character::complete::alpha0;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

use super::css_value::css_value;
use super::css_value::CssValue;

use super::utility::space_with_newline;

#[derive(Debug, PartialEq)]
pub struct Declaration {
  pub name: String,
  pub value: Vec<CssValue>,
  pub important: bool,
}

impl Declaration {
  pub fn new(name: String) -> Self {
    Self {
      name,
      value: Vec::new(),
      important: false,
    }
  }
}

fn declaration_name(input: &str) -> IResult<&str, String> {
  map(
    tuple((alpha1, many0(tuple((char('-'), alpha1))), alpha0)),
    |(s1, s2, s3)| {
      let mut name = String::from(s1);
      for (c, s) in s2 {
        name.push(c);
        name.push_str(s);
      }
      name.push_str(s3);
      name
    },
  )(input)
}

fn important(input: &str) -> IResult<&str, bool> {
  map(tuple((char('!'), alpha1)), |(_, s)| s == "important")(input)
}

pub fn declaration_list(input: &str) -> IResult<&str, Vec<Declaration>> {
  map(
    tuple((
      separated_list1(tuple((space0, tag(";"), space0)), declaration),
      opt(tag(";")),
      opt(space_with_newline),
    )),
    |(declarations, _, _)| declarations,
  )(input)
}

pub fn declaration(input: &str) -> IResult<&str, Declaration> {
  map(
    tuple((
      declaration_name,
      space0,
      char(':'),
      space0,
      many1(css_value),
      opt(tuple((space1, important))),
    )),
    |(name, _, _, _, value, important)| {
      let mut declaration = Declaration::new(name);
      declaration.value = value;
      declaration.important = important.is_some();
      declaration
    },
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_declaration_name() {
    assert_eq!(declaration_name("color"), Ok(("", String::from("color"))));
    assert_eq!(
      declaration_name("background-color"),
      Ok(("", String::from("background-color")))
    );
    assert_eq!(
      declaration_name("border-radius"),
      Ok(("", String::from("border-radius")))
    );
  }

  #[test]
  fn test_important() {
    assert_eq!(important("!important"), Ok(("", true)));
  }

  #[test]
  fn test_declaration() {
    assert_eq!(
      declaration("color: red"),
      Ok((
        "",
        Declaration {
          name: String::from("color"),
          value: vec![CssValue::Keyword(String::from("red"))],
          important: false,
        }
      ))
    );
    assert_eq!(
      declaration("color: red !important"),
      Ok((
        "",
        Declaration {
          name: String::from("color"),
          value: vec![CssValue::Keyword(String::from("red"))],
          important: true,
        }
      ))
    );
  }

  #[test]
  fn test_declaration_list() {
    assert_eq!(
      declaration_list("color: red; background-color: blue"),
      Ok((
        "",
        vec![
          Declaration {
            name: String::from("color"),
            value: vec![CssValue::Keyword(String::from("red"))],
            important: false,
          },
          Declaration {
            name: String::from("background-color"),
            value: vec![CssValue::Keyword(String::from("blue"))],
            important: false,
          },
        ]
      ))
    );
    assert_eq!(
      declaration_list("color: red; background-color: blue;"),
      Ok((
        "",
        vec![
          Declaration {
            name: String::from("color"),
            value: vec![CssValue::Keyword(String::from("red"))],
            important: false,
          },
          Declaration {
            name: String::from("background-color"),
            value: vec![CssValue::Keyword(String::from("blue"))],
            important: false,
          },
        ]
      ))
    );
  }
}
