#[derive(Debug, Clone)]
pub enum State {
  Data,

  RCDATA,
  RCDATALessThanSign,
  RCDATAEndTagOpen,
  RCDATAEndTagName,

  RAWTEXT,
  RAWTEXTLessThanSign,
  RAWTEXTEndTagOpen,
  RAWTEXTEndTagName,

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
