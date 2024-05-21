// ref: https://developer.mozilla.org/ja/docs/Web/CSS/display
#[derive(Debug)]
pub enum Display {
  FullValue(DisplayOutside, DisplayInside), // ref: https://developer.mozilla.org/ja/docs/Web/CSS/display/multi-keyword_syntax_of_display
  Box(DisplayBox),
}

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/display-outside
#[derive(Debug)]
pub enum DisplayOutside {
  Block,
  Inline,
}

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/display-inside
#[derive(Debug)]
pub enum DisplayInside {
  Flow,
  FlowRoot,
  Table,
  Flex,
  Grid,
  Ruby,
}

// ref: https://developer.mozilla.org/ja/docs/Web/CSS/display-box
#[derive(Debug)]
pub enum DisplayBox {
  Contents,
  None,
}
