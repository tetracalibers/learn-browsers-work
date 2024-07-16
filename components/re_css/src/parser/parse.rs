use css::parser::selector::selector_list;
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
  QualifiedRule, SimpleBlock, StyleRule,
};
use crate::token::{prelude::*, Bracket};

fn trimed<'a, F, O>(parser: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
  F: FnMut(&'a str) -> IResult<&'a str, O>,
{
  delimited(multispace0, parser, multispace0)
}

fn blocked<'a, F, O>(content: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
  F: FnMut(&'a str) -> IResult<&'a str, O>,
{
  delimited(tag("{"), trimed(content), tag("}"))
}

pub fn rules(input: &str) -> IResult<&str, Vec<CSSRule>> {
  many1(trimed(alt((at_rule_as_rule, style_rule_as_rule))))(input)
}

/* 生成規則 --------------------------------------- */

// ref: https://triple-underscore.github.io/css-syntax-ja.html#typedef-block-contents

// 宣言は許容され，［ at-規則, 有修飾規則 ］は無効
fn declaration_list(input: &str) -> IResult<&str, Vec<Declaration>> {
  many0(trimed(terminated(declaration, tag(";"))))(input)
}

// 有修飾規則は許容され，［ 宣言, at-規則 ］は無効
fn qualified_rule_list(input: &str) -> IResult<&str, Vec<BlockContent>> {
  many0(qualified_rule_as_content)(input)
}

// at-規則は許容され，［ 宣言, 有修飾規則 ］は無効
fn at_rule_list(input: &str) -> IResult<&str, Vec<BlockContent>> {
  many0(at_rule_as_content)(input)
}

// ［ 宣言, at-規則 ］は許容され， 有修飾規則は無効
fn declaration_rule_list(input: &str) -> IResult<&str, Vec<BlockContent>> {
  many0(alt((
    at_rule_as_content,
    terminated(declaration_as_content, tag(";")),
  )))(input)
}

// ［ 有修飾規則, at-規則 ］は許容され， 宣言は無効
fn rule_list(input: &str) -> IResult<&str, Vec<BlockContent>> {
  many0(alt((at_rule_as_content, qualified_rule_as_content)))(input)
}

/* 構成要素 --------------------------------------- */

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

// ref: https://triple-underscore.github.io/css-syntax-ja.html#style-rules
fn style_rule(input: &str) -> IResult<&str, StyleRule> {
  map(
    tuple((selector_list, trimed(blocked(declaration_list)))),
    |(selectors, declarations)| StyleRule {
      selectors,
      declarations,
    },
  )(input)
}
fn style_rule_as_rule(input: &str) -> IResult<&str, CSSRule> {
  map(style_rule, CSSRule::StyleRule)(input)
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
  blocked(many0(alt((
    trimed(terminated(declaration_as_content, tag(";"))),
    at_rule_as_content,
    qualified_rule_as_content,
  ))))(input)
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
