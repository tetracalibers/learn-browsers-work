use css;
use fast_html;
use html;

use std::env;

fn run_html() {
  let target = r#"<h1>こんにちは</h1>"#;

  let document = html::debugger::get_document_from_html(target);

  html::debugger::print_dom_tree(&document);

  println!("-------------------");

  let json = html::debugger::dom_in_body_to_json(&document);

  println!("{}", json);
}

fn run_fast_html() {
  let target = r#"<h1>こんにちは</h1>"#;

  let mut tokenizer = fast_html::tokenizer::Tokenizer::new(target.as_bytes());

  loop {
    match tokenizer.next_token() {
      fast_html::tokenizer::token::Token::EOF => {
        println!("EOF");
        break;
      }
      token => {
        println!("{:?}", token);
      }
    }
  }
}

fn run_css() {
  css::parser::selector::main();
}

fn main() {
  env_logger::init();

  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Please specify the target.");
    return;
  }

  let target = &args[1];

  match target.as_str() {
    "html" => run_html(),
    "fast_html" => run_fast_html(),
    "css" => run_css(),
    _ => println!("Please specify the target."),
  }
}
