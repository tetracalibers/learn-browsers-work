pub mod document;
pub mod element;
pub mod node;
pub mod text;
pub mod tree;

use document::Document;
use element::Element;

use node::DOMNode;
use node::DOMNodeData;
use node::NodePtr;

use tree::TreeNode;
use tree::WeakTreeNode;

pub fn create_element(
  document: WeakTreeNode<DOMNode>,
  tag_name: &str,
) -> NodePtr {
  let node = DOMNode::new(DOMNodeData::Element(Element::new(tag_name)));
  node.set_document(document);
  NodePtr(TreeNode::new(node))
}

pub fn create_document() -> NodePtr {
  NodePtr(TreeNode::new(DOMNode::new(DOMNodeData::Document(
    Document::new(),
  ))))
}
