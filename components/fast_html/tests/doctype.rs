extern crate fast_html;

use fast_html::tokenizer::token::Token;
use fast_html::tokenizer::Tokenizer;

use ecow::EcoString;

#[test]
fn html5_doctype() {
  let html = r#"<!DOCTYPE html>"#;

  let mut tokenizer = Tokenizer::new(html.as_bytes());
  let actual = tokenizer.next_token();

  let excepted = Token::DOCTYPE {
    name: Some(EcoString::from("html")),
    force_quirks: false,
  };

  assert_eq!(actual, excepted);
}
