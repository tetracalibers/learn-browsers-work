extern crate html;

use html::debugger::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parse_html_benchmark(c: &mut Criterion) {
  c.bench_function("parse html", |b| {
    b.iter(|| {
      let html = r#"
            <h1>This is heading</h1>
            <p>This is paragraph</p>
            <p>This <mark>keyword</mark> is important</p>
            "#;
      get_document_from_html(black_box(html));
    })
  });
}

criterion_group!(benches, parse_html_benchmark);
criterion_main!(benches);
