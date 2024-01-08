#[derive(Debug)]
pub struct HTMLElement {
  tag_name: String,
}

impl HTMLElement {
  pub fn new(tag_name: String) -> HTMLElement {
    HTMLElement { tag_name }
  }

  pub fn tag_name(&self) -> String {
    self.tag_name.clone()
  }
}
