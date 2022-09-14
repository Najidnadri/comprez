// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


 
//! ```rust
//! use comprez_macro::Comprezable;
//! use comprez::{*, error::{CompressError, DecompressError}, comprezable::Comprezable};   
//! 
//! #[derive(Comprezable, Debug)]
//! struct MyStruct {
//!     [#maxNum=10000] //Compulsory for each field
//!     num1: u32,
//!     [#maxNum=888]
//!     num2: u16,
//!     [#maxNum=100] //from -100 to 100
//!     num3: i8,
//!     other_struct: OtherStruct,
//!     vec1: Vec<u8>
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

use core::panic;

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

    pub fn chunk_up_v2(binaries: &mut Vec<u8>, vec: Vec<Self>) -> Result<Vec<Vec<u8>>, DecompressError> {
        let mut res: Vec<Vec<u8>> = vec![];
        for binarychunk in vec {
            match binarychunk {
                Self::Single(s) => {
                    let compressed = binaries.drain(0 .. s).collect::<Vec<_>>();
                    res.push(compressed);
                },
                Self::Nested(chunks) => {
                    //let mut chunked_binaries = vec![];
                    let a = BinaryChunk::chunk_up_v2(binaries, chunks).unwrap().into_iter().flatten().collect::<Vec<_>>();
                    res.push(a);
                },
                Self::Delimeter => {
                    let compressed = delimeter_chunk_v2(binaries).unwrap();
                    res.push(compressed);
                }
            }
        }
        
        Ok(res)
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



fn delimeter_chunk_v2(binaries: &mut Vec<u8>) -> Result<Vec<u8>, DecompressError> {
    //chunk the meta length properly
    let mut chunk_metas: Vec<u8> = vec![];
    loop {
        if binaries.len() < 8 {
            return Err(DecompressError::WrongBytesLength(String::new()))
        }
        let delimeter = binaries.remove(0);
        match delimeter {
            0 => {
                chunk_metas.extend(binaries.drain(0 .. 7));
            },
            1 => {
                chunk_metas.extend(binaries.drain(0 .. 7));
                break
            },
            _ => {
                panic!()
            }
        };
    }
    
    //turn the chunked to parseable binaries
    let chunk_metas = chunk_metas.chunks(8).filter(|&chunk| chunk.len() == 8).collect::<Vec<&[u8]>>()
    .into_iter()
    .flatten()
    .map(|bit| bit.to_string())
    .collect::<String>();

    //binaries to int AKA; size (in bytes) of the compressed vec
    let meta = u128::from_str_radix(&chunk_metas, 2).unwrap();
    let length = binaries.len() as u128;
    if length < meta * 8 {
        return Err(DecompressError::WrongBytesLength(String::new()))
    }
    
    

    let mut res_binaries: Vec<u8> = vec![];
    for _ in 1 ..= meta {
        if binaries.len() < 8 {
            return Err(DecompressError::WrongBytesLength(String::new()))
        }
        res_binaries.extend(binaries.drain(0 .. 8));
    }
    
    Ok(res_binaries)
}



#[cfg(test)]
mod tests {
    use lz4_flex::decompress_size_prepended;

    use super::*;

    #[test]
    fn test_vec_u8() {
        
        let raw = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let demo = vec![8u8, 128, 11, 0, 0, 0, 176, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let compressed = Compressed::Bytes(demo);
        let mut binaries = compressed.to_binaries();
        

        let a = delimeter_chunk_v2(&mut binaries).unwrap();
        let decompressed = Compressed::Binaries(a).to_bytes();
        println!("decompressed: {:?}", decompressed);

        //binary format
        let b = decompress_size_prepended(&decompressed).unwrap();

        println!(" raw: {:?}", raw);
        println!("decompressed: {:?}",  b);

        assert_eq!(raw, b);

    }

    #[test]
    fn binary_chunk_v2() {
        let demo_nested = vec![BinaryChunk::Single(1), BinaryChunk::Single(3), BinaryChunk::Nested(vec![BinaryChunk::Single(5)]), BinaryChunk::Single(2)];
        let demo = BinaryChunk::Nested(demo_nested);
        let mut demo_binaries = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10,11, 12, 13, 14, 15, 16];
        
        if let BinaryChunk::Nested(chunks) = demo {
            let _chunk = BinaryChunk::chunk_up_v2(&mut demo_binaries, chunks);
        }
 
    }
}