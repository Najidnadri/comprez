
use comprez_macro::{self, Comprezable};
use comprez::{*, error::{CompressError, DecompressError}};   

#[derive(Comprezable, Debug, PartialEq, PartialOrd, Clone)]
struct FatherStruct {
    #[maxNum=10000]
    num1: i32,
    #[maxNum=10000]
    num2: i64,
    #[maxNum=100]
    num3: u8,
    #[maxNum=1000]
    num4: u16,
    num5: SonStruct,
}

impl FatherStruct {
    pub fn new(num1: i32, num2: i64, num3: u8, num4: u16, num5: SonStruct) -> Self {
        FatherStruct { num1, num2, num3, num4, num5 }
    }
}

#[derive(Comprezable, Debug, PartialEq, PartialOrd, Clone)]
struct SonStruct {
    #[maxNum=10000]
    num6: i128,
    #[maxNum=100]
    num7: i8,
    num8: GrandSonStruct
}

impl SonStruct {
    pub fn new(num6: i128, num7: i8, num8: GrandSonStruct) -> Self {
        SonStruct { num6, num7, num8 }
    }
}

#[derive(Comprezable, Debug, PartialEq, PartialOrd, Clone)]
struct GrandSonStruct {
    #[maxNum=10000]
    num9: u128,
}

impl GrandSonStruct {
    pub fn new(num9: u128) -> Self {
        GrandSonStruct { num9 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::{self, thread_rng, Rng};

    #[test]
    fn internal() {
        for _i  in 0 ..= 10000 {
            let mut rng = thread_rng();
            let num1:i32 = rng.gen_range(-9999 ..= 9999); 
            let num2:i64 = rng.gen_range(-9999 ..= 9999); 
            let num3:u8 = rng.gen_range(0 ..= 99); 
            let num4:u16 = rng.gen_range(0 ..= 999); 
            let num6:i128 = rng.gen_range(-9999 ..= 9999); 
            let num7:i8 = rng.gen_range(-99 ..= 99); 
            let num9:u128 = rng.gen_range(0 ..= 9999); 
    
            let num8 = GrandSonStruct::new(num9);
            let num5 = SonStruct::new(num6, num7, num8);
            let data = FatherStruct::new(num1, num2, num3, num4, num5);
    
            let compressed = data.clone().compress(None).unwrap();
            let sized_down = 48 - compressed.to_bytes().len();
            println!("From 48 bytes to {}, cut down: {}", compressed.to_bytes().len(), sized_down);
            let decompressed = FatherStruct::decompress(compressed).unwrap();
            assert!(data.eq(&decompressed));
        }
    }
}