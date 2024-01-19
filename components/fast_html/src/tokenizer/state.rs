#[derive(Debug, Clone)]
pub enum State {
  Data,

  TagName,
  TagOpen,
  EndTagOpen,

  BeforeAttributeName,
  AttributeName,
  AfterAttributeName,

  BeforeAttributeValue,
  AttributeValueDoubleQuoted,
  AttributeValueSingleQuoted,
  AttributeValueUnQuoted,
  AfterAttributeValueQuoted,
}
