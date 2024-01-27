extern crate fast_html;

use fast_html::debugger::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parse_html_benchmark(c: &mut Criterion) {
  c.bench_function("named_char_ref", |b| {
    b.iter(|| {
      let html =
        r#"<p>Put the &lt;h1> at the beginning of the heading and the &lt;h1> at the end.</p>"#;
      get_document_from_html(black_box(html));
    })
  });
}

criterion_group!(benches, parse_html_benchmark);
criterion_main!(benches);
