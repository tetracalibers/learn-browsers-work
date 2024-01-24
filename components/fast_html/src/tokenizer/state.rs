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

  CommentStart,
  CommentStartDash,
  Comment,
  CommentLessThanSign,
  CommentLessThanSignBang,
  CommentLessThanSignBangDash,
  CommentLessThanSignBangDashDash,
  CommentEndDash,
  CommentEnd,
  CommentEndBang,
  BogusComment,

  DOCTYPE,
  BeforeDOCTYPEName,
  DOCTYPEName,
  AfterDOCTYPEName,
  BogusDOCTYPE,
}
