use nom::branch::alt;
use nom::bytes::complete::escaped_transform;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha0;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::none_of;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::value;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

pub fn space_with_newline(input: &str) -> IResult<&str, &str> {
  map(tuple((space0, opt(newline), space0)), |(_, _, _)| "")(input)
}

// ハイフンを含むalpha1（開始文字と終了文字はalpha1）
pub fn alpha1_with_hyphen(input: &str) -> IResult<&str, String> {
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

pub fn double_quoted(input: &str) -> IResult<&str, &str> {
  map(
    tuple((tag("\""), take_until("\""), tag("\""))),
    |(_, s, _)| s,
  )(input)
}

pub fn single_quoted(input: &str) -> IResult<&str, &str> {
  map(tuple((tag("'"), take_until("'"), tag("'"))), |(_, s, _)| s)(input)
}

pub fn quoted(input: &str) -> IResult<&str, &str> {
  alt((double_quoted, single_quoted))(input)
}

// 例："this is a \"string\"."
pub fn double_quoted_within_esceped_quote(
  input: &str,
) -> IResult<&str, String> {
  delimited(
    tag("\""),
    escaped_transform(none_of("\\\""), '\\', value("\"", tag("\""))),
    tag("\""),
  )(input)
}

pub fn single_quoted_within_esceped_quote(
  input: &str,
) -> IResult<&str, String> {
  delimited(
    tag("'"),
    escaped_transform(none_of("\\'"), '\\', value("'", tag("'"))),
    tag("'"),
  )(input)
}

pub fn quoted_within_esceped_quote(input: &str) -> IResult<&str, String> {
  alt((
    double_quoted_within_esceped_quote,
    single_quoted_within_esceped_quote,
  ))(input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_space_with_newline() {
    assert_eq!(space_with_newline(" \n "), Ok(("", "")));
    assert_eq!(space_with_newline(" "), Ok(("", "")));
    assert_eq!(space_with_newline("\n"), Ok(("", "")));
  }

  #[test]
  fn test_alpha1_with_hyphen() {
    assert_eq!(alpha1_with_hyphen("a"), Ok(("", "a".to_string())));
    assert_eq!(alpha1_with_hyphen("a-b"), Ok(("", "a-b".to_string())));
    assert_eq!(alpha1_with_hyphen("a-b-c"), Ok(("", "a-b-c".to_string())));
  }

  #[test]
  fn test_double_quoted_within_esceped_quote() {
    assert_eq!(
      double_quoted_within_esceped_quote(r#""this is a 'string'.""#),
      Ok(("", "this is a 'string'.".to_string()))
    );
    assert_eq!(
      double_quoted_within_esceped_quote(r#""this is a \"string\".""#),
      Ok(("", "this is a \"string\".".to_string()))
    );
  }

  #[test]
  fn test_single_quoted_within_esceped_quote() {
    assert_eq!(
      single_quoted_within_esceped_quote(r#"'this is a "string".'"#),
      Ok(("", "this is a \"string\".".to_string()))
    );
    assert_eq!(
      single_quoted_within_esceped_quote(r#"'this is a \'string\'.'"#),
      Ok(("", "this is a 'string'.".to_string()))
    );
  }
}
