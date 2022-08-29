use crate::unsigned::Unsigned;

mod error;
mod unsigned;
mod signed;

fn main() {
    let num = 1234u32;
    let compressed = Unsigned::compress_61_439(num).unwrap();
    let decompressed = Unsigned::decompress_61_439(compressed.to_vec()).unwrap();

    println!("size before compression: {}", num.to_be_bytes().len());
    println!("size after compression: {}", compressed.len());

    println!("{}", decompressed == num);


}







