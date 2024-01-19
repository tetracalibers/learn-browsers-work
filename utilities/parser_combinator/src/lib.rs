type ParseResult<'a, Output> = Option<(&'a str, Output)>;

pub trait Parser<'a, Output> {
  fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
  F: Fn(&'a str) -> ParseResult<'a, Output>,
{
  fn parse(&self, input: &'a str) -> Option<(&'a str, Output)> {
    self(input)
  }
}

pub fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
  move |input: &'a str| match input.get(0..expected.len()) {
    Some(next) if next == expected => Some((&input[expected.len()..], ())),
    _ => None,
  }
}

pub fn pair<'a, P1, P2, O1, O2>(
  parser1: P1,
  parser2: P2,
) -> impl Parser<'a, (O1, O2)>
where
  P1: Parser<'a, O1>,
  P2: Parser<'a, O2>,
{
  move |input| {
    parser1.parse(input).and_then(|(next_input, result1)| {
      parser2
        .parse(next_input)
        .map(|(last_input, result2)| (last_input, (result1, result2)))
    })
  }
}

pub fn map<'a, P, F, O1, O2>(parser: P, map_fn: F) -> impl Parser<'a, O2>
where
  P: Parser<'a, O1>,
  F: Fn(O1) -> O2,
{
  move |input| {
    parser.parse(input).map(|(next_input, result)| (next_input, map_fn(result)))
  }
}

pub fn left<'a, P1, P2, O1, O2>(parser1: P1, parser2: P2) -> impl Parser<'a, O1>
where
  P1: Parser<'a, O1>,
  P2: Parser<'a, O2>,
{
  map(pair(parser1, parser2), |(left, _right)| left)
}

pub fn right<'a, P1, P2, O1, O2>(
  parser1: P1,
  parser2: P2,
) -> impl Parser<'a, O2>
where
  P1: Parser<'a, O1>,
  P2: Parser<'a, O2>,
{
  map(pair(parser1, parser2), |(_left, right)| right)
}

pub fn zero_or_more<'a, P, O>(parser: P) -> impl Parser<'a, Vec<O>>
where
  P: Parser<'a, O>,
{
  move |mut input| {
    let mut result = Vec::new();

    while let Some((next_input, next_item)) = parser.parse(input) {
      input = next_input;
      result.push(next_item);
    }

    Some((input, result))
  }
}

pub fn one_or_more<'a, P, O>(parser: P) -> impl Parser<'a, Vec<O>>
where
  P: Parser<'a, O> + Clone,
{
  map(
    pair(parser.clone(), zero_or_more(parser)),
    |(head, mut tail)| {
      tail.insert(0, head);
      tail
    },
  )
}

pub fn any_char(input: &str) -> ParseResult<char> {
  input.chars().next().map(|next| (&input[next.len_utf8()..], next))
}

pub fn pred<'a, P, O, F>(parser: P, predicate: F) -> impl Parser<'a, O>
where
  P: Parser<'a, O>,
  F: Fn(&O) -> bool,
{
  move |input| match parser.parse(input) {
    Some((next_input, value)) if predicate(&value) => Some((next_input, value)),
    _ => None,
  }
}

pub fn whitespace_char<'a>() -> impl Parser<'a, char> {
  pred(any_char, |c| c.is_whitespace())
}

/* -------------------------------------------- */
