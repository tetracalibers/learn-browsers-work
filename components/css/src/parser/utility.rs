use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::combinator::map;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;

pub fn space_with_newline(input: &str) -> IResult<&str, &str> {
  map(tuple((space0, opt(newline), space0)), |(_, _, _)| "")(input)
}
