use html::tokenizer;
use html::tree_builder;

fn main() {
  let target = r#"<!DOCTYPE html>
  <html>
  <body>
  
  <h2>HTML Buttons</h2>
  <p>HTML buttons are defined with the button tag:</p>
  
  <button>Click me</button>
  
  </body>
  </html>
  "#
  .chars();

  let tokenizer = tokenizer::Tokenizer::new(target);
  let tree_builder = tree_builder::TreeBuilder::default(tokenizer);

  let document = tree_builder.run();

  document.print_tree(0);
}
