#[derive(Debug, Clone)]
pub enum State {
  Data,

  RCDATA,

  RAWTEXT,

  TagName,
  TagOpen,
  EndTagOpen,
  SelfClosingStartTag,

  BeforeAttributeName,
  AttributeName,
  AfterAttributeName,

  BeforeAttributeValue,
  AttributeValueDoubleQuoted,
  AttributeValueSingleQuoted,
  AttributeValueUnQuoted,
  AfterAttributeValueQuoted,

  MarkupDeclarationOpen,
  DOCTYPE,
  BeforeDOCTYPEName,
  DOCTYPEName,
  AfterDOCTYPEName,
  BogusDOCTYPE,
}
