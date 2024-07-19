use crate::box_model::BoxModel;

#[derive(Debug)]
pub struct LayoutBox {
  box_type: BoxType,
  box_model: BoxModel,
}

#[derive(Debug)]
enum BoxType {
  BlockBox,
  InlineBox,
  TextSequence,
}
