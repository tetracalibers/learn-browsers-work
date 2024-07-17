use crate::{parser::structure::ComponentValue, value::property::Property};

use super::{utils::expand_four_box_values, ExpandedProperty};

pub fn expand_margin(values: &[ComponentValue]) -> Option<ExpandedProperty> {
  expand_four_box_values(
    values,
    (
      Property::MarginTop,
      Property::MarginRight,
      Property::MarginBottom,
      Property::MarginLeft,
    ),
  )
}
