use nom::{
  branch::alt,
  bytes::complete::tag_no_case,
  character::complete::{char, hex_digit1},
  combinator::recognize,
  multi::many_m_n,
  sequence::{pair, preceded},
  IResult,
};

fn hex_digit_1_to_6(input: &str) -> IResult<&str, &str> {
  recognize(many_m_n(1, 6, hex_digit1))(input)
}

fn hex_digit_1_to_5(input: &str) -> IResult<&str, &str> {
  recognize(many_m_n(1, 5, hex_digit1))(input)
}

fn question_mark_1_to_6(input: &str) -> IResult<&str, &str> {
  recognize(many_m_n(1, 6, char('?')))(input)
}

fn unicode_range_1(input: &str) -> IResult<&str, &str> {
  recognize(preceded(
    pair(tag_no_case("U"), char('+')),
    hex_digit_1_to_6,
  ))(input)
}

fn unicode_range_2(input: &str) -> IResult<&str, &str> {
  recognize(preceded(
    pair(tag_no_case("U"), char('+')),
    pair(hex_digit_1_to_5, question_mark_1_to_6),
  ))(input)
}

fn unicode_range_3(input: &str) -> IResult<&str, &str> {
  recognize(preceded(
    pair(tag_no_case("U"), char('+')),
    pair(hex_digit_1_to_6, preceded(char('-'), hex_digit_1_to_6)),
  ))(input)
}

pub fn unicode_range_token(input: &str) -> IResult<&str, &str> {
  alt((unicode_range_3, unicode_range_2, unicode_range_1))(input)
}
