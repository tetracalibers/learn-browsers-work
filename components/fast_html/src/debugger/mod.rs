use super::tokenizer::Tokenizer;
use super::tree_builder::TreeBuilder;

use std::collections::HashMap;

use fast_dom::document::Document;
use fast_dom::element::Element;
use fast_dom::node::DOMNode;
use fast_dom::node::DOMNodeData;
use fast_dom::node::NodePtr;
use fast_dom::text::Text;
use fast_dom::tree::TreeNode;

use serde_json;
use serde_json::json;

/* -------------------------------------------- */

pub fn get_document_from_html(html: &str) -> NodePtr {
  let target = html.as_bytes();

  let empty_document = NodePtr(TreeNode::new(DOMNode::new(
    DOMNodeData::Document(Document::new()),
  )));

  let tokenizer = Tokenizer::new(target);
  let tree_builder = TreeBuilder::new(tokenizer, empty_document);

  tree_builder.run()
}

/* print dom tree ----------------------------- */

fn print_dom_tree_core(root: &TreeNode<DOMNode>, depth: usize) {
  let indent = "    ".repeat(depth) + "|-";

  let mut print_this = true;

  if let Some(text_node) = root.as_maybe_text() {
    if text_node.value.borrow().trim().is_empty() {
      print_this = false;
    }
  }

  if print_this {
    println!("{}{:?}", indent, root);
  }

  for child in root.iterate_children() {
    print_dom_tree_core(&child, depth + 1);
  }
}

pub fn print_dom_tree(document: &NodePtr) {
  print_dom_tree_core(document, 0);
}

/* dom to json -------------------------------- */

fn text_node_to_json(node: &Text) -> serde_json::Value {
  json!({
    "type": "text",
    "value": node.value.borrow().as_str(),
  })
}

fn element_node_to_json(node: &Element) -> serde_json::Value {
  let mut attributes = node
    .attributes()
    .borrow()
    .iter()
    .map(|(key, value)| (String::from(key), String::from(value)))
    .collect::<HashMap<String, String>>();

  let class_attribute = node.class_list().borrow().join(" ");
  if !class_attribute.is_empty() {
    attributes.insert("class".to_string(), class_attribute);
  }

  if let Some(id) = node.id().borrow().as_ref() {
    attributes.insert("id".to_string(), id.to_string());
  }

  if attributes.is_empty() {
    json!({
      "type": "element",
      "tag": node.tag_name().as_str(),
    })
  } else {
    json!({
      "type": "element",
      "tag": node.tag_name().as_str(),
      "attributes": attributes,
    })
  }
}

fn document_node_to_json() -> serde_json::Value {
  json!({
    "type": "document",
  })
}

fn dom_to_json_core(
  root: &TreeNode<DOMNode>,
  depth: usize,
) -> serde_json::Value {
  let mut children = Vec::new();

  for child in root.iterate_children() {
    let inner_json = dom_to_json_core(&child, depth + 1);
    if inner_json != serde_json::Value::Null {
      children.push(inner_json);
    }
  }

  let mut json = json!(serde_json::Value::Null);

  if let Some(text_node) = root.as_maybe_text() {
    if !text_node.value.borrow().trim().is_empty() {
      json = text_node_to_json(text_node);
    }
  }

  if let Some(element_node) = root.as_maybe_element() {
    json = element_node_to_json(element_node);
  }

  if root.as_maybe_document().is_some() {
    json = document_node_to_json();
  }

  if !children.is_empty() {
    json["children"] = json!(children);
  }

  json
}

pub fn dom_to_json(document: &NodePtr) -> serde_json::Value {
  dom_to_json_core(document, 0)
}

pub fn dom_to_json_string(document: &NodePtr) -> String {
  serde_json::to_string_pretty(&dom_to_json(document)).unwrap()
}

pub fn dom_body_to_json(document: &NodePtr) -> serde_json::Value {
  let root = document.0.clone();

  let html = root.first_child().unwrap();
  let head = html.first_child().unwrap();
  let body = head.next_sibling().unwrap();

  dom_to_json_core(&body, 0)
}

pub fn dom_body_to_json_string(document: &NodePtr) -> String {
  serde_json::to_string_pretty(&dom_body_to_json(document)).unwrap()
}
