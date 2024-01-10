use html::tokenizer;
use html::tree_builder;

fn main() {
  let target = r#"<!DOCTYPE html>
  <html>
  <body>
  
  <h2>HTML Images</h2>
  <p>HTML images are defined with the img tag:</p>
  
  <img src="w3schools.jpg" alt="W3Schools.com" width="104" height="142">
  
  </body>
  </html>
  "#
  .chars();

  let tokenizer = tokenizer::Tokenizer::new(target);
  let tree_builder = tree_builder::TreeBuilder::default(tokenizer);

  let document = tree_builder.run();

  document.print_tree(0);
}
