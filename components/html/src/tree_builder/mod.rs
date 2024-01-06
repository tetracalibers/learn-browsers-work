use super::tokenizer::Tokenizing;

pub struct TreeBuilder<T: Tokenizing> {
  tokenizer: T,
}

impl<T: Tokenizing> TreeBuilder<T> {
  pub fn new(tokenizer: T) -> Self {
    TreeBuilder { tokenizer }
  }

  pub fn run(&mut self) {
    loop {
      let token = self.tokenizer.next_token();
      println!("{:?}", token);

      if token.is_eof() {
        break;
      }
    }
  }
}
