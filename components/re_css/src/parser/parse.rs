use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::multispace0,
  combinator::{map, opt},
  multi::{many0, many1, separated_list1},
  sequence::{delimited, pair, terminated, tuple},
  IResult,
};

use super::structure::{
  AtRule, BlockContent, CSSRule, ComponentValue, Declaration, Function,
  QualifiedRule, SimpleBlock,
};
use crate::token::{prelude::*, Bracket};

fn trimed<'a, F, O>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
  F: FnMut(&'a str) -> IResult<&'a str, O>,
{
  delimited(multispace0, parser, multispace0)
}

pub fn rules(input: &str) -> IResult<&str, Vec<CSSRule>> {
  many1(trimed(alt((at_rule_as_rule, qualified_rule_as_rule))))(input)
}

fn at_rule_as_rule(input: &str) -> IResult<&str, CSSRule> {
  map(at_rule, CSSRule::AtRule)(input)
}
fn at_rule_as_content(input: &str) -> IResult<&str, BlockContent> {
  map(at_rule, BlockContent::AtRule)(input)
}
fn at_rule(input: &str) -> IResult<&str, AtRule> {
  alt((regular_at_rule, nested_at_rule))(input)
}
fn regular_at_rule(input: &str) -> IResult<&str, AtRule> {
  map(
    tuple((at_keyword_str, many0(component_value), tag(";"))),
    |(name, prelude, _)| AtRule {
      name,
      prelude,
      block: None,
    },
  )(input)
}
fn nested_at_rule(input: &str) -> IResult<&str, AtRule> {
  map(
    tuple((at_keyword_str, many0(component_value), simple_curly_block)),
    |(name, prelude, block)| AtRule {
      name,
      prelude,
      block: Some(block),
    },
  )(input)
}

fn qualified_rule_as_rule(input: &str) -> IResult<&str, CSSRule> {
  map(qualified_rule, CSSRule::QualifiedRule)(input)
}
fn qualified_rule_as_content(input: &str) -> IResult<&str, BlockContent> {
  map(qualified_rule, BlockContent::QualifiedRule)(input)
}
fn qualified_rule(input: &str) -> IResult<&str, QualifiedRule> {
  map(
    tuple((many1(component_value), trimed(curly_block_content))),
    |(prelude, block)| QualifiedRule { prelude, block },
  )(input)
}

fn curly_block_content(input: &str) -> IResult<&str, Vec<BlockContent>> {
  map(
    tuple((
      tag("{"),
      multispace0,
      many0(alt((
        trimed(terminated(declaration_as_content, tag(";"))),
        at_rule_as_content,
        qualified_rule_as_content,
      ))),
      multispace0,
      tag("}"),
    )),
    |(_, _, content, _, _)| content,
  )(input)
}

fn declaration_as_content(input: &str) -> IResult<&str, BlockContent> {
  map(declaration, BlockContent::Declaration)(input)
}
fn declaration(input: &str) -> IResult<&str, Declaration> {
  map(
    tuple((
      ident_token_str,
      multispace0,
      tag(":"),
      many1(component_value),
      opt(important),
    )),
    |(name, _, _, value, important_opt)| Declaration {
      name,
      value,
      important: important_opt.is_some(),
    },
  )(input)
}

fn important(input: &str) -> IResult<&str, (&str, &str)> {
  pair(tag("!"), trimed(tag("important")))(input)
}

fn component_value(input: &str) -> IResult<&str, ComponentValue> {
  alt((
    simple_block_as_component,
    function_as_component,
    trimed(preserved_token),
  ))(input)
}

fn simple_block_as_component(input: &str) -> IResult<&str, ComponentValue> {
  map(simple_block, ComponentValue::SimpleBlock)(input)
}
fn simple_block(input: &str) -> IResult<&str, SimpleBlock> {
  alt((simple_curly_block, simple_square_block, simple_round_block))(input)
}
fn simple_curly_block(input: &str) -> IResult<&str, SimpleBlock> {
  map(
    delimited(tag("{"), many0(component_value), tag("}")),
    |value| SimpleBlock {
      associated: Bracket::Curly,
      value,
    },
  )(input)
}
fn simple_square_block(input: &str) -> IResult<&str, SimpleBlock> {
  map(
    delimited(tag("["), many0(component_value), tag("]")),
    |value| SimpleBlock {
      associated: Bracket::Square,
      value,
    },
  )(input)
}
fn simple_round_block(input: &str) -> IResult<&str, SimpleBlock> {
  map(
    delimited(tag("("), many0(component_value), tag(")")),
    |value| SimpleBlock {
      associated: Bracket::Round,
      value,
    },
  )(input)
}

fn function_as_component(input: &str) -> IResult<&str, ComponentValue> {
  map(function, ComponentValue::Function)(input)
}
fn function(input: &str) -> IResult<&str, Function> {
  map(
    tuple((
      function_token_str,
      separated_list1(tag(","), component_value),
      tag(")"),
    )),
    |(name, value, _)| Function { name, value },
  )(input)
}

fn preserved_token(input: &str) -> IResult<&str, ComponentValue> {
  map(
    alt((
      // composite
      dimension_token,
      percentage_token,
      url_token,
      at_keyword_token,
      // atomic
      escape_token,
      hash_token,
      ident_token,
      number_token,
      string_token,
      unicode_range_token,
    )),
    ComponentValue::PreservedToken,
  )(input)
}
