use std::env;

const TARGET_HTML: &str = r#"<p>Put the &lt;h1> at the beginning of the heading and the &lt;h1> at the end.</p>"#;

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
  css::parser::stylesheet::main();
}

fn run_re_css() {
  let sample = r#"
    h1 {
      color: blue !important;
      text-align: center;
      box-shadow: 12px 12px 2px 1px rgba(0, 0, 255, .2);
    }
  "#;
  let syntax = re_css::parser::parse::rules(sample);
  println!("{:#?}", syntax);

  let sample2 = r#"
    div {
      margin-top: 10px;
    }
  "#;
  if let Ok((_, rules)) = re_css::parser::parse::rules(sample2) {
    let first_rule = rules.iter().next().unwrap();
    if let re_css::parser::structure::CSSRule::StyleRule(style) = first_rule {
      let declaration = style.declarations.iter().next().unwrap();
      let values = re_css::value::Value::parse(
        re_css::value::property::Property::MarginTop,
        &declaration.value,
      );
      println!("{:?}", values);
    }
  }
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
    "re_css" => run_re_css(),
    _ => println!("Please specify the target."),
  }
}
