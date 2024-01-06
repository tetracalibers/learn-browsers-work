use html::tokenizer;

fn main() {
  let mut tokenizer = tokenizer::Tokenizer::new("<p>Hello</p>".chars());
  loop {
    let token = tokenizer.next_token();
    println!("{:?}", token);

    if token.is_eof() {
      break;
    }
  }
}
