use border_color::expand_border_color;
use css::structs::declaration_value::DeclarationValue;

use crate::{property::Property, value::Value};

pub type ExpandedProperty = Option<Vec<(Property, Option<Value>)>>;

mod border_color;

pub(crate) mod prelude {
  pub use super::border_color::expand_border_color;
  pub use super::ExpandedProperty;
}

pub fn get_expander_shorthand_property(
  property: &str,
) -> Option<&dyn Fn(&[&[DeclarationValue]]) -> ExpandedProperty> {
  match property {
    "border-color" => Some(&expand_border_color),
    _ => None,
  }
}
