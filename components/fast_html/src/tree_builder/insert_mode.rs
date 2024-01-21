#[derive(Debug, Clone)]
pub enum InsertMode {
  Initial,

  BeforeHtml,

  BeforeHead,
  InHead,
  AfterHead,

  InBody,
  AfterBody,
  AfterAfterBody,
}
