use css;
use html;

use std::env;

fn run_html() {
  let target = r#"<!DOCTYPE html>
  <html>
  <body>
  
  <h2>An Unordered HTML List</h2>
  <p href="main">This is a paragraph.</p>
  
  </body>
  </html>
  "#;

  let document = html::debugger::get_document_from_html(target);

  html::debugger::print_dom_tree(&document);

  println!("-------------------");

  let structure = html::debugger::get_dom_list_with_depth(&document);

  println!("{:?}", structure);
}

fn run_css() {
  css::parser::selector::main();
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Please specify the target.");
    return;
  }

  let target = &args[1];

  match target.as_str() {
    "html" => run_html(),
    "css" => run_css(),
    _ => println!("Please specify the target."),
  }
}
