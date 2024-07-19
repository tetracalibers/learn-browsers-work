// ref: https://drafts.csswg.org/css-box/#box-model

use crate::primitive::EdgeSizes;

#[derive(Debug)]
pub struct BoxModel {
  padding: EdgeSizes,
  border: EdgeSizes,
  margin: EdgeSizes,
}
