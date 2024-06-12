use css::structs::declaration_value::DeclarationValue;

use crate::{property::Property, value::Value};

use super::ExpandedProperty;

pub fn expand_border_color(values: &[&[DeclarationValue]]) -> ExpandedProperty {
  match values.len() {
    1 => {
      let value = Value::parse(&Property::BorderTopColor, values[0]);

      if value.is_none() {
        return None;
      }

      Some(vec![
        (Property::BorderTopColor, value.clone()),
        (Property::BorderRightColor, value.clone()),
        (Property::BorderBottomColor, value.clone()),
        (Property::BorderLeftColor, value),
      ])
    }
    2 => {
      let border_y = Value::parse(&Property::BorderTopColor, values[0]);
      let border_x = Value::parse(&Property::BorderRightColor, values[1]);

      if border_y.is_none() || border_x.is_none() {
        return None;
      }

      Some(vec![
        (Property::BorderTopColor, border_y.clone()),
        (Property::BorderRightColor, border_x.clone()),
        (Property::BorderBottomColor, border_y),
        (Property::BorderLeftColor, border_x),
      ])
    }
    3 => {
      let border_top = Value::parse(&Property::BorderTopColor, values[0]);
      let border_x = Value::parse(&Property::BorderRightColor, values[1]);
      let border_bottom = Value::parse(&Property::BorderBottomColor, values[2]);

      if border_top.is_none() || border_x.is_none() || border_bottom.is_none() {
        return None;
      }

      Some(vec![
        (Property::BorderTopColor, border_top),
        (Property::BorderRightColor, border_x.clone()),
        (Property::BorderBottomColor, border_bottom),
        (Property::BorderLeftColor, border_x),
      ])
    }
    4 => {
      let border_top = Value::parse(&Property::BorderTopColor, values[0]);
      let border_right = Value::parse(&Property::BorderRightColor, values[1]);
      let border_bottom = Value::parse(&Property::BorderBottomColor, values[2]);
      let border_left = Value::parse(&Property::BorderLeftColor, values[3]);

      if border_top.is_none()
        || border_right.is_none()
        || border_bottom.is_none()
        || border_left.is_none()
      {
        return None;
      }

      Some(vec![
        (Property::BorderTopColor, border_top),
        (Property::BorderRightColor, border_right),
        (Property::BorderBottomColor, border_bottom),
        (Property::BorderLeftColor, border_left),
      ])
    }
    _ => None,
  }
}
