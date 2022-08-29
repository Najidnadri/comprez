use std::char::from_digit;

use crate::error::CompressError;

use super::Unsigned;

//compress
impl Unsigned {
    //1 byte
    pub fn compress_478_even(num: u32) -> Result<[u8; 1], CompressError> {
        if num % 2 != 0 {
            return Err(CompressError::create(CompressError::NotEven(format!("{}", num))))
        }
        if num > 479 {
            return Err(CompressError::create(CompressError::Overflow(format!("given:{}, max-num: 478", num))))
        }

        let mult_25 = num / 30;
        let remainder = num % 30;
    
    
        let mut mult_25_bit = encode(mult_25, 2, 4).unwrap();
        let remainder_bit = encode(remainder/2, 2, 4).unwrap();
        mult_25_bit.extend(remainder_bit);

        let bytes = to_byte(mult_25_bit);
        Ok([bytes])
    } 

    //2 bytes
    pub fn compress_122_878_even(num: u32) ->Result<[u8; 2], CompressError> {
        if num % 2 != 0 {
            return Err(CompressError::create(CompressError::NotEven(format!("{}", num))))
        }
        if num > 122_878 {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: 122_878", num))))
        } 

        let mult_25 = num / 30;
        let remainder = num % 30;
    
    
        let mut mult_25_bit = encode(mult_25, 2, 12).unwrap();
        let remainder_bit = encode(remainder/2, 2, 4).unwrap();
        mult_25_bit.extend(remainder_bit);

        let bytes = mult_25_bit.chunks(8).collect::<Vec<&[u8]>>();
        let mut res = vec![];
        for byte in bytes {
            res.push(to_byte(byte.to_vec()));
        }

        Ok([res[0], res[1]])
    }

    pub fn compress_61_439(num: u32) -> Result<[u8;2], CompressError> {
        if num > 61_439 {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: 61_439", num))))
        } 

        let mult_25 = num / 30;
        let mut remainder = num % 30;
    
    
        let mut mult_25_bit = encode(mult_25, 2, 11).unwrap();
        let minus_12 = match remainder > 15 {
            true => {
                remainder -= 15;
                1u8
            },
            false => {
                0u8
            }
        };
        let remainder_bit = encode(remainder, 2, 4).unwrap();
        mult_25_bit.push(minus_12);
        mult_25_bit.extend(remainder_bit);

        let bytes = mult_25_bit.chunks(8).collect::<Vec<&[u8]>>();
        let mut res = vec![];
        for byte in bytes {
            res.push(to_byte(byte.to_vec()));
        }

        Ok([res[0], res[1]])
    }

    //3bytes
    pub fn compress_31_457_278_even(num: u64) -> Result<[u8;3], CompressError> {
        if num % 2 != 0 {
            return Err(CompressError::create(CompressError::NotEven(format!("{}", num))))
        }
        if num > 31_457_278 {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: 31_457_278", num))))
        } 

        let mult_25 = num / 30;
        let remainder = num % 30;
    
    
        let mut mult_25_bit = encode_64(mult_25, 2, 20).unwrap();
        let remainder_bit = encode_64(remainder/2, 2, 4).unwrap();
        mult_25_bit.extend(remainder_bit);

        let bytes = mult_25_bit.chunks(8).collect::<Vec<&[u8]>>();
        let mut res = vec![];
        for byte in bytes {
            res.push(to_byte(byte.to_vec()));
        }

        Ok([res[0], res[1], res[2]])
    }
    
    pub fn compress_15_728_639(num: u64) -> Result<[u8; 3], CompressError> {
        if num > 15_728_639 {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: 15_728_639", num))))
        }
        let mult_25 = num / 30;
        let mut remainder = num % 30;
    
        let mut mult_25_bit = encode_64(mult_25, 2, 19).unwrap();
        let minus_12 = match remainder > 15 {
            true => {
                remainder -= 15;
                1u8
            },
            false => {
                0u8
            }
        };
        let remainder_bit = encode_64(remainder, 2, 4).unwrap();
        mult_25_bit.push(minus_12);
        mult_25_bit.extend(remainder_bit);

        let bytes = mult_25_bit.chunks(8).collect::<Vec<&[u8]>>();
        let mut res = vec![];
        for byte in bytes {
            res.push(to_byte(byte.to_vec()));
        }

        Ok([res[0], res[1], res[2]])
    
    }

    //4bytes
    pub fn compress_8_053_063_678_even(num: u64) -> Result<[u8; 4], CompressError> {
        if num % 2 != 0 {
            return Err(CompressError::create(CompressError::NotEven(format!("{}", num))))
        }
        if num > 8_053_063_678 {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: 8_053_063_678", num))))
        } 

        let mult_25 = num / 30;
        let remainder = num % 30;
    
    
        let mut mult_25_bit = encode_64(mult_25, 2, 28).unwrap();
        let remainder_bit = encode_64(remainder/2, 2, 4).unwrap();
        mult_25_bit.extend(remainder_bit);

        let bytes = mult_25_bit.chunks(8).collect::<Vec<&[u8]>>();
        let mut res = vec![];
        for byte in bytes {
            res.push(to_byte(byte.to_vec()));
        }

        Ok([res[0], res[1], res[2], res[3]])

    }

    pub fn compress_4_026_531_839(num: u64) -> Result<[u8;4], CompressError> {
        if num > 4_026_531_839 {
            return Err(CompressError::create(CompressError::Overflow(format!("given: {}, max-num: 4_026_531_839", num))))
        } 

        let mult_25 = num / 30;
        let mut remainder = num % 30;
    
        let mut mult_25_bit = encode_64(mult_25, 2, 27).unwrap();
        let minus_12 = match remainder > 15 {
            true => {
                remainder -= 15;
                1u8
            },
            false => {
                0u8
            }
        };
        let remainder_bit = encode_64(remainder, 2, 4).unwrap();
        mult_25_bit.push(minus_12);
        mult_25_bit.extend(remainder_bit);

        let bytes = mult_25_bit.chunks(8).collect::<Vec<&[u8]>>();
        let mut res = vec![];
        for byte in bytes {
            res.push(to_byte(byte.to_vec()));
        }

        Ok([res[0], res[1], res[2], res[3]])
    }
 }







fn encode(mut n: u32, r: u32, bit_size: usize) -> Result<Vec<u8>, CompressError> {

    let mut binaries = Vec::new();
    loop {
       if let Some(c) = from_digit(n % r, r) {
          binaries.insert(0, c.to_digit(2).unwrap() as u8);
       } else {
          return Err(CompressError::EncodeErr(format!("Error encoding {} to base {}", n, r)))
       }
       n /= r;
       if n == 0 {
          break
       }
    }
    while binaries.len() < bit_size {
        binaries.insert(0, 0);
    }
    Ok(binaries)
}

fn encode_64(mut num: u64, r: u64, bit_size: usize) -> Result<Vec<u8>, CompressError> {
    let mut res = vec![];
    while num != 0 {
        let remainder = num % r;
        num = num / r;
        res.push(remainder as u8);
    }
    res.reverse();
    while res.len() < bit_size {
        res.insert(0, 0);
    }
    Ok(res)
}


fn to_byte(bits: Vec<u8>) -> u8 {
    bits.iter()
    .fold(0, |result, &bit| {
        (result << 1) ^ bit
    })
}