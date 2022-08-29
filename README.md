
# Rust Library for Compressing big integers.

A rust library for compressing and decompressing big integers.
Note that the library is still under heavy development and breaking changes may occur.



## Example

```
let num = 1234u32;

let compressed = Unsigned::compress_61_439(num).unwrap();
let decompressed = Unsigned::decompress_61_439(compressed.to_vec()).unwrap();

println!("size before compression: {}", num.to_be_bytes().len());
//size before compression: 4

println!("size after compression: {}", compressed.len());
//size after compression: 2

println!("{}", decompressed == num);
//true
```
## Performance

TODO!


## Features

- [ ] Unsigned 
- [ ] Signed 
- [ ] Rust int type 
- [ ] Big Int
- [ ] Struct derive macro for compression





## A coffee?
[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/najidnadri)