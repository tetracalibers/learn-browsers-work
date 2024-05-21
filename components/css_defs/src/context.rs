/// Location of the CSS applied
#[derive(Debug)]
pub enum CSSLocation {
  /// Inline CSS (in HTML tags)
  Inline,
  /// Embedded CSS (in HTML style tag)
  Embedded,
  /// External CSS (in external css file)
  External,
}

#[derive(Debug)]
pub enum CascadeOrigin {
  Author,
  User,
  UserAgent,
}
