use crate::error::DecompressError;

use super::super::Unsigned;

//decompress
impl Unsigned {
    pub fn decompress_478_even(bytes: Vec<u8>) -> Result<u32, DecompressError> {
        if bytes.len() != 1 {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("given: {}-length, shouldbe: 1-length", bytes.len()))))
        }

        let binaries = to_binaries(bytes)?;
        let num = decompress_num_even(binaries, 4)?;

        Ok(num)
    }

    pub fn decompress_122_878_even(bytes: Vec<u8>) -> Result<u32, DecompressError> {
        if bytes.len() != 2 {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("given: {}-length, shouldbe: 2-length", bytes.len()))))
        }

        let binaries = to_binaries(bytes)?;
        let num = decompress_num_even(binaries, 12)?;

        Ok(num)
    }

    pub fn decompress_61_439(bytes: Vec<u8>) -> Result<u32, DecompressError> {
        if bytes.len() != 2 {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("given: {}-length, shouldbe: 2-length", bytes.len()))))
        }

        let binaries = to_binaries(bytes)?;
        let num = decompress_num(binaries, 11)?;

        Ok(num)
    }

    //3 bytes
    pub fn decompress_31_457_278_even(bytes: Vec<u8>) -> Result<u64, DecompressError> {
        if bytes.len() != 3 {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("given: {}-length, shouldbe: 3-length", bytes.len()))))
        }

        let binaries = to_binaries(bytes)?;
        let num = decompress_num_even_u64(binaries, 20)?;

        Ok(num)
    }

    pub fn decompress_15_728_639(bytes: Vec<u8>) -> Result<u64, DecompressError> {
        if bytes.len() != 3 {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("given: {}-length, shouldbe: 3-length", bytes.len()))))
        }

        let binaries = to_binaries(bytes)?;
        let num = decompress_num_u64(binaries, 19)?;

        Ok(num)
    }

    //4bytes
    pub fn decompress_8_053_063_678_even(bytes: Vec<u8>) -> Result<u64, DecompressError> {
        if bytes.len() != 4 {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("given: {}-length, shouldbe: 4-length", bytes.len()))))
        }

        let binaries = to_binaries(bytes)?;
        let num = decompress_num_even_u64(binaries, 28)?;

        Ok(num)
    }

    pub fn decompress_4_026_531_839(bytes: Vec<u8>) -> Result<u64, DecompressError> {
        if bytes.len() != 4 {
            return Err(DecompressError::create(DecompressError::WrongBytesLength(format!("given: {}-length, shouldbe: 4-length", bytes.len()))))
        }

        let binaries = to_binaries(bytes)?;
        let num = decompress_num_u64(binaries, 27)?;

        Ok(num)
    }
}












fn to_binaries(bytes: Vec<u8>) -> Result<Vec<u8>, DecompressError> {

    let mut binaries = vec![];
    for byte in bytes {
        let mut a = format!("{:b}", byte);
        while a.len() != 8 {
            a.insert(0, '0');
        }
        let temp_binaries = a.chars().map(|c| c.to_digit(2).unwrap() as u8).collect::<Vec<u8>>();
        binaries.extend(temp_binaries);
    }

    Ok(binaries)
}

fn decompress_num_even(binaries: Vec<u8>, split_at: usize) -> Result<u32, DecompressError> {
    let (mult_25, remainder) = binaries.split_at(split_at);


    let mult_25_bits = mult_25.iter().map(|bit| bit.to_string()).collect::<String>();
    let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();

    let mult_25 = u32::from_str_radix(&mult_25_bits, 2).unwrap() * 30;
    let remainder = u32::from_str_radix(&remainder_bits, 2).unwrap() * 2;


    Ok(mult_25 + remainder)
}

fn decompress_num(binaries: Vec<u8>, split_at: usize) ->Result<u32, DecompressError> {
    let (mult_25, remainder_15) = binaries.split_at(split_at);
    let (minus_15, remainder) = remainder_15.split_at(1);


    let mult_25_bits = mult_25.iter().map(|bit| bit.to_string()).collect::<String>();
    let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();

    let mult_25 = u32::from_str_radix(&mult_25_bits, 2).unwrap() * 30;
    let remainder = u32::from_str_radix(&remainder_bits, 2).unwrap();
    let minus_15 = match minus_15[0] {
        1 => 15 as u32,
        0 => 0 as u32,
        _ => panic!()
    };

    Ok(mult_25 + remainder + minus_15)
}

fn decompress_num_even_u64(binaries: Vec<u8>, split_at: usize) -> Result<u64, DecompressError> {
    let (mult_25, remainder) = binaries.split_at(split_at);


    let mult_25_bits = mult_25.iter().map(|bit| bit.to_string()).collect::<String>();
    let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();

    let mult_25 = u64::from_str_radix(&mult_25_bits, 2).unwrap() * 30;
    let remainder = u64::from_str_radix(&remainder_bits, 2).unwrap() * 2;


    Ok(mult_25 + remainder)
}

fn decompress_num_u64(binaries: Vec<u8>, split_at: usize) ->Result<u64, DecompressError> {
    let (mult_25, remainder_15) = binaries.split_at(split_at);
    let (minus_15, remainder) = remainder_15.split_at(1);


    let mult_25_bits = mult_25.iter().map(|bit| bit.to_string()).collect::<String>();
    let remainder_bits = remainder.iter().map(|bit| bit.to_string()).collect::<String>();

    let mult_25 = u64::from_str_radix(&mult_25_bits, 2).unwrap() * 30;
    let remainder = u64::from_str_radix(&remainder_bits, 2).unwrap();
    let minus_15 = match minus_15[0] {
        1 => 15 as u64,
        0 => 0 as u64,
        _ => panic!()
    };

    Ok(mult_25 + remainder + minus_15)
}