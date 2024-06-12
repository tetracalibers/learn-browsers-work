// ref: https://developer.mozilla.org/ja/docs/Web/CSS/display
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Display {
  FullValue(DisplayOutside, DisplayInside), // ref: https://developer.mozilla.org/ja/docs/Web/CSS/display/multi-keyword_syntax_of_display
  Box(DisplayBox),
}

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/display-outside
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DisplayOutside {
  Block,
  Inline,
}

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/display-inside
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DisplayInside {
  Flow,
  FlowRoot,
  Table,
  Flex,
  Grid,
  Ruby,
}

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/display-box
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DisplayBox {
  Contents,
  None,
}
