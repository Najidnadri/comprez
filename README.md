
# Rust Library for Compressing Structs.

A rust library for compressing and decompressing structs.
Note that the library is still under heavy development and breaking changes may occur.

## Description
The library is not complete yet, For now only struct with unsigned integers fields are supported. Many more will come. Stay tune.


## Installation

Put this into your cargo.toml

```
[dependencies]
comprez_macro = 0.1.0
comprez = 0.1.0
```



## Example

```
use comprez_macro::Comprezable;
use comprez::{*, error::{CompressError, DecompressError}};   

#[derive(Comprezable, Debug)]
struct MyStruct {
    [#maxNum=10000] //Compulsory for each field
    num1: u32,
    [#maxNum=888]
    num2: u16,
    [#maxNum=100]
    num3: u8,
    other_struct: OtherStruct
}

#[derive(Comprezable, Debug)]
struct OtherStruct {
    #[maxNum=1000000]
    num4: u128,
}

fn main() {
    let demo_data = Mystruct {
        num1: 900,
        num2: 100,
        num3: 10,
        other_struct: OtherStruct { num4: 200 }
    };
    
    let compressed = demo_data.compress(None).unwrap(); //Ignore the arguments, just put None.
    let compressed_bytes = compressed.to_bytes();
    let compressed_binaries = compressed.to_binaries();

    let decompressed = Mystruct::decompressed(compressed).unwrap();
    println!("{:?}", decompressed);
}
```
## Performance

TODO!


## Features

- [x] Unsigned 
- [ ] Signed 
- [ ] Big Int
- [ ] Even numbers
- [ ] Enum & Union
- [ ] Strings
- [ ] Slices & Vectors





## A coffee?
[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/najidnadri)