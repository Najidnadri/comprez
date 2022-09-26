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

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError>;

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk;

    fn decompress(compressed: Compressed) -> Result<Self, DecompressError> where Self: Sized;

    fn decompress_from_binaries(compressed: &mut Vec<u8>, bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized;

}

/* 
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
*/

impl Comprezable for u16 {
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);
        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);
        let res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(_compressed: Compressed) -> Result<u16, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>,  bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}

impl Comprezable for u32 {
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;        
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);
        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);
        let res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(_compressed: Compressed) -> Result<u32, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>,  bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}
impl Comprezable for u64 {
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;        
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);
        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);
        let res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(_compressed: Compressed) -> Result<u64, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>,  bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}
impl Comprezable for u128 {
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;        
        if self > max_num {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: {}", self,max_num))))
        }
        let mult_8_bit_size = find_mult_8_bit_size(max_num, 8, 0, 2);
        let compressed_binaries = compress_num(self, mult_8_bit_size, 8, 2, 0);
        let res = compressed_binaries.iter().map(|&binary| binary as u8).collect::<Vec<u8>>();

        Ok(Compressed::Binaries(res))
    }

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num, 8, 0, 2) + 3)
    }

    fn decompress(_compressed: Compressed) -> Result<u128, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>,  bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
        })?;
    
        Ok(mult_8 + remainder)
    }
}

 
//signed
impl Comprezable for i8 {
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;        
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

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(_compressed: Compressed) -> Result<i8, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>, bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let mut compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let sign = compressed_binaries.remove(0);
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
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

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;        
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

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(_compressed: Compressed) -> Result<i16, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>, bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let mut compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let sign = compressed_binaries.remove(0);
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
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

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;        
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

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(_compressed: Compressed) -> Result<i32, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>, bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let mut compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let sign = compressed_binaries.remove(0);
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
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

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;        
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

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(_compressed: Compressed) -> Result<i64, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>, bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let mut compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let sign = compressed_binaries.remove(0);
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
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

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let max_num = max_num.unwrap() as Self;        
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

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let max_num = max_num.unwrap() as Self;
        BinaryChunk::Single(find_mult_8_bit_size(max_num.abs(), 8, 0, 2) + 4)
    }

    fn decompress(_compressed: Compressed) -> Result<i128, DecompressError> {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>, bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        if compressed.len() < bit_size.unwrap() {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("Not enough bytes"))))
        }
        let mut compressed_binaries = compressed.drain( .. bit_size.unwrap()).collect::<Vec<u8>>();
        let sign = compressed_binaries.remove(0);
        let split_at = compressed_binaries.len() - 3;

        let (mult_8, remainder) = compressed_binaries.split_at(split_at);

        let mult_8_bits = mult_8.iter().map(|bit| bit.to_string()).collect::<String>();
        let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();
    
        let mult_8 = Self::from_str_radix(&mult_8_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", mult_8_bits)))
        })? * 8;
        let remainder = Self::from_str_radix(&remainder_bits, 2).map_err(|_| {
            DecompressError::create(DecompressError::BinariesToIntErr(format!("Binaries given: {}", remainder_bits)))
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

    fn compress_to_binaries(self, _max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let compressed = Compressed::Bytes(compress_prepend_size(self.as_slice()));
        let compressed_length = compressed.to_bytes().len();

        let compressed_metalength = Compressed::Binaries(compress_metalength_v2(compressed_length));
      
        let res = compressed_metalength.combine(compressed);
        Ok(res)
    }

    fn max_binaries(_max_num: Option<u128>) -> BinaryChunk {
        BinaryChunk::Delimeter
    }

    fn decompress(_compressed: Compressed) -> Result<Self, DecompressError> where Self: Sized {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>, _bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        let mut chunk_metas: Vec<u8> = vec![];
        loop {
            if compressed.len() < 8 {
                return Err(DecompressError::WrongBytesLength(format!("Not enough bytes to calculate the metalength of compressed vector")))
            }
            let delimeter = compressed.remove(0);
            match delimeter {
                0 => {
                    chunk_metas.extend(compressed.drain(0 .. 7));
                },
                1 => {
                    chunk_metas.extend(compressed.drain(0 .. 7));
                    break
                },
                _ => {
                    panic!()
                }
            };
        }

        let chunk_metas = chunk_metas
        .into_iter()
        .map(|bit| bit.to_string())
        .collect::<String>();
    
    
        //binaries to int AKA; size (in bytes) of the compressed vec
        let meta = u128::from_str_radix(&chunk_metas, 2).unwrap();
        let length = compressed.len() as u128;
        if length < meta * 8 {
            return Err(DecompressError::WrongBytesLength(format!("length of compressed bytes is shorter than meta-length of compressed vector, AKA meta-length out of bound")))
        }
        

        let mut res_binaries: Vec<u8> = vec![];
        for _ in 1 ..= meta {
            res_binaries.extend(compressed.drain(0 .. 8));
        }

        //decode here
        let res_binaries = Compressed::Binaries(res_binaries).to_bytes();
        Ok(decompress_size_prepended(res_binaries.as_slice()).unwrap())
    }
}

use std::fmt::Debug;
impl<T: Comprezable + Clone + Debug> Comprezable for Vec<T> {
    fn compress(self) -> Result<Compressed, CompressError> {
        Err(CompressError::create(CompressError::DataNoSupported(String::new())))
    }

    fn compress_to_binaries(self, max_num: Option<u128>) -> Result<Compressed, CompressError> {
        let mut all_compressed = Compressed::new();
 
        for element in self {
            let compressed = element.compress_to_binaries(max_num)?;
            all_compressed = all_compressed.combine(compressed);
        }

        let len = all_compressed.to_bytes().len();

        let compressed_metalength = Compressed::Binaries(compress_metalength_v2(len));
        let res = compressed_metalength.combine(all_compressed);
        Ok(res)
    }

    fn max_binaries(max_num: Option<u128>) -> BinaryChunk {
        let binary_chunk = T::max_binaries(max_num);
        binary_chunk
    }

    fn decompress(_compressed: Compressed) -> Result<Self, DecompressError> where Self: Sized {
        Err(DecompressError::create(DecompressError::PrimitiveDataErr(String::new())))
    }

    fn decompress_from_binaries(compressed: &mut Vec<u8>, bit_size: Option<usize>) -> Result<Self, DecompressError> where Self:Sized {
        let size = calc_delimeter_size(compressed)?;

        let mut res_binaries: Vec<u8> = vec![];
        for _ in 1 ..= size {
            res_binaries.extend(compressed.drain(0 .. 8));
        }

        //decode here
        let mut res: Vec<T> = vec![];
        loop {
            match T::decompress_from_binaries(&mut res_binaries, bit_size) {
                Ok(t) => {
                    res.push(t);
                },
                Err(_) => {
                    break
                }
            }
        }
        Ok(res)
    }
}

fn calc_delimeter_size(compressed: &mut Vec<u8>) -> Result<u128, DecompressError> {
    let mut chunk_metas: Vec<u8> = vec![];
    loop {
        if compressed.len() < 8 {
            return Err(DecompressError::WrongBytesLength(format!("Not enough bytes to calculate the metalength of compressed vector")))
        }
        let delimeter = compressed.remove(0);
        match delimeter {
            0 => {
                chunk_metas.extend(compressed.drain(0 .. 7));
            },
            1 => {
                chunk_metas.extend(compressed.drain(0 .. 7));
                break
            },
            _ => {
                panic!()
            }
        };
    }

    let chunk_metas = chunk_metas
    .into_iter()
    .map(|bit| bit.to_string())
    .collect::<String>();


    //binaries to int AKA; size (in bytes) of the compressed vec
    let meta = u128::from_str_radix(&chunk_metas, 2).unwrap();
    let length = compressed.len() as u128;
    if length < meta * 8 {
        return Err(DecompressError::WrongBytesLength(format!("length of compressed bytes is shorter than meta-length of compressed vector, AKA meta-length out of bound. meta-length: {}, compressed bytes length: {}", meta, length)))
    }

    Ok(meta)
}




fn compress_num<N: Ord + std::ops::Div<Output = N> + std::ops::Rem<Output = N> + Copy>(num: N, mult_8_bit_size: usize, divider: N, r: N, zero: N) -> Vec<N> {
    let mult_8 = num / divider; //divider = 8
    let remainder = num % divider;

    let mut mult_8_bit = encode(mult_8, r, mult_8_bit_size, zero); //r =2
    let remainder_bit = encode(remainder, r, 3, zero);
    mult_8_bit.extend(remainder_bit);

    mult_8_bit
}

pub fn compress_metalength(num: usize) -> Result<Vec<u8>, CompressError> {
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
    chunked_binaries.last_mut()
    .ok_or(CompressError::create(CompressError::MetaForVecErr(String::new())))?
    [0] = 1;

    let res = chunked_binaries.into_iter().flatten().collect::<Vec<_>>();
    Ok(res)
}

pub fn compress_metalength_v2(num: usize) -> Vec<u8> {
    let mut binaries = encode(num, 2, 0, 0).iter().map(|&bit| bit as u8).collect::<Vec<u8>>();
    binaries.reverse();

    let mut reversed_chunked_meta = binaries.chunks(7).map(|chunk| {
        let mut temp = chunk.to_vec();

        while temp.len() < 7 {
            temp.push(0);
        }

        temp.push(0);
        temp.reverse();
        temp
    }).collect::<Vec<Vec<u8>>>();
    reversed_chunked_meta.reverse();
    reversed_chunked_meta.last_mut()
    .ok_or(CompressError::create(CompressError::MetaForVecErr(String::new()))).unwrap()
    [0] = 1;

    let res = reversed_chunked_meta.into_iter().flatten().collect::<Vec<u8>>();
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
