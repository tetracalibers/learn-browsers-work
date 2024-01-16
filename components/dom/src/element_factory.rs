use ecow::EcoString;

use tree::{TreeNode, WeakTreeNode};

use super::elements::html_element::HTMLElement;
use crate::elements::ElementData;
use crate::node::NodeData;

use super::element::Element;

use super::node::{Node, NodePtr};

pub fn create_element(document: WeakTreeNode<Node>, tag_name: &str) -> NodePtr {
  let element_data =
    ElementData::Unknown(HTMLElement::new(EcoString::from(tag_name)));
  let node = Node::new(NodeData::Element(Element::new(element_data)));

  node.set_document(document);
  NodePtr(TreeNode::new(node))
}
