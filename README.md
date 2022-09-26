
# Rust Library for Compressing Structs.

A rust library for compressing and decompressing structs.
Note that the library is still under heavy development and breaking changes may occur.

## Description
The library is not complete yet, For now only struct with unsigned integers fields are supported. Many more will come. Stay tune.


## Installation

Put this into your cargo.toml

```
[dependencies]
comprez_macro = 0.2.6
comprez = 0.2.6
```



## Example

```
use comprez_macro::Comprezable;
use comprez::{*, error::{CompressError, DecompressError}};   

#[derive(Comprezable, Debug, Clone)]
struct MyStruct {
    [#maxNum=10000] //Compulsory for each field
    num1: u32,
    [#maxNum=888]
    num2: u16,
    [#maxNum=100]
    num3: i8, //use i8 instead of u8
    other_struct: OtherStruct,
    vec1: Vec<u8>,
    vec2: Vec<OtherStruct>
}

#[derive(Comprezable, Debug, Clone)]
struct OtherStruct {
    #[maxNum=1000000]
    num4: u128,
}

fn main() {
    let demo_data = Mystruct {
        num1: 900,
        num2: 100,
        num3: 10,
        other_struct: OtherStruct { num4: 200 },
        vec1: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        vec2: vec![OtherStruct{num4: 100}, OtherStruct{num4: 200}],
    };
    
    let compressed = demo_data.compress().unwrap();
    let compressed_bytes = compressed.to_bytes();
    let compressed_binaries = compressed.to_binaries();

    let compressed = Compressed::from_bytes(compressed_bytes);
    let compressed = Compressed::from_binaries(compressed_binaries);
    let decompressed = Mystruct::decompressed(compressed).unwrap();
    println!("{:?}", decompressed);
}
```
## Performance

TODO!


## Features

- [x] Unsigned 
- [x] Signed 
- [x] Vec::u8
- [x] Vec of Comprezable(s)
- [ ] enums
- [ ] Async Write/Read 
- [ ] Even numbers
- [ ] Strings
- [ ] Slices & Vectors


## Credits

- Vector of u8 are compressed using LZ4 flex library.


## A coffee?
[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/najidnadri)