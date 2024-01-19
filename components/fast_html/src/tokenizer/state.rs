#[derive(Debug, Clone)]
pub enum State {
  Data,

  TagName,
  TagOpen,
  EndTagOpen,
}
