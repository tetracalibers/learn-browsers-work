#[derive(Debug, Clone)]
pub enum InsertMode {
  Initial,
  BeforeHtml,
  BeforeHead,
  InHead,
  InHeadNoScript,
  AfterHead,
  InBody,
  AfterBody,
  AfterAfterBody,

  Text,

  InTable,
  InTableText,
  InTableBody,
  InRow,
}
