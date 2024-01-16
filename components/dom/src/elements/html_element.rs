use ecow::EcoString;

#[derive(Debug)]
pub struct HTMLElement {
  tag_name: EcoString,
}

impl HTMLElement {
  pub fn new(tag_name: EcoString) -> HTMLElement {
    HTMLElement { tag_name }
  }

  pub fn tag_name(&self) -> EcoString {
    self.tag_name.clone()
  }
}
