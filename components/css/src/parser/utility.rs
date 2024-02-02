use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::alpha0;
use nom::character::complete::alpha1;
use nom::character::complete::char;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many0;
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
