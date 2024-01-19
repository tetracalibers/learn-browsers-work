use std::env;

fn run_html() {
  let target = r#"<h1>This is heading</h1>
  <p>This is paragraph</p>
  <p>This <mark>keyword</mark> is important</p>"#;

  let document = html::debugger::get_document_from_html(target);

  html::debugger::print_dom_tree(&document);

  //println!("-------------------");

  //let json = html::debugger::dom_in_body_to_json(&document);

  //println!("{}", json);
}

fn run_fast_html() {
  let target = r#"<h1>This is heading</h1>
  <p>This is paragraph</p>
  <p>This <mark>keyword</mark> is important</p>"#;

  let document = fast_html::debugger::get_document_from_html(target);

  fast_html::debugger::print_dom_tree(&document);
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
