use fast_dom::node::NodePtr;
use re_css::value::{property::Property, Value};
use tree::TreeNode;

use crate::layout_box::{LayoutBox, LayoutBoxPtr};

struct LayoutTreeBuilder {
  parent_stack: Vec<LayoutBoxPtr>,
}

impl LayoutTreeBuilder {
  pub fn new() -> Self {
    Self {
      parent_stack: Vec::new(),
    }
  }

  fn build_layout_tree(&mut self, node: &NodePtr) {
    let display = node.get_style(&Property::Display);

    if let Value::Display(display) = display {
      if display.is_none() {
        return;
      }
    }

    let layout_box = TreeNode::new(LayoutBox::new(node));
  }
}
