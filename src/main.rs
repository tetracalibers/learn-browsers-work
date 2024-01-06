use html::tokenizer;

fn main() {
  let target = r#"<p id='x'>"#.chars();
  print!("{:?}\n", target);

  let mut tokenizer = tokenizer::Tokenizer::new(target);
  loop {
    let token = tokenizer.next_token();
    println!("{:?}", token);

    if token.is_eof() {
      break;
    }
  }
}
