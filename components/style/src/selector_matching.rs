use css::structs::selector::{
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

fn is_match_selectors(element: &NodePtr, selectors: &SelectorList) -> bool {
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

  true
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

  use css::cssom::stylesheet::CSSRule;
  use css::parser::parse_css;

  use fast_dom::create_document;
  use fast_dom::create_element;
  use fast_dom::tree::WeakTreeNode;

  fn assert_style_rule_matched_element(rule: &CSSRule, element: &NodePtr) {
    match rule {
      CSSRule::Style(style) => {
        let selectors = &style.selector;
        assert!(is_match_selectors(&element, selectors));
      }
    }
  }

  fn assert_style_rule_not_matched_element(rule: &CSSRule, element: &NodePtr) {
    match rule {
      CSSRule::Style(style) => {
        let selectors = &style.selector;
        assert!(!is_match_selectors(&element, selectors));
      }
    }
  }

  #[test]
  fn match_simple_type_ignore_case() {
    let element =
      create_element(WeakTreeNode::from(&create_document().0), "h1");
    let css = "h1 { color: red; } H1 { text-align: center; }";

    let stylesheet = parse_css(css).unwrap();

    let mut rules = stylesheet.rules.iter();

    let first_rule = rules.next().unwrap();
    let second_rule = rules.next().unwrap();

    assert_style_rule_matched_element(&first_rule, &element);
    assert_style_rule_matched_element(&second_rule, &element);
  }

  #[test]
  fn match_simple_class() {
    let element =
      create_element(WeakTreeNode::from(&create_document().0), "h1");
    element.as_element().set_attribute("class", "foo");
    let css = "h1.foo { color: red; }";

    let stylesheet = parse_css(css).unwrap();
    let rules = stylesheet.rules.first().unwrap();

    assert_style_rule_matched_element(&rules, &element);
  }

  #[test]
  fn match_simple_id() {
    let element =
      create_element(WeakTreeNode::from(&create_document().0), "h1");
    element.as_element().set_attribute("id", "hoge");
    let css = "h1#hoge { color: red; }";

    let stylesheet = parse_css(css).unwrap();
    let rules = stylesheet.rules.first().unwrap();

    assert_style_rule_matched_element(&rules, &element);
  }

  #[test]
  fn match_attribute_has() {
    let element =
      create_element(WeakTreeNode::from(&create_document().0), "h1");
    element.as_element().set_attribute("data-foo", "");
    let css = "h1[data-foo] { color: red; }";

    let stylesheet = parse_css(css).unwrap();
    let rules = stylesheet.rules.first().unwrap();

    assert_style_rule_matched_element(&rules, &element);
  }

  #[test]
  fn match_attribute_equal() {
    let element =
      create_element(WeakTreeNode::from(&create_document().0), "h1");
    element.as_element().set_attribute("data-foo", "bar");
    let css = "h1[data-foo=\"bar\"] { color: red; }";

    let stylesheet = parse_css(css).unwrap();
    let rules = stylesheet.rules.first().unwrap();

    assert_style_rule_matched_element(&rules, &element);
  }

  // todo: more attribute tests

  #[test]
  fn match_simple_decendant() {
    let doc = create_document();
    let parent = create_element(WeakTreeNode::from(&doc.0), "section");
    let child = create_element(WeakTreeNode::from(&doc.0), "h1");
    parent.append_child(child.0.clone());

    let css = "section h1 { font-weight: bold; }";

    let stylesheet = parse_css(css).unwrap();
    let rules = stylesheet.rules.first().unwrap();

    assert_style_rule_matched_element(&rules, &child);
  }

  #[test]
  fn match_simple_child() {
    let doc = create_document();
    let parent = create_element(WeakTreeNode::from(&doc.0), "section");
    let child = create_element(WeakTreeNode::from(&doc.0), "h1");
    parent.append_child(child.0.clone());

    let css = "section > h1 { font-weight: bold; }";

    let stylesheet = parse_css(css).unwrap();
    let rules = stylesheet.rules.first().unwrap();

    assert_style_rule_matched_element(&rules, &child);
  }

  // todo: more combinator tests

  #[test]
  fn match_invalid_child() {
    let doc = create_document();
    let parent = create_element(WeakTreeNode::from(&doc.0), "section");
    let child = create_element(WeakTreeNode::from(&doc.0), "h1");
    parent.append_child(child.0.clone());

    let css = "h1 > section { font-weight: bold; }";

    let stylesheet = parse_css(css).unwrap();
    let rules = stylesheet.rules.first().unwrap();

    assert_style_rule_not_matched_element(&rules, &child);
  }
}
