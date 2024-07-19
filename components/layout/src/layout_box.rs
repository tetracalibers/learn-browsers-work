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
}

pub struct LayoutBoxPtr(TreeNode<LayoutBox>);

impl TreeNodeHooks<LayoutBox> for LayoutBox {}

#[derive(Debug)]
enum BoxType {
  BlockBox,
  InlineBox,
  TextSequence,
}

impl LayoutBox {
  pub fn new(node: &NodePtr) -> Self {
    Self {
      box_type: Self::judge_box_type(node),
      box_model: Default::default(),
    }
  }

  fn judge_box_type(node: &NodePtr) -> BoxType {
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

  pub fn is_inline(&self) -> bool {
    match self.box_type {
      BoxType::InlineBox => true,
      BoxType::TextSequence => true,
      _ => false,
    }
  }
}
