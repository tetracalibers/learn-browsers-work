use html::tokenizer;
use html::tree_builder;

fn main() {
  let target = r#"<!DOCTYPE html>
  <html>
  <head>
    <title>My First HTML</title>
    <meta charset="UTF-8">
  </head>
  <body>
  
  <p>The HTML head element contains meta data.</p>
  <p>Meta data is data about the HTML document.</p>
  
  </body>
  </html>
  "#
  .chars();

  let tokenizer = tokenizer::Tokenizer::new(target);
  let tree_builder = tree_builder::TreeBuilder::default(tokenizer);

  tree_builder.run();
}
