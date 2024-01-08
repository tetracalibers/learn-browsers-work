use html::tokenizer;
use html::tree_builder;

fn main() {
  let target = r#"<!DOCTYPE HTML PUBLIC "-//W3C//DTD HTML 4.01//EN"
  "http://www.w3.org/TR/html4/strict.dtd">"#
    .chars();

  let tokenizer = tokenizer::Tokenizer::new(target);
  let tree_builder = tree_builder::TreeBuilder::default(tokenizer);

  tree_builder.run();
}
