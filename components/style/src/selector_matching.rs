use css::parser::selector::{
  AttributeOperator, Combinator, ComplexSelectorSequence, CompoundSelector,
  SelectorList, SimpleSelector,
};
use ecow::EcoString;
use fast_dom::{element::Element, node::NodePtr};

fn get_parent(element: &NodePtr) -> Option<NodePtr> {
  let parent = element.parent();

  if let Some(parent) = parent {
    if parent.is_element() {
      return Some(NodePtr(parent));
    }
  }

  None
}

fn get_prev_sibling(element: &NodePtr) -> Option<NodePtr> {
  element.prev_sibling().map(|sibling| NodePtr(sibling))
}

/* -------------------------------------------- */

fn is_match_selectors(element: &NodePtr, selectors: SelectorList) -> bool {
  selectors.iter().any(|selector| is_match_selector(element.clone(), selector))
}

fn is_match_selector(
  element: NodePtr,
  selector: &ComplexSelectorSequence,
) -> bool {
  let mut current_element = Some(element);

  for (selector_seq, combinator) in selector.iter().rev() {
    if let Some(el) = current_element.clone() {
      match combinator {
        Some(Combinator::Child) => {
          let parent = get_parent(&el);
          if let Some(p) = &parent {
            if !is_match_compound_selector(p, selector_seq) {
              return false;
            }
          }
          current_element = parent;
        }
        Some(Combinator::Descendant) => loop {
          let parent = get_parent(&el);
          if let Some(p) = &parent {
            if is_match_compound_selector(p, selector_seq) {
              current_element = parent;
              break;
            }
          }
          return false;
        },
        Some(Combinator::NextSibling) => {
          let sibling = get_prev_sibling(&el);
          if let Some(s) = &sibling {
            if !is_match_compound_selector(s, selector_seq) {
              return false;
            }
          }
          current_element = sibling;
        }
        Some(Combinator::SubsequentSibling) => loop {
          let sibling = get_prev_sibling(&el);
          if let Some(s) = &sibling {
            if is_match_compound_selector(s, selector_seq) {
              current_element = sibling;
              break;
            }
          }
          return false;
        },
        None => {
          if !is_match_compound_selector(&el, selector_seq) {
            return false;
          }
        }
      }
    } else {
      return false;
    }
  }

  false
}

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
