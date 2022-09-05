use criterion::{black_box, criterion_group, criterion_main, Criterion};


use comprez_macro::Comprezable;
use comprez::{*, error::{CompressError, DecompressError}}; 

#[derive(Comprezable, Clone )]
struct Mystruct {
    #[maxNum=1000]
    num1: u32,
    #[maxNum=10000]
    num2: u64,
}



fn compress_benchmark(c: &mut Criterion) {
    let demo_data = black_box(Mystruct{num1: 900, num2: 1000} );
    c.bench_function("compress benchmark", |b| b.iter(|| demo_data.clone().compress(None).unwrap()));
}


criterion_group!(benches, compress_benchmark);
criterion_main!(benches);