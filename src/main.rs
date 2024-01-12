use html::tokenizer;
use html::tree_builder;

use css;

fn run_html() {
  let target = r#"<!DOCTYPE html>
  <html>
  <body>
  
  <h2>An Unordered HTML List</h2>
  
  <ul>
    <li>Coffee</li>
    <li>Tea</li>
    <li>Milk</li>
  </ul>
  
  <h2>An Ordered HTML List</h2>
  
  <ol>
    <li>Coffee</li>
    <li>Tea</li>
    <li>Milk</li>
  </ol>
  
  </body>
  </html>
  "#
  .chars();

  let tokenizer = tokenizer::Tokenizer::new(target);
  let tree_builder = tree_builder::TreeBuilder::default(tokenizer);

  let document = tree_builder.run();

  document.print_tree(0);
}

fn run_css() {
  css::parser::selector::main();
}

fn main() {
  run_css();
}
