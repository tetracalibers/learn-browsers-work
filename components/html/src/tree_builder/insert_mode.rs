#[derive(Debug, Clone)]
pub enum InsertMode {
  Initial,
  BeforeHtml,
  BeforeHead,
  InHead,
  InHeadNoScript,
  AfterHead,

  Text,
}
