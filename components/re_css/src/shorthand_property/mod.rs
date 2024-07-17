mod margin;
mod utils;

use margin::expand_margin;

use crate::{
  parser::structure::ComponentValue,
  value::{property::Property, Value},
};

pub type ExpandedProperty = Vec<(Property, Option<Value>)>;

pub fn get_expander_shorthand_property(
  property: &str,
) -> Option<&dyn Fn(Vec<ComponentValue>) -> Option<ExpandedProperty>> {
  match property {
    "margin" => Some(&expand_margin),
    _ => None,
  }
}
