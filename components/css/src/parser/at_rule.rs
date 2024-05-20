use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha1;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

use crate::structs::at_rule::AtRule;
use crate::structs::at_rule::RegularAtRule;

fn at_rule_identifier(input: &str) -> IResult<&str, &str> {
  let identifier_name = tuple((alpha1, opt(tuple((tag("-"), alpha1)))));

  map(tuple((tag("@"), identifier_name)), |(_, (name, _))| name)(input)
}

fn regular_at_rule(input: &str) -> IResult<&str, AtRule> {
  map(
    tuple((at_rule_identifier, space1, take_until(";"), tag(";"))),
    |(name, _, value, _)| {
      AtRule::Regular(RegularAtRule {
        name: name.to_string(),
        value: value.to_string(),
      })
    },
  )(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_at_rule_identifier() {
    assert_eq!(at_rule_identifier("@import"), Ok(("", "import")));
  }

  #[test]
  fn test_regular_at_rule() {
    assert_eq!(
      regular_at_rule("@import url(\"style.css\");"),
      Ok((
        "",
        AtRule::Regular(RegularAtRule {
          name: "import".to_string(),
          value: "url(\"style.css\")".to_string(),
        })
      ))
    );
  }
}
