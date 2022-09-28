// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! # Description
//! Comprez is a compression library for compressing any struct and enums
//! 
//! # Example
//! ```rust
//! 
//! use comprez_macro::Comprezable;
//! use comprez::comprezable::Comprezable;   
//! 
//! #[derive(Comprezable, Debug)]
//! struct MyStruct {
//!     #[maxNum=10000] //Compulsory for each field
//!     num1: u32,
//!     #[maxNum=888]
//!     num2: u16,
//!     #[maxNum=100] //from -100 to 100
//!     num3: i8,//use i8 instead of u8
//!     other_struct: OtherStruct,
//!     vec1: Vec<u8>,
//!     vec2: Vec<OtherStruct>,
//!     #[maxNum=200] 
//!     vec3: Vec<u16>
//! }
//! 
//! #[derive(Comprezable, Debug)]
//! struct OtherStruct {
//!     #[maxNum=1000000]
//!     num4: u128,
//! }
//!
//! fn main() {
//!     let demo_data = Mystruct {
//!         num1: 900,
//!         num2: 100,
//!         num3: 10,
//!         other_struct: OtherStruct { num4: 200 },
//!         vec1: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
//!         vec2: vec![OtherStruct{num4: 100}, OtherStruct{num4: 200}],
//!         vec3: vec[11, 12, 13, 14],
//!     };
//!     
//!     let compressed = demo_data.compress().unwrap()
//!     let compressed_bytes = compressed.to_bytes();
//!     let compressed_binaries = compressed.to_binaries();
//!     
//!     let compressed = Compressed::from_bytes(compressed_bytes);
//!     let compressed = Compressed::from_binaries(compressed_binaries);
//!     let decompressed = Mystruct::decompressed(compressed).unwrap();
//!     println!("{:?}", decompressed);
//! }
//! ```
//! 
//! ## Note
//! Since the compression of Vec<u8> uses LZ4 flex crate, compressing small vectors might increase the space instead. 
//! However in the future, self implementation will be created.


use comprezable::Comprezable;
use error::DecompressError;

pub mod error;

/// Trait for compressing structs
pub mod comprezable;







///Wrapper for compression results and decompression arguments
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Compressed {
    Binaries(Vec<u8>),
    Bytes(Vec<u8>),
}

impl Compressed {
    pub fn new() -> Self {
        Self::Binaries(vec![])
    }

    ///extract the bytes from the wrapper
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Self::Binaries(binaries) => {
                let mut binaries = binaries.clone();
                while binaries.len() % 8 != 0 {
                    binaries.push(0);
                }
                let chunked_binaries = binaries.chunks(8).collect::<Vec<&[u8]>>();
                let mut bytes = vec![];
                for chunk in chunked_binaries {
                    bytes.push(to_byte(chunk));
                }
                bytes
            },
            Self::Bytes(bytes) => {
                bytes.to_vec()
            },
        }
    }


    ///Extract the binaries from the wrapper
    pub fn to_binaries(&self) -> Vec<u8> {
        match self {
            Self::Binaries(binaries) => {
                binaries.to_vec()
            },
            Self::Bytes(bytes) => {
                to_binary(bytes.to_vec())
            }
        }
    }

    ///Handler not important
    pub fn extend_to_res(self, res: &mut Vec<u8>) {
        let binaries = self.to_binaries();
        res.extend(binaries);
    }


    pub fn combine(self, other: Compressed) -> Self {
        let mut binaries = self.to_binaries();
        let other_binaries = other.to_binaries();
        binaries.extend(other_binaries);

        Self::Binaries(binaries)
    }

    ///Vec<u8> to Compressed bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Compressed::Bytes(bytes)
    }

    ///Vec<u8> to Compressed binaries.
    pub fn from_binaries(binaries: &[u8]) -> Self {
        Compressed::Binaries(binaries.to_vec())
    }

}


#[derive(Debug, Clone)]
pub enum BinaryChunk {
    Single(usize),
    Nested(Vec<BinaryChunk>),
    Delimeter,
}

impl BinaryChunk {
    pub fn flatten(&self) -> Vec<usize> {
        match self {
            Self::Single(size) => {
                vec![*size]
            },
            Self::Nested(sizes) => {
                sizes.iter().fold(vec![], |mut acc, chunk| { acc.extend(chunk.flatten()); acc})
            },
            Self::Delimeter => {
                vec![0]
            }
        }
    }


    pub fn decompress<T: Comprezable>(&self, compressed: &mut Vec<u8>) -> Result<T, DecompressError> {

        match self {
            BinaryChunk::Single(size) => {
                T::decompress_from_binaries(compressed, Some(*size))
            },
            BinaryChunk::Nested(_) => {
                T::decompress_from_binaries(compressed, None)
            },
            BinaryChunk::Delimeter => {
                T::decompress_from_binaries(compressed, None)
            }
        }
    }
}


fn to_byte(bits: &[u8]) -> u8 {
    bits.iter()
    .fold(0, |result, &bit| {
        (result << 1) ^ bit
    })
}

fn to_binary(bytes: Vec<u8>) -> Vec<u8> {

    let mut binaries = vec![];
    for byte in bytes {
        let mut a = format!("{:b}", byte);
        while a.len() < 8 {
            a.insert(0, '0');
        }
        let temp_binaries = a.chars().map(|c| c.to_digit(2).unwrap() as u8).collect::<Vec<u8>>();
        binaries.extend(temp_binaries);
    }

    binaries
}






/* 
#[cfg(test)]
mod tests {
    use lz4_flex::decompress_size_prepended;

    use super::*;

    #[test]
    fn test_vec_u8() {
        

    }

}
*/