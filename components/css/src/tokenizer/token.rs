#[derive(Debug)]
pub enum Token {
  Delim(char),
  Hash { value: String, type_flag: HashType },
  String(String),
  BadString,
  Whitespace,
  Comma,
  Colon,
  SemiColon,
  ParentheseOpen,
  ParentheseClose,
  BracketOpen,
  BracketClose,
  BraceOpen,
  BraceClose,
  EOF,
}

#[derive(Debug)]
enum HashType {
  Id,
}

impl Token {
  pub fn append_to_string_token(&mut self, ch: char) {
    if let Token::String(data) = self {
      data.push(ch);
    }
  }
}
