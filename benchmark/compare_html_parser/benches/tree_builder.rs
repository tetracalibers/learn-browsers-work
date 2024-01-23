use criterion::*;
use fast_html;
use html5ever::tendril::TendrilSink;
use markup5ever_rcdom as rcdom;

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

    register_benchmark(cr, &input, &format!("html5ever-{}", file), || {
      let _ =
        html5ever::parse_document(rcdom::RcDom::default(), Default::default())
          .from_utf8()
          .read_from(&mut input.as_bytes());
    });

    register_benchmark(cr, &input, &format!("my_fast_html-{}", file), || {
      fast_html::debugger::get_document_from_html(&input);
    });

    //register_benchmark(cr, &input, &format!("my_html-{}", file), || {
    //  html::debugger::get_document_from_html(&input);
    //});
  }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
