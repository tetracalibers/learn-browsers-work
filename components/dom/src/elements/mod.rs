pub mod html_element;

use html_element::HTMLElement;

use super::node::NodeHooks;

trait ElementHooks {
  #[allow(unused_variables)]
  fn on_attribute_change(&self, name: &str, value: &str) {}
}

#[derive(Debug)]
pub enum ElementData {
  Unknown(HTMLElement),
}

impl NodeHooks for ElementData {}

impl ElementHooks for ElementData {}

impl ElementData {
  pub fn handle_attribute_change(&self, name: &str, value: &str) {
    self.on_attribute_change(name, value);
  }

  pub fn tag_name(&self) -> String {
    match self {
      ElementData::Unknown(element) => element.tag_name(),
    }
  }
}
