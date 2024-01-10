use html::tokenizer;
use html::tree_builder;

fn main() {
  let target = r#"<!DOCTYPE html>
  <html>
  <body>
  
  <h2>HTML Links</h2>
  <p>HTML links are defined with the a tag:</p>
  
  <a href="https://www.w3schools.com">This is a link</a>
  
  </body>
  </html>
  "#
  .chars();

  let tokenizer = tokenizer::Tokenizer::new(target);
  let tree_builder = tree_builder::TreeBuilder::default(tokenizer);

  let document = tree_builder.run();

  document.print_tree(0);
}
