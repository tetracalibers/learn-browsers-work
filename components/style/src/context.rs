use re_css::parser::structure::StyleRule;

#[derive(Debug)]
pub struct ContextualRule<'a> {
  pub style: StyleRule<'a>,
  pub origin: CascadeOrigin,
  pub location: CSSLocation,
}

/// Location of the CSS applied
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CSSLocation {
  /// Inline CSS (in HTML tags)
  Inline,
  /// Embedded CSS (in HTML style tag)
  Embedded,
  /// External CSS (in external css file)
  External,
}

// ref: https://www.w3.org/TR/css3-cascade/#cascading-origins
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CascadeOrigin {
  Author,
  User,
  UserAgent,
}
