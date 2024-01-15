use css;
use html;

use std::env;

fn run_html() {
  let target = r#"<ul>
  <li>item1</li>
  <li>item2</li>
  <li>item3</li>
</ul>
  "#;

  let document = html::debugger::get_document_from_html(target);

  html::debugger::print_dom_tree(&document);

  println!("-------------------");

  let json = html::debugger::dom_in_body_to_json(&document);

  println!("{}", json);
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
