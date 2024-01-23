use criterion::*;
use fast_html::tokenizer::token::Token;
use fast_html::tokenizer::Tokenizer;
use lol_html::HtmlRewriter;
use lol_html::Settings;

fn register_benchmark<F>(
  cr: &mut Criterion,
  input: &str,
  name: &str,
  mut code: F,
) where
  F: FnMut(),
{
  let input = &input;

  let mut group = cr.benchmark_group(name);
  group.throughput(Throughput::Bytes(input.as_bytes().len() as u64));
  group.bench_function(name, |b| {
    b.iter(|| {
      code();
    });
  });
  group.finish();
}

pub fn criterion_benchmark(cr: &mut Criterion) {
  let files = std::fs::read_dir("data").unwrap();

  for file in files {
    let file = file.unwrap().file_name();
    let file = file.to_str().unwrap();

    let path = format!("data/{}", file);
    let input = std::fs::read_to_string(&path).unwrap();

    register_benchmark(cr, &input, &format!("tl-{}", file), || {
      let _ = tl::parse(input.as_str(), tl::ParserOptions::default());
    });

    register_benchmark(cr, &input, &format!("lol_html-{}", file), || {
      let mut v = Vec::new();
      let mut rewriter =
        HtmlRewriter::new(Settings::default(), |c: &[u8]| {
          v.extend_from_slice(c);
        });

      let _ = rewriter.write(input.as_bytes());
    });

    register_benchmark(
      cr,
      &input,
      &format!("my_fast_html_tokenizer-{}", file),
      || {
        let mut tokenizer = Tokenizer::new(input.as_bytes());
        loop {
          if let Token::EOF = tokenizer.next_token() {
            break;
          }
        }
      },
    );
  }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
