use html::tokenizer;
use html::tree_builder;

fn main() {
  let target = r#"<!DOCTYPE html>
  <html>
  <body>
  
  <h1>My First Heading</h1>
  
  <p>My first paragraph.</p>
  
  </body>
  </html>
  "#
  .chars();

  let tokenizer = tokenizer::Tokenizer::new(target);
  let tree_builder = tree_builder::TreeBuilder::default(tokenizer);

  tree_builder.run();
}
