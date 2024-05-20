use css::parser::selector::{
  AttributeOperator, CompoundSelector, SimpleSelector,
};
use ecow::EcoString;
use fast_dom::{element::Element, node::NodePtr};

fn is_match_compound_selector(
  element: &NodePtr,
  selector: &CompoundSelector,
) -> bool {
  if let Some(element) = element.as_maybe_element() {
    return selector.values().iter().all(|simple_selector| {
      is_match_simple_selector(element, &simple_selector)
    });
  }
  false
}

fn is_match_simple_selector(
  element: &Element,
  selector: &SimpleSelector,
) -> bool {
  match selector {
    SimpleSelector::Universal => true,
    SimpleSelector::Type(t) => element.tag_name().eq_ignore_ascii_case(t),
    SimpleSelector::Class(c) => {
      element.class_list().borrow().contains(&EcoString::from(c))
    }
    SimpleSelector::Id(id) => element.id().map_or(false, |i| i == *id),
    SimpleSelector::Attribute(attr_selector) => {
      if let Some(value) = &attr_selector.value {
        if let Some(op) = &attr_selector.operator {
          match op {
            AttributeOperator::Equal => element
              .attributes()
              .get(attr_selector.name.as_str())
              .map_or(false, |a| a.eq(value)),
            AttributeOperator::Contains => element
              .attributes()
              .get(attr_selector.name.as_str())
              .map_or(false, |a| a.contains(value.as_str())),
            AttributeOperator::StartsWith => element
              .attributes()
              .get(attr_selector.name.as_str())
              .map_or(false, |a| a.starts_with(value.as_str())),
            AttributeOperator::EndsWith => element
              .attributes()
              .get(attr_selector.name.as_str())
              .map_or(false, |a| a.ends_with(value.as_str())),
          }
        } else {
          false
        }
      } else {
        element.attributes().get(attr_selector.name.as_str()).is_some()
      }
    }
    _ => false,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use css::parser::selector::AttributeOperator;
  use css::parser::selector::AttributeSelector;
  use css::parser::selector::SimpleSelector;
  use ecow::eco_vec;
  use ecow::EcoString;
  use fast_dom::element::Element;
  use rustc_hash::FxHashMap;
  use std::cell::RefCell;

  #[test]
  fn test_is_match_simple_selector() {
    let mut attributes = FxHashMap::default();
    attributes.insert(EcoString::from("class"), EcoString::from("foo"));
    attributes.insert(EcoString::from("id"), EcoString::from("baz"));
    attributes.insert(EcoString::from("data-foo"), EcoString::from("baz"));

    let element = Element {
      tag_name: EcoString::from("div"),
      id: RefCell::new(Some(EcoString::from("bar"))),
      attributes: RefCell::new(attributes),
      class_list: RefCell::new(eco_vec![EcoString::from("foo")]),
    };

    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Universal
    ));
    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Type("div".to_string())
    ));
    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Class("foo".to_string())
    ));
    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Id("bar".to_string())
    ));
    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Attribute(AttributeSelector {
        name: "data-foo".to_string(),
        operator: None,
        value: None,
      })
    ));
    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Attribute(AttributeSelector {
        name: "data-foo".to_string(),
        operator: Some(AttributeOperator::Equal),
        value: Some("baz".to_string()),
      })
    ));
    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Attribute(AttributeSelector {
        name: "data-foo".to_string(),
        operator: Some(AttributeOperator::Contains),
        value: Some("a".to_string()),
      })
    ));
    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Attribute(AttributeSelector {
        name: "data-foo".to_string(),
        operator: Some(AttributeOperator::StartsWith),
        value: Some("b".to_string()),
      })
    ));
    assert!(is_match_simple_selector(
      &element,
      &SimpleSelector::Attribute(AttributeSelector {
        name: "data-foo".to_string(),
        operator: Some(AttributeOperator::EndsWith),
        value: Some("z".to_string()),
      })
    ));
  }
}
