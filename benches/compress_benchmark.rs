use comprez::comprezable::{compress_metalength_v2};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/*
use comprez_macro::Comprezable;
use comprez::{*, error::{CompressError, DecompressError}, comprezable::Comprezable}; 

#[derive(Comprezable, Clone )]
struct Mystruct {
    #[maxNum=1000]
    num1: u32,
    #[maxNum=10000]
    num2: u64,
}
*/



fn compress_benchmark(c: &mut Criterion) {
    //let demo_data = black_box(Mystruct{num1: 900, num2: 1000} );
    //c.bench_function("compress benchmark", |b| b.iter(|| demo_data.clone().compress(None).unwrap()));
}
/* 
fn split_benchmark(c: &mut Criterion) {
    c.bench_function("split_benchmark_v2", |b| b.iter( || test_split(demo.clone())));
}


fn test_split(mut demo: &[u8]) {



    /* 
    let mut vec = vec![];
    for _i in 1 ..= 1000 {
        if demo.len() < 8 {
            break
        }
        vec.push(demo.drain(0 ..= 8).collect::<Vec<i32>>());
    }
    */
    
    
    let mut vec = vec![];
    for _i in 1..= 1000 {
        if demo.len() < 8 {
            break
        }
        let (left, right) = demo.split_at(8);
        vec.push(left);
        demo = right;
    }
    
    

}
*/

fn metalength_bench(c: &mut Criterion) {
    let demo_data = black_box::<usize>(100 );
    c.bench_function("metalength_bench", |b| b.iter( || metalength(demo_data)));
}

fn metalength(num: usize) {
    //let _ = compress_metalength(num);
    let _ = compress_metalength_v2(num);
} 



criterion_group!(benches, metalength_bench);
criterion_main!(benches);