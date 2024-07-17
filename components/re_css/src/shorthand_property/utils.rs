use crate::{
  parser::structure::ComponentValue,
  value::{property::Property, Value},
};

use super::ExpandedProperty;

pub fn expand_four_box_values(
  values: Vec<ComponentValue>,
  trbl: (Property, Property, Property, Property),
) -> Option<ExpandedProperty> {
  if values.len() == 1 {
    let value = Value::parse(&trbl.0, &values[0]);

    if value.is_none() {
      return None;
    }

    return Some(vec![
      (trbl.0, value.clone()),
      (trbl.1, value.clone()),
      (trbl.2, value.clone()),
      (trbl.3, value),
    ]);
  }

  if values.len() == 2 {
    let value_y = Value::parse(&trbl.0, &values[0]);
    let value_x = Value::parse(&trbl.1, &values[1]);

    if value_y.is_none() || value_x.is_none() {
      return None;
    }

    return Some(vec![
      (trbl.0, value_y.clone()),
      (trbl.1, value_x.clone()),
      (trbl.2, value_y),
      (trbl.3, value_x),
    ]);
  }

  if values.len() == 3 {
    let value_top = Value::parse(&trbl.0, &values[0]);
    let value_x = Value::parse(&trbl.1, &values[1]);
    let value_bottom = Value::parse(&trbl.2, &values[2]);

    if value_top.is_none() || value_x.is_none() || value_bottom.is_none() {
      return None;
    }

    return Some(vec![
      (trbl.0, value_top),
      (trbl.1, value_x.clone()),
      (trbl.2, value_bottom),
      (trbl.3, value_x),
    ]);
  }

  if values.len() == 4 {
    let value_top = Value::parse(&trbl.0, &values[0]);
    let value_right = Value::parse(&trbl.1, &values[1]);
    let value_bottom = Value::parse(&trbl.2, &values[2]);
    let value_left = Value::parse(&trbl.3, &values[3]);

    if value_top.is_none()
      || value_right.is_none()
      || value_bottom.is_none()
      || value_left.is_none()
    {
      return None;
    }

    return Some(vec![
      (trbl.0, value_top),
      (trbl.1, value_right),
      (trbl.2, value_bottom),
      (trbl.3, value_left),
    ]);
  }

  None
}
