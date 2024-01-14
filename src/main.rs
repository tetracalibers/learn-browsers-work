use html::tokenizer;
use html::tree_builder;

use dom::node::Node;

use tree::TreeNode;

use css;

fn run_html() {
  let target = r#"<!DOCTYPE html>
  <html>
  <body>
  
  <h2>An Unordered HTML List</h2>
  <p href="main">This is a paragraph.</p>
  
  </body>
  </html>
  "#
  .chars();

  let tokenizer = tokenizer::Tokenizer::new(target);
  let tree_builder = tree_builder::TreeBuilder::default(tokenizer);

  let document = tree_builder.run();

  document.print_tree(0);

  println!("-------------------");

  let root = document.0;

  // build Vec<(usize, TreeNode<Node>)>
  let mut node_list_with_depth = Vec::new();
  let depth = 0;

  // 兄弟要素も含めて深さ優先で走査
  fn traverse(
    node: TreeNode<Node>,
    depth: usize,
    node_list_with_depth: &mut Vec<(usize, TreeNode<Node>)>,
  ) {
    // 空文字しかないテキストノード以外をpush
    let mut append_node = true;

    if let Some(text_node) = node.as_maybe_text() {
      if text_node.characters.get_data().trim().is_empty() {
        append_node = false;
      }
    }

    if append_node {
      node_list_with_depth.push((depth, node.clone()));
    }

    if let Some(first_child) = node.first_child() {
      traverse(first_child, depth + 1, node_list_with_depth);
    }
    if let Some(next_sibling) = node.next_sibling() {
      traverse(next_sibling, depth, node_list_with_depth);
    }
  }

  traverse(root, depth, &mut node_list_with_depth);

  println!("{:?}", node_list_with_depth);
}

fn run_css() {
  css::parser::selector::main();
}

fn main() {
  run_html();
}
