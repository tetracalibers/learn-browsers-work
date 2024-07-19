// ref: https://drafts.csswg.org/css-box/#box-model

use crate::primitive::EdgeSizes;

#[derive(Debug)]
pub struct BoxModel {
  padding: EdgeSizes,
  border: EdgeSizes,
  margin: EdgeSizes,
}

impl Default for BoxModel {
  fn default() -> Self {
    Self {
      padding: EdgeSizes::default(),
      border: EdgeSizes::default(),
      margin: EdgeSizes::default(),
    }
  }
}
