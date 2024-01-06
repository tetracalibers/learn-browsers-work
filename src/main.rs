use html::tokenizer;
use html::tree_builder;

fn main() {
  let target = r#"<p id='x'>"#.chars();
  print!("{:?}\n", target);

  let tokenizer = tokenizer::Tokenizer::new(target);
  let mut tree_builder = tree_builder::TreeBuilder::new(tokenizer);

  tree_builder.run();
}
