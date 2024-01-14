use nom::{
  branch::alt,
  bytes::complete::{tag, take_until, take_while, take_while1},
  character::complete::{char, multispace0},
  combinator::{map, opt, value},
  multi::{many0, separated_list1},
  sequence::{preceded, terminated},
  IResult,
};

#[derive(Debug)]
pub struct Stylesheet {
  rules: Vec<StyleRule>,
}

#[derive(Debug)]
pub struct StyleRule {
  selectors: Vec<Selector>,
  declarations: Vec<Declaration>,
}

#[derive(Debug)]
pub enum Selector {
  Simple(SimpleSelector),
  Compound(Vec<SimpleSelector>),
}

#[derive(Debug, Clone)]
pub enum SimpleSelector {
  Type(String),
  Universal,
  Id(String),
  Class(String),
  Attribute(AttributeSelector),
  Pseudo(String),
}

#[derive(Debug, Clone)]
pub struct AttributeSelector {
  name: String,
  operator: Option<AttributeOperator>,
  value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AttributeOperator {
  Equal,
  Contains,
  StartsWith,
  EndsWith,
}

#[derive(Debug)]
pub struct Declaration {
  property: String,
  value: String,
}

fn identifier(input: &str) -> IResult<&str, &str> {
  take_while1(|c: char| c.is_alphanumeric() || c == '-' || c == '_')(input)
}

fn combinator(input: &str) -> IResult<&str, &str> {
  take_while(|c: char| c.is_whitespace())(input)
}

fn simple_selector_type(input: &str) -> IResult<&str, SimpleSelector> {
  alt((
    map(identifier, |name| SimpleSelector::Type(name.to_string())),
    value(SimpleSelector::Universal, char('*')),
    map(preceded(char('#'), identifier), |id| {
      SimpleSelector::Id(id.to_string())
    }),
    map(preceded(char('.'), identifier), |class| {
      SimpleSelector::Class(class.to_string())
    }),
    map(attribute_selector, |attribute| {
      SimpleSelector::Attribute(attribute)
    }),
    map(preceded(char(':'), identifier), |pseudo| {
      SimpleSelector::Pseudo(pseudo.to_string())
    }),
  ))(input)
}

fn attribute_selector(input: &str) -> IResult<&str, AttributeSelector> {
  let (input, name) = identifier(input)?;
  let (input, operator) = opt(attribute_operator)(input)?;
  let (input, value) = opt(preceded(char('='), identifier))(input)?;

  Ok((
    input,
    AttributeSelector {
      name: name.to_string(),
      operator,
      value: value.map(|v| v.to_string()),
    },
  ))
}

fn attribute_operator(input: &str) -> IResult<&str, AttributeOperator> {
  alt((
    value(AttributeOperator::Equal, char('=')),
    value(AttributeOperator::Contains, tag("~=")),
    value(AttributeOperator::StartsWith, tag("^=")),
    value(AttributeOperator::EndsWith, tag("$=")),
  ))(input)
}

fn compound_selector(input: &str) -> IResult<&str, Vec<SimpleSelector>> {
  separated_list1(char(','), preceded(multispace0, simple_selector_type))(input)
}

fn selector(input: &str) -> IResult<&str, Selector> {
  alt((
    map(simple_selector_type, Selector::Simple),
    map(compound_selector, Selector::Compound),
  ))(input)
}

fn declaration(input: &str) -> IResult<&str, Declaration> {
  let (input, property) = take_until(":")(input)?;
  let (input, _) = char(':')(input)?;
  let (input, value) = take_until(";")(input)?;
  let (input, _) = char(';')(input)?;

  Ok((
    input,
    Declaration {
      property: property.trim().to_string(),
      value: value.trim().to_string(),
    },
  ))
}

fn style_rule(input: &str) -> IResult<&str, StyleRule> {
  let (input, selectors) =
    terminated(separated_list1(combinator, selector), char('{'))(input)?;
  let (input, declarations) = separated_list1(multispace0, declaration)(input)?;
  let (input, _) = char('}')(input)?;

  Ok((
    input,
    StyleRule {
      selectors,
      declarations,
    },
  ))
}

pub fn stylesheet(input: &str) -> IResult<&str, Stylesheet> {
  let (input, rules) = many0(style_rule)(input)?;

  Ok((input, Stylesheet { rules }))
}

pub fn main() {
  let (_, result) = selector(r#"[attribute^="x"]"#).unwrap();

  println!("{:?}", result);
}
