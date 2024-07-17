use fast_dom::node::NodePtr;
use re_css::value::{
  length::{Length, LengthUnit},
  percentage::Percentage,
  property::{Properties, Property},
  Value, BASE_FONT_SIZE,
};
use strum::IntoEnumIterator;

fn to_computed_values(node: &NodePtr, styles: &mut Properties) {
  let parent_font_size = node
    .parent()
    .map(|parent| {
      NodePtr(parent).get_style(&Property::FontSize).to_absolute_px()
    })
    .unwrap_or(BASE_FONT_SIZE);

  let root_font_size = node
    .owner_document()
    .map(|root| NodePtr(root).get_style(&Property::FontSize).to_absolute_px())
    .unwrap_or(BASE_FONT_SIZE);

  let mut updates = Vec::new();

  for (property, value) in styles.iter() {
    match value {
      Value::Length(length) => match length {
        Length {
          value,
          unit: LengthUnit::Em,
        } => {
          let abs_length = Length::new_px(value * parent_font_size);
          updates.push((property.clone(), Value::Length(abs_length)));
        }
        Length {
          value,
          unit: LengthUnit::Rem,
        } => {
          let abs_length = Length::new_px(value * root_font_size);
          updates.push((property.clone(), Value::Length(abs_length)));
        }
        _ => {}
      },
      Value::Percentage(percentage) => match percentage {
        Percentage(value) if matches!(property, Property::FontSize) => {
          let abs_length = Length::new_px(value * parent_font_size / 100.);
          updates.push((property.clone(), Value::Length(abs_length)));
        }
        _ => {}
      },
      _ => {}
    }
  }

  for (property, value) in updates {
    styles.insert(property, value);
  }
}

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
