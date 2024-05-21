mod at_rule;
mod component_value;
mod declaration;
pub mod selector;
mod style_rule;
pub mod stylesheet;
mod utility;

use crate::cssom::stylesheet::StyleSheet;

pub fn parse_css(input: &str) -> Result<StyleSheet, String> {
  match stylesheet::stylesheet(input) {
    Ok((_, stylesheet)) => Ok(stylesheet),
    Err(e) => Err(format!("{:?}", e)),
  }
}
