use fast_dom::node::NodePtr;
use re_css::value::{property::Property, Value};
use tree::TreeNode;

use crate::layout_box::{BoxType, LayoutBox, LayoutBoxPtr};

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

    let parent = if layout_box.is_inline() {
      self.get_parent_for_inline()
    } else {
      self.get_parent_for_block()
    };
  }

  fn get_parent_for_inline(&self) -> Option<LayoutBoxPtr> {
    todo!("get_parent_for_inline");
  }

  fn get_parent_for_block(&self) -> Option<LayoutBoxPtr> {
    let parent = self.parent_stack.iter().rfind(|parent_box| {
      parent_box.is_block() && parent_box.can_have_children()
    });

    if let Some(parent) = parent {
      if !parent.has_no_child() && parent.children_are_inline() {
        let anonymous =
          TreeNode::new(LayoutBox::new_anonymous(BoxType::BlockBox));

        parent.transfer_children_to(anonymous.clone());
        parent.append_child(anonymous);
      }

      return Some(parent.clone());
    }

    None
  }
}
