use serde_json;
use serde_json::json;

use std::borrow::Borrow;
use std::collections::HashMap;

use super::tokenizer::Tokenizer;
use super::tree_builder::TreeBuilder;

use dom::document::Document;
use dom::node::Node;
use dom::node::NodeData;
use dom::node::NodePtr;

use tree::TreeNode;

pub fn get_document_from_html(html: &str) -> NodePtr {
  let target = html.chars();

  let empty_document = NodePtr(TreeNode::new(Node::new(NodeData::Document(
    Document::new(),
  ))));

  let tokenizer = Tokenizer::new(target);
  let tree_builder = TreeBuilder::new(tokenizer, empty_document);

  let document = tree_builder.run();

  document
}

fn print_dom_tree_core(root: &TreeNode<Node>, depth: usize) {
  let indent = "    ".repeat(depth) + "|-";

  let mut print_this = true;

  if let Some(text_node) = root.as_maybe_text() {
    if text_node.characters.get_data().trim().is_empty() {
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

// 兄弟要素も含めて深さ優先で走査
fn traverse(
  node: &TreeNode<Node>,
  depth: usize,
  node_list_with_depth: &mut Vec<(usize, TreeNode<Node>)>,
) {
  // 空文字しかないテキストノード以外をpush
  let mut append_node = true;

  if let Some(text_node) = node.as_maybe_text() {
    if text_node.characters.get_data().trim().is_empty() {
      append_node = false;
    }
  }

  if append_node {
    node_list_with_depth.push((depth, node.clone()));
  }

  if let Some(first_child) = node.first_child() {
    traverse(&first_child, depth + 1, node_list_with_depth);
  }
  if let Some(next_sibling) = node.next_sibling() {
    traverse(&next_sibling, depth, node_list_with_depth);
  }
}

// build Vec<(usize, TreeNode<Node>)>
pub fn get_dom_list_with_depth(
  document: &NodePtr,
) -> Vec<(usize, TreeNode<Node>)> {
  let root = document.0.borrow();

  // build Vec<(usize, TreeNode<Node>)>
  let mut node_list_with_depth = Vec::new();
  let depth = 0;

  traverse(root, depth, &mut node_list_with_depth);

  node_list_with_depth
}

pub fn get_dom_list_with_depth_in_body(
  document: &NodePtr,
) -> Vec<(usize, TreeNode<Node>)> {
  let root = document.0.borrow();

  let html = root.first_child().unwrap();
  let head = html.first_child().unwrap();
  let body = head.next_sibling().unwrap();

  let mut node_list_with_depth = Vec::new();
  let depth = 0;

  traverse(&body, depth, &mut node_list_with_depth);

  node_list_with_depth
}

pub fn dom_list_to_recursive_json(
  node_list_with_depth: &Vec<(usize, TreeNode<Node>)>,
) -> serde_json::Value {
  let mut json = json!({});

  for (depth, node) in node_list_with_depth {
    let mut current_json = &mut json;

    for _ in 1..*depth {
      current_json = current_json
        .get_mut("children")
        .unwrap()
        .as_array_mut()
        .unwrap()
        .last_mut()
        .unwrap();
    }

    let mut simple_node;

    if let Some(text_node) = node.as_maybe_text() {
      simple_node = json!({
        "type": "text",
        "data": text_node.characters.get_data(),
      });
    } else if let Some(element_node) = node.as_maybe_element() {
      simple_node = json!({
        "type": "element",
        "tag": element_node.tag_name(),
      });

      let mut attributes = element_node
        .attributes()
        .borrow()
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect::<HashMap<String, String>>();

      let classes = element_node.class_list().borrow().join(" ");

      if !classes.is_empty() {
        attributes.insert("class".to_string(), classes);
      }

      if let Some(id) = element_node.id().borrow().as_ref() {
        attributes.insert("id".to_string(), id.clone());
      }

      if !attributes.is_empty() {
        simple_node["attributes"] = json!(attributes);
      }
    } else if let Some(_) = node.as_maybe_document() {
      simple_node = json!({
        "type": "document",
      });
    } else {
      panic!("Unknown node type.");
    }

    if depth == &0 {
      json = simple_node;
      continue;
    }

    let children = current_json.get_mut("children");

    if children.is_none() {
      current_json["children"] = json!([]);
    }

    current_json["children"].as_array_mut().unwrap().push(simple_node);
  }

  json
}

pub fn dom_to_json(document: &NodePtr) -> String {
  let node_list_with_depth = get_dom_list_with_depth(document);

  let json_obj = dom_list_to_recursive_json(&node_list_with_depth);
  let json = serde_json::to_string_pretty(&json_obj).unwrap();

  json
}

pub fn dom_in_body_to_json(document: &NodePtr) -> String {
  let node_list_with_depth = get_dom_list_with_depth_in_body(document);

  let json_obj = dom_list_to_recursive_json(&node_list_with_depth);
  let json = serde_json::to_string_pretty(&json_obj).unwrap();

  json
}
