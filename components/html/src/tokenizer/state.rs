#[derive(Debug, Clone)]
pub enum State {
  Data,

  RCDATA,
  RCDATALessThanSign,
  RCDATAEndTagOpen,
  RCDATAEndTagName,

  RAWTEXT,

  CharacterReference,
  NamedCharacterReference,
  NumericCharacterReference,

  TagName,
  TagOpen,
  EndTagOpen,
  SelfClosingStartTag,

  MarkupDeclarationOpen,

  CommentStart,
  BogusComment,

  BeforeAttributeName,
  AttributeName,
  AfterAttributeName,

  BeforeAttributeValue,
  AttributeValueDoubleQuoted,
  AttributeValueSingleQuoted,
  AttributeValueUnQuoted,
  AfterAttributeValueQuoted,

  DOCTYPE,
  BeforeDOCTYPEName,
  DOCTYPEName,
  AfterDOCTYPEName,
  BogusDOCTYPE,
}
