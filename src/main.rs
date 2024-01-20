use std::env;

const TARGET_HTML: &str =
  r#"<a href="https://example.com" target="_blank">sample link</a>"#;

fn run_html() {
  let document = html::debugger::get_document_from_html(TARGET_HTML);

  html::debugger::print_dom_tree(&document);

  println!("-------------------");

  let json = html::debugger::dom_in_body_to_json(&document);

  println!("{}", json);
}

fn run_fast_html() {
  let document = fast_html::debugger::get_document_from_html(TARGET_HTML);

  fast_html::debugger::print_dom_tree(&document);

  println!("-------------------");

  let json = fast_html::debugger::dom_body_to_json_string(&document);
  println!("{}", json);
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
