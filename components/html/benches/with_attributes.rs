extern crate html;

use html::debugger::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parse_html_benchmark(c: &mut Criterion) {
  c.bench_function("with_attributes", |b| {
    b.iter(|| {
      let html =
        r#"<a href="https://example.com" target="_blank">sample link</a>"#;
      get_document_from_html(black_box(html));
    })
  });
}

criterion_group!(benches, parse_html_benchmark);
criterion_main!(benches);
