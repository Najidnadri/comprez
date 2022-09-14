use crate::{error::{DecompressError, CompressError}, Compressed, BinaryChunk};

pub trait Comprezable<Rhs = Self> {
    ///Compress function
    /// ## Example
    /// ```rust
    /// use comprez_macro::Comprezable;
    /// use comprez::{*, error::{CompressError, DecompressError}, comprezable::Comprezable};
    /// 
    /// #[derive(Comprezable, Debug)]
    /// struct MyStruct {
    ///     num1: u32,
    /// }
    /// fn main() {
    ///     let demo = MyStruct{num1: 99};
    ///      
    ///     let compressed = demo.compressed().unwrap();
    /// }
    /// ```
    fn compress(self) -> Result<Compressed, CompressError>;

    fn compress_to_binaries(self, max_num: Option<Rhs>) -> Result<Compressed, CompressError>;

    fn max_binaries(max_num: Option<Rhs>) -> BinaryChunk;

    fn decompress(compressed: Compressed) -> Result<Self, DecompressError> where Self: Sized;
}


impl Comprezable for u8 {
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }


    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<Self>) -> Result<Compressed, CompressError> {
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

use lz4_flex::{compress_prepend_size, decompress_size_prepended};

///Credit to LZ4 flex library for this compression
impl Comprezable for Vec<u8> {
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, _max_num: Option<Self>) -> Result<Compressed, CompressError> {
        let compressed = Compressed::Bytes(compress_prepend_size(self.as_slice()));
        let compressed_length = compressed.to_bytes().len();

        let compressed_metalength = Compressed::Binaries(compress_metalength(compressed_length));
        
        let res = compressed_metalength.combine(compressed);
        Ok(res)
    }

    fn max_binaries(_max_num: Option<Self>) -> BinaryChunk {
        BinaryChunk::Delimeter
    }

    fn decompress(compressed: Compressed) -> Result<Self, DecompressError> where Self: Sized {
        let decompressed = decompress_size_prepended(compressed.to_bytes().as_slice()).unwrap();
        Ok(decompressed)
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

fn compress_metalength(num: usize) -> Vec<u8> {
    //find the binary format of metalength
    let mut binaries = encode(num, 2, 8, 0).iter().map(|&bit| bit as u8).collect::<Vec<u8>>();

    //make full use of byte size
    let remainders = binaries.len() % 8;
    if remainders != 0 {
        for _n in 0 .. (8 - remainders) {
            binaries.insert(0, 0);
        }
    }

    //chunk to 7 element each, the last got inserted with 1, the output will be vector of  binaries of 8 
    let mut chunked_binaries = binaries.chunks(7).map(|chunk| {
        let mut temp = chunk.to_vec();
        temp.insert(0, 0);
        while temp.len() < 8 {
            temp.push(0)
        }
        temp
    }).collect::<Vec<Vec<u8>>>();
    chunked_binaries.last_mut().unwrap()[0] = 1;

    let res = chunked_binaries.into_iter().flatten().collect::<Vec<_>>();
    res
}


//turn integer to base 2, and then insert 0 to the front depends on the bit size
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


/* 
#[cfg(test)]
mod tests {
    use std::io::Read;

    use comprez_macro::Comprezable;

    use super::*;
    use super::super::chunk_up;


    #[derive(Comprezable, Debug, Clone)]
    struct Demo {
        #[maxNum=1000]
        num1: u32,
        vec: Vec::<u8>
    }

    #[test]
    fn it_works() {
        
        let mut file = std::fs::File::open("myself.jpg").unwrap();
        let mut buffer = vec![];
        let readed = file.read_to_end(&mut buffer).unwrap();
        println!("readed: {}", readed);

        let demo = Demo {
            num1: 10,
            vec: buffer,
        };
        let compressed = demo.clone().compress(None).unwrap();
        println!("compressed lenth: {:?}", compressed.to_bytes().len());

        
        let decompressed = Demo::decompress(compressed).unwrap().vec;

        assert_eq!(demo.vec, decompressed);
    }




    //normal static test
    #[derive(Comprezable, Debug, Clone)]
    struct Struct1 {
        #[maxNum=1000]
        num1: u32,
        #[maxNum=5000]
        num2: u64,
        #[maxNum=100]
        num3: i8,
    }

    #[test]
    fn normal_static_test() {
        let demo_data = Struct1 {
            num1: 1000,
            num2: 1774,
            num3: -78,
        };

        let a = u32::max_binaries(Some(1000));
        let b = u64::max_binaries(Some(5000));
        let c = i8::max_binaries(Some(100));
        println!("{:?}", [a, b, c]);

        let compressed = demo_data.compress(None).unwrap();
        println!("compressed: {:?}", compressed.to_binaries());
        let decompressed = Struct1::decompress(compressed).unwrap();

        println!("{:?}", decompressed);
    }
}
*/
