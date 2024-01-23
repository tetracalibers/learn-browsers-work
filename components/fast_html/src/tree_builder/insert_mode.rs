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

  Text,

  InTable,
  InTableText,
  InTableBody,
  InRow,
  InCell,
  InCaption,
  InColumnGroup,

  InSelect,
  InSelectInTable,
}
