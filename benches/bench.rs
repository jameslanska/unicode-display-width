use criterion::{criterion_group, criterion_main, Criterion};
use unicode_display_width::width;

fn bench(bencher: &mut Criterion) {
    let random_unicode = "|asdf j;agj;asdlkj as;ldgj a≈ßƒ/woietl;,mxc,mjio;øˆ∆˙©†ƒ©¬˙∆≥ngar;lj092389034t[ergopgsfdmăܦؤـ|";

    bencher.bench_function("line with random Unicode", |b| {
        b.iter(|| width(random_unicode))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
