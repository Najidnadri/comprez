 
use comprez_macro::{self, Comprezable};
use comprez::{*, error::{CompressError, DecompressError}, comprezable::Comprezable};   

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
    fn random_int() {
        let mut count = 0;
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
    
            let compressed = data.clone().compress().unwrap();
            let decompressed = FatherStruct::decompress(compressed).unwrap();
            assert!(data.eq(&decompressed));
            count += 1;
        }
        println!("random int done: {} test done", count);
    }


    #[derive(Comprezable, Debug, PartialEq, PartialOrd, Clone)]
    struct RandomVec {
        data: Vec<u8>,
    }

    #[test]
    fn random_vec() {
        let mut rng = thread_rng();
        let mut count = 1;
        for _i in 0..= 100 {
            let random_vec = (0 ..= 10000).map(|_| {
                let byte: u8 = rng.gen_range(0 ..= 255);
                byte
            }).collect::<Vec<u8>>();

            let data = RandomVec {
                data: random_vec
            };

            let compressed = data.clone().compress().unwrap();
            let decompressed = RandomVec::decompress(compressed).unwrap();

            assert_eq!(data, decompressed);
            count += 1;
            if count % 100 == 0 {
                println!("{} random vec test done", count);
            }
        }

        println!("random vec done: {} tests done", count);
    }




    struct AllRandomFather {
        num1: u32,
        num2: i8,
        son: AllRandomSon,
    }

    struct AllRandomSon {
        num3: i64,
        num4: i32,
        grand_son: AllRandomGrandson,
    }

    struct AllRandomGrandson {
        vec1: Vec::<u8>
    }

    #[test]
    fn all_random() {

    }


    #[derive(Comprezable, Debug)]
    struct MyStruct {
        #[maxNum=1000]
        num1: u32,
        #[maxNum=10000]
        num2: u64,
        num3: Idk,
        num4: Vec<u8>,
    }

    #[derive(Comprezable, Debug)]
    struct Idk {
        #[maxNum=500]
        num1: i64
    }

    #[test]
    fn playground() {
        let demo = MyStruct {
            num1: 900,
            num2: 1000,
            num3: Idk {num1: -75},
            num4: vec![],
        };
        let a = demo.compress().unwrap();
        println!("{:?}", a.to_binaries().split_at(24).1);
        println!("{:?}", a.to_binaries().split_at(34).1); 
        let decompressed = MyStruct::decompress(a).unwrap();
        println!("{:?}", decompressed);
    }

    #[test]
    fn playground1() {
        let demo = Compressed::Binaries(vec![1, 1, 1, 0, 0, 0, 0, 1, 0, 0]);
        let num = u32::decompress(demo).unwrap();
        let demo = Compressed::Binaries(vec![0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0]);
        let num2 = u64::decompress(demo).unwrap();
        let demo = Compressed::Binaries(vec![0,0,0,1, 0, 0, 1, 0, 1, 1]);
        let num3 = i64::decompress(demo).unwrap();

        println!("{}", num);
        println!("{}", num2);
        println!("{}", num3);
    }
}
