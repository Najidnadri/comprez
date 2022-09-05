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
//! use comprez::{*, error::{CompressError, DecompressError}};   
//! 
//! #[derive(Comprezable, Debug)]
//! struct MyStruct {
//!     [#maxNum=10000] //Compulsory for each field
//!     num1: u32,
//!     [#maxNum=888]
//!     num2: u16,
//!     [#maxNum=100] //from -100 to 100
//!     num3: i8,
//!     other_struct: OtherStruct
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
//!         other_struct: OtherStruct { num4: 200 }
//!     };
//!     
//!     let compressed = demo_data.compress(None).unwrap(); //Ignore the arguments, just put None.
//!     let compressed_bytes = compressed.to_bytes();
//!     let compressed_binaries = compressed.to_binaries();
//! 
//!     let decompressed = Mystruct::decompressed(compressed).unwrap();
//!     println!("{:?}", decompressed);
//! }
//! ```

use error::{CompressError, DecompressError};

pub mod error;


///
/// Trait for compressing structs
pub trait Comprezable<Rhs = Self> {
    ///Just put None for the max_num parameter
    fn compress(self, max_num: Option<Rhs>) -> Result<Compressed, CompressError>;

    fn max_binaries(max_num: Option<Rhs>) -> BinaryChunk;

    fn decompress(compressed: Compressed) -> Result<Self, DecompressError> where Self: Sized;
}


impl Comprezable for u8 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);

        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);

        Ok(Compressed::Binaries(compressed_binaries))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(compressed: Compressed) -> Result<u8, DecompressError> {
        let binaries = compressed.to_binaries();
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = u8::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u8", mult_8_bits)))
        })? * 8;
        let remainder = u8::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u8", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}

impl Comprezable for u16 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);
        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);
        let res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(compressed: Compressed) -> Result<u16, DecompressError> {
        let binaries = compressed.to_binaries();
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = u16::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u16", mult_8_bits)))
        })? * 8;
        let remainder = u16::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u16", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}

impl Comprezable for u32 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);
        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);
        let res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(compressed: Compressed) -> Result<u32, DecompressError> {
        let binaries = compressed.to_binaries();
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = u32::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u32", mult_8_bits)))
        })? * 8;
        let remainder = u32::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u32", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}
impl Comprezable for u64 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);
        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);
        let res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(compressed: Compressed) -> Result<u64, DecompressError> {
        let binaries = compressed.to_binaries();
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = u64::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u64", mult_8_bits)))
        })? * 8;
        let remainder = u64::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u64", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}
impl Comprezable for u128 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);
        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);
        let res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(compressed: Compressed) -> Result<u128, DecompressError> {
        let binaries = compressed.to_binaries();
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = u128::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u128", mult_8_bits)))
        })? * 8;
        let remainder = u128::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: u128", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}

 
//signed
impl Comprezable for i8 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self.abs() > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self, max_num))))
        }


        let mult_8_bit_size = find_mult_8_bit_size(max_num.abs(), 8, 0, 2);
        let compressed_binaries = compress_num(self.abs(), mult_8_bit_size, 8, 2, 0);
        let mut res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        if self < 0 {
            res.insert(0, 0)
        } else {
            res.insert(0, 1);
        }

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(compressed: Compressed) -> Result<i8, DecompressError> {
        let mut binaries = compressed.to_binaries();
        let sign = binaries.remove(0);
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = i8::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i8", mult_8_bits)))
        })? * 8;
        let remainder = i8::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i8", remainder_bits)))
        })?;

        let mut res = mult_8 + remainder;

        match sign {
            0 => {
                res = res * -1;
            },
            1 => (),
            _ => panic!()
        }
    
        Ok(res)
    }
}

impl Comprezable for i16 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self.abs() > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self, max_num))))
        }


        let mult_8_bit_size = find_mult_8_bit_size(max_num.abs(), 8, 0, 2);
        let compressed_binaries = compress_num(self.abs(), mult_8_bit_size, 8, 2, 0);
        let mut res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        if self < 0 {
            res.insert(0, 0)
        } else {
            res.insert(0, 1);
        }

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(compressed: Compressed) -> Result<i16, DecompressError> {
        let mut binaries = compressed.to_binaries();
        let sign = binaries.remove(0);
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = i16::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i16", mult_8_bits)))
        })? * 8;
        let remainder = i16::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i16", remainder_bits)))
        })?;

        let mut res = mult_8 + remainder;

        match sign {
            0 => {
                res = res * -1;
            },
            1 => (),
            _ => panic!()
        }
    
        Ok(res)
    }
}


impl Comprezable for i32 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self.abs() > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self, max_num))))
        }


        let mult_8_bit_size = find_mult_8_bit_size(max_num.abs(), 8, 0, 2);
        let compressed_binaries = compress_num(self.abs(), mult_8_bit_size, 8, 2, 0);
        let mut res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        if self < 0 {
            res.insert(0, 0)
        } else {
            res.insert(0, 1);
        }

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(compressed: Compressed) -> Result<i32, DecompressError> {
        let mut binaries = compressed.to_binaries();
        let sign = binaries.remove(0);
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = i32::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i32", mult_8_bits)))
        })? * 8;
        let remainder = i32::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i32", remainder_bits)))
        })?;

        let mut res = mult_8 + remainder;

        match sign {
            0 => {
                res = res * -1;
            },
            1 => (),
            _ => panic!()
        }
    
        Ok(res)
    }
}

impl Comprezable for i64 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self.abs() > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self, max_num))))
        }


        let mult_8_bit_size = find_mult_8_bit_size(max_num.abs(), 8, 0, 2);
        let compressed_binaries = compress_num(self.abs(), mult_8_bit_size, 8, 2, 0);
        let mut res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        if self < 0 {
            res.insert(0, 0)
        } else {
            res.insert(0, 1);
        }

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(compressed: Compressed) -> Result<i64, DecompressError> {
        let mut binaries = compressed.to_binaries();
        let sign = binaries.remove(0);
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = i64::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i64", mult_8_bits)))
        })? * 8;
        let remainder = i64::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i64", remainder_bits)))
        })?;

        let mut res = mult_8 + remainder;

        match sign {
            0 => {
                res = res * -1;
            },
            1 => (),
            _ => panic!()
        }
    
        Ok(res)
    }
}

impl Comprezable for i128 {
    fn compress(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap();
        if self.abs() > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self, max_num))))
        }


        let mult_8_bit_size = find_mult_8_bit_size(max_num.abs(), 8, 0, 2);
        let compressed_binaries = compress_num(self.abs(), mult_8_bit_size, 8, 2, 0);
        let mut res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        if self < 0 {
            res.insert(0, 0)
        } else {
            res.insert(0, 1);
        }

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<Self>) -> BinaryChunk {
        let max_num = max_num.unwrap();
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(compressed: Compressed) -> Result<i128, DecompressError> {
        let mut binaries = compressed.to_binaries();
        let sign = binaries.remove(0);
        let split_at = binaries.len() - 3;

        let (mult_8, remainder) = binaries.split_at(split_at);
    
    
        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = i128::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i128", mult_8_bits)))
        })? * 8;
        let remainder = i128::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}, integer value: i128", remainder_bits)))
        })?;

        let mut res = mult_8 + remainder;

        match sign {
            0 => {
                res = res * -1;
            },
            1 => (),
            _ => panic!()
        }
    
        Ok(res)
    }
}


fn compress_num<N: Ord + std::ops::Div<Output = N> + std::ops::Rem<Output = N> + Copy>(num: N, mult_8_bit_size: usize, divider: N, r: N, zero: N) -> Vec<N> {
    let mult_8 = num / divider; //divider = 8
    let remainder = num % divider;

    let mut mult_8_bit = encode(mult_8, r, mult_8_bit_size, zero); //r =2
    let remainder_bit = encode(remainder, r, 3, zero);
    mult_8_bit.extend(remainder_bit);

    mult_8_bit
}

fn encode<N: Ord + std::ops::Div<Output = N> + std::ops::Rem<Output = N> + Copy>(mut num: N, r: N, bit_size: usize, zero: N) -> Vec<N> {
    let mut res = vec![];
    while num != zero {
        let remainder = num % r;
        num = num / r;
        res.push(remainder);
    }
    res.reverse();
    while res.len() < bit_size {
        res.insert(0, zero);
    }
    res
}

fn find_mult_8_bit_size<N: Ord + std::ops::Div<Output = N> + Copy>(mut num: N, divider: N, zero: N, r: N) -> usize {

    num = num / divider; //divder = 8
    let mut res = 0; 
    while num != zero { //zero = 0
        num = num / r; // r = 2
        res += 1;
    }
    res
}


///Wrapper for compression results
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Compressed {
    Binaries(Vec<u8>),
    Bytes(Vec<u8>),
}

impl Compressed {
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

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Compressed::Bytes(bytes)
    }

    pub fn from_binaries(binaries: &[u8]) -> Self {
        Compressed::Binaries(binaries.to_vec())
    }

}


#[derive(Debug, Clone)]
pub enum BinaryChunk {
    Single(usize),
    Nested(Vec<BinaryChunk>),
}

impl BinaryChunk {
    pub fn flatten(&self) -> Vec<usize> {
        match self {
            Self::Single(size) => {
                vec![*size]
            },
            Self::Nested(sizes) => {
                sizes.iter().fold(vec![], |mut acc, chunk| { acc.extend(chunk.flatten()); acc})
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


pub fn chunk_up<'a>(mut binaries: &'a [u8], chunks: &'a [usize]) -> Result<Vec<&'a [u8]>, DecompressError> {
    let mut compressed = Vec::new();
    for chunk in chunks {
        if chunk > &binaries.len() {
            return Err(DecompressError::WrongBytesLength(String::new()))
        }
        let (left, right) = binaries.split_at(*chunk);
        compressed.push(left);
        binaries = right;
    }

    if !binaries.is_empty() {
        for binary in binaries {
            if binary != &0 {
                return Err(DecompressError::WrongBytesLength(String::new()))
            }
        }
    }
    Ok(compressed)
}


