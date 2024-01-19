extern crate html;

use html::tokenizer::token::Token;
use html::tokenizer::Tokenizer;
use html::tokenizer::Tokenizing;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn tokenize(html: &str) {
  let mut tokenizer = Tokenizer::new(html.chars());

  loop {
    if let Token::EOF = tokenizer.next_token() {
      break;
    }
  }
}

fn tokenize_html_benchmark(c: &mut Criterion) {
  c.bench_function("tokenize_html", |b| {
    b.iter(|| {
      let html = r#"
            <h1>This is heading</h1>
            <p>This is paragraph</p>
            <p>This <mark>keyword</mark> is important</p>
            "#;
      tokenize(black_box(html));
    })
  });
}

criterion_group!(benches, tokenize_html_benchmark);
criterion_main!(benches);
