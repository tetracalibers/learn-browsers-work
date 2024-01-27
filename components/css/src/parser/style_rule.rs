use super::declaration::Declaration;
use super::selector::SelectorList;

#[derive(Debug)]
pub struct StyleRule {
  selector: SelectorList,
  declarations: Vec<Declaration>,
}
