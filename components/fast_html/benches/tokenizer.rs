extern crate fast_html;

use fast_html::tokenizer::token::Token;
use fast_html::tokenizer::Tokenizer;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn tokenize(html: &str) {
  let mut tokenizer = Tokenizer::new(html.as_bytes());

  loop {
    if let Token::EOF = tokenizer.next_token() {
      break;
    }
  }
}

fn tokenize_html_byte_benchmark(c: &mut Criterion) {
  c.bench_function("tokenize_html_as_byte", |b| {
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

criterion_group!(benches, tokenize_html_byte_benchmark);
criterion_main!(benches);
