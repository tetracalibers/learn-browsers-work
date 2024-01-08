#[derive(Debug)]
pub enum InsertMode {
  Initial,
  BeforeHtml,
  BeforeHead,
  InHead,
  InHeadNoScript,
  AfterHead,
}
