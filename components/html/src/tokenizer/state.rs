#[derive(Debug, Clone)]
pub enum State {
  Data,

  CharacterReference,
  NamedCharacterReference,
  NumericCharacterReference,

  TagName,
  TagOpen,
  EndTagOpen,
  SelfClosingStartTag,

  MarkupDeclarationOpen,

  BogusComment,

  BeforeAttributeName,
  AttributeName,
  AfterAttributeName,

  BeforeAttributeValue,
  AttributeValueDoubleQuoted,
  AttributeValueSingleQuoted,
  AttributeValueUnQuoted,
  AfterAttributeValueQuoted,
}
