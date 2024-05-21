use crate::cssom::stylesheet::Rule;
use crate::cssom::stylesheet::StyleSheet;

use super::style_rule::style_rule;
use super::utility::space_with_newline;

use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

pub fn stylesheet(input: &str) -> IResult<&str, StyleSheet> {
  map(
    many0(tuple((
      space_with_newline,
      map(style_rule, |rule| Rule::StyleRule(rule)),
      space_with_newline,
    ))),
    |result| StyleSheet {
      rules: result.into_iter().map(|(_, rule, _)| rule).collect(),
    },
  )(input.trim())
}

pub fn main() {
  let sample = r#"    
    * {
      display: inline;
    }
    
    .aaa {
      display: none;
    }
  "#;

  println!("{:#?}", stylesheet(sample));
}
