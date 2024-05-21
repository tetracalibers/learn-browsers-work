use css::structs::selector::Specificity;
use css_defs::{
  context::{CSSLocation, CascadeOrigin},
  value::Value,
};

#[derive(Debug)]
struct PropertyDeclaration {
  pub value: Value,
  pub important: bool,
  pub origin: CascadeOrigin,
  pub location: CSSLocation,
  pub specificity: Specificity,
}
