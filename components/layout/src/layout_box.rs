use std::ops::Deref;

use fast_dom::node::NodePtr;
use re_css::value::{
  display::{Display, DisplayInside, DisplayOutside},
  property::Property,
  Value,
};
use tree::{TreeNode, TreeNodeHooks};

use crate::box_model::BoxModel;

#[derive(Debug)]
pub struct LayoutBox {
  box_type: BoxType,
  box_model: BoxModel,
  node: Option<NodePtr>,
}

pub struct LayoutBoxPtr(pub TreeNode<LayoutBox>);

impl TreeNodeHooks<LayoutBox> for LayoutBox {}

#[derive(Debug)]
pub enum BoxType {
  BlockBox,
  InlineBox,
  TextSequence,
}

impl LayoutBox {
  pub fn new(node: NodePtr) -> Self {
    Self {
      box_type: Self::get_box_type(&node),
      box_model: Default::default(),
      node: Some(node),
    }
  }

  pub fn new_anonymous(box_type: BoxType) -> Self {
    LayoutBox {
      box_type,
      box_model: Default::default(),
      node: None,
    }
  }

  fn get_box_type(node: &NodePtr) -> BoxType {
    if node.is_text() {
      return BoxType::TextSequence;
    }

    match node.get_style(&Property::Display) {
      Value::Display(display) => match display {
        Display::Full(ref outer, ref inner) => match (outer, inner) {
          (DisplayOutside::Block, DisplayInside::Flow) => BoxType::BlockBox,
          (DisplayOutside::Inline, DisplayInside::Flow) => BoxType::InlineBox,
          (DisplayOutside::Inline, DisplayInside::FlowRoot) => {
            BoxType::InlineBox
          }
          _ => unimplemented!("Unsupport display type: {:#?}", display),
        },
        _ => unimplemented!("Unsupport display type: {:#?}", display),
      },
      _ => unreachable!(),
    }
  }

  pub fn is_block(&self) -> bool {
    match self.box_type {
      BoxType::BlockBox => true,
      _ => false,
    }
  }

  pub fn is_inline(&self) -> bool {
    match self.box_type {
      BoxType::InlineBox => true,
      BoxType::TextSequence => true,
      _ => false,
    }
  }

  pub fn is_anonymous(&self) -> bool {
    self.node.is_none()
  }

  pub fn can_have_children(&self) -> bool {
    match self.box_type {
      BoxType::TextSequence => false,
      _ => true,
    }
  }

  pub fn node(&self) -> Option<NodePtr> {
    self.node.clone()
  }

  pub fn friendly_name(&self) -> &str {
    match self.box_type {
      BoxType::BlockBox => "BlockBox",
      BoxType::InlineBox => "InlineBox",
      BoxType::TextSequence => "TextSequence",
    }
  }
}

impl LayoutBoxPtr {
  pub fn children_are_inline(&self) -> bool {
    self.iterate_children().all(|child| child.is_inline())
  }
}

impl Deref for LayoutBoxPtr {
  type Target = TreeNode<LayoutBox>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Clone for LayoutBoxPtr {
  fn clone(&self) -> Self {
    LayoutBoxPtr(self.0.clone())
  }
}
