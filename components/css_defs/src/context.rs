use css::cssom::style_rule::StyleRule;

#[derive(Debug)]
pub struct ContextualRule {
  pub inner: StyleRule,
  pub origin: CascadeOrigin,
  pub location: CSSLocation,
}

/// Location of the CSS applied
#[derive(Debug, PartialEq, Eq)]
pub enum CSSLocation {
  /// Inline CSS (in HTML tags)
  Inline,
  /// Embedded CSS (in HTML style tag)
  Embedded,
  /// External CSS (in external css file)
  External,
}

// ref: https://www.w3.org/TR/css3-cascade/#cascading-origins
#[derive(Debug, PartialEq, Eq)]
pub enum CascadeOrigin {
  Author,
  User,
  UserAgent,
}
