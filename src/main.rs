use std::env;

const TARGET_HTML: &str = r#"<!DOCTYPE html>
<html>
<head>
  <title>document title</title>
</head>
<body>
<iframe
  src="https://example.org"
  title="iframe Example 1"
  width="400"
  height="300">
</iframe>

</body>
</html>"#;

fn run_html(html: &str) {
  let document = html::debugger::get_document_from_html(html);

  html::debugger::print_dom_tree(&document);

  println!("-------------------");

  let json = html::debugger::dom_to_json(&document);

  println!("{}", json);
}

fn run_fast_html(html: &str) {
  let document = fast_html::debugger::get_document_from_html(html);

  fast_html::debugger::print_dom_tree(&document);

  println!("-------------------");

  let json = fast_html::debugger::dom_to_json_string(&document);
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
    "html" => {
      let html = if args.len() > 2 {
        &args[2]
      } else {
        TARGET_HTML
      };

      run_html(html);
    }
    "fast_html" => {
      let html = if args.len() > 2 {
        &args[2]
      } else {
        TARGET_HTML
      };

      run_fast_html(html);
    }
    "css" => run_css(),
    _ => println!("Please specify the target."),
  }
}
