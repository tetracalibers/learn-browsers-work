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

  pub fn build(mut self, root: NodePtr) -> Option<LayoutBoxPtr> {
    let root_node = if root.is_document() {
      root.first_child().map(|child| NodePtr(child))
    } else {
      Some(root)
    };

    if let Some(root_node) = root_node {
      let display = root_node.get_style(&Property::Display);

      if let Value::Display(display) = display {
        if display.is_none() {
          return None;
        }
      }

      let root_box =
        LayoutBoxPtr(TreeNode::new(LayoutBox::new(root_node.clone())));

      self.parent_stack.push(root_box.clone());
      root_node.for_each_child(|child| self.build_layout_tree(NodePtr(child)));
      self.parent_stack.pop();

      return Some(root_box);
    }

    None
  }

  fn build_layout_tree(&mut self, node: NodePtr) {
    let display = node.get_style(&Property::Display);

    if let Value::Display(display) = display {
      if display.is_none() {
        return;
      }
    }

    let layout_box = TreeNode::new(LayoutBox::new(node.clone()));

    let parent = if layout_box.is_inline() {
      self.get_parent_for_inline()
    } else {
      self.get_parent_for_block()
    };

    if let Some(parent) = parent {
      parent.append_child(layout_box.clone());

      if !node.is_element() && !node.is_document() {
        return;
      }

      self.parent_stack.push(LayoutBoxPtr(layout_box));
      node.for_each_child(|child| self.build_layout_tree(NodePtr(child)));
      self.parent_stack.pop();
    } else {
      let parent_stack = self
        .parent_stack
        .iter()
        .map(|parent| format!("{} {:?}", parent.friendly_name(), parent.node()))
        .collect::<Vec<String>>();

      log::error!(
        "Couldn't find the correct parent for node: {} {:?}",
        LayoutBoxPtr(layout_box).friendly_name(),
        node
      );
      log::error!("Parent stack: {:#?}", parent_stack);
    }
  }

  fn get_parent_for_inline(&self) -> Option<LayoutBoxPtr> {
    let parent = self.parent_stack.last();

    if let Some(parent) = parent {
      if parent.has_no_child() || parent.children_are_inline() {
        return Some(parent.clone());
      }

      let get_last_node = || parent.last_child().map(|node| LayoutBoxPtr(node));

      let require_anonymous = get_last_node()
        .map(|last_node| {
          !(last_node.is_anonymous() && last_node.children_are_inline())
        })
        .unwrap_or(true);

      if require_anonymous {
        let anonymous =
          TreeNode::new(LayoutBox::new_anonymous(BoxType::BlockBox));
        parent.append_child(anonymous);
      }

      return get_last_node();
    }

    None
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
