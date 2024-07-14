use css_defs::{
  property::{Properties, Property},
  value::Value,
};
use fast_dom::node::NodePtr;
use strum::IntoEnumIterator;

fn to_specified_values(node: &NodePtr, styles: &mut Properties) {
  let inherit = |property: Property| {
    if let Some(parent) = &node.parent() {
      return (property.clone(), parent.get_style(&property));
    }

    return (property.clone(), Value::initial(&property));
  };

  let specified_values = Property::iter()
    .map(|property| {
      if let Some(value) = styles.get(&property) {
        // Explicit defaulting
        match value {
          Value::Initial => {
            return (property, Value::initial(&property));
          }
          Value::Inherit => {
            return inherit(property);
          }
          Value::Unset => {
            if property.inheritable() {
              return inherit(property);
            }
            return (property, Value::initial(&property));
          }
          _ => {
            return (property, value.clone());
          }
        }
      }

      if property.inheritable() {
        return inherit(property);
      }

      return (property, Value::initial(&property));
    })
    .collect::<Properties>();

  for (property, value) in specified_values {
    styles.insert(property, value);
  }
}
