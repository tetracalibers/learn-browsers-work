use super::tokenizer::Tokenizer;
use super::tree_builder::TreeBuilder;

use fast_dom::document::Document;
use fast_dom::node::DOMNode;
use fast_dom::node::DOMNodeData;
use fast_dom::node::NodePtr;
use fast_dom::tree::TreeNode;

/* -------------------------------------------- */

pub fn get_document_from_html(html: &str) -> NodePtr {
  let target = html.as_bytes();

  let empty_document = NodePtr(TreeNode::new(DOMNode::new(
    DOMNodeData::Document(Document::new()),
  )));

  let tokenizer = Tokenizer::new(target);
  let mut tree_builder = TreeBuilder::new(tokenizer, empty_document);

  let document = tree_builder.run();

  document
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
