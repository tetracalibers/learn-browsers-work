use crate::{property::Property, value::Value};

pub type ExpandedProperty = Option<Vec<(Property, Option<Value>)>>;

mod border_color;

pub(crate) mod prelude {
  pub use super::border_color::expand_border_color;
  pub use super::ExpandedProperty;
}
