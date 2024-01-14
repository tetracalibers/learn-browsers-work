use std::borrow::Borrow;

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

pub fn print_dom_tree(document: &NodePtr) {
  document.print_tree(0);
}

// build Vec<(usize, TreeNode<Node>)>
pub fn get_dom_list_with_depth(
  document: &NodePtr,
) -> Vec<(usize, TreeNode<Node>)> {
  let root = document.0.borrow();

  // build Vec<(usize, TreeNode<Node>)>
  let mut node_list_with_depth = Vec::new();
  let depth = 0;

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

  traverse(root, depth, &mut node_list_with_depth);

  node_list_with_depth
}
