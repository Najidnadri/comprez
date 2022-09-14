
#[derive(Debug)]
pub enum CompressError {
    Overflow(String),
    Unknown(String),
    NotEven(String),
    EncodeErr(String),
    DataNoSupported(String),
}

impl CompressError {
    pub fn create(self) -> Self {
        match self {
            Self::Overflow(s) => { //s= given: {}, max-num: 263_882_790_666_238
                CompressError::Overflow(format!("Compress Error; Number overflow; {}", s))
            },
            Self::NotEven(s) => {
                CompressError::NotEven(format!("Compress Error; Number is not even; given:{}", s))
            },
            Self::EncodeErr(s) => {
                CompressError::EncodeErr(s)
            }
            Self::Unknown(s) => {
                CompressError::Unknown(s)
            },
            Self::DataNoSupported(_) => {
                CompressError::DataNoSupported(format!("Primitive data type is not supported for using this function"))
            }
        }
    }
}

#[derive(Debug)]
pub enum DecompressError {
    WrongBytesLength(String),
    BinariesToIntErr(String),
    Unknown(String),
    FromBytesErr(String),
    FromBinariesErr(String)
}

impl DecompressError {
    pub fn create(self) -> Self {
        match self {
            Self::WrongBytesLength(s) => {
                DecompressError::WrongBytesLength(format!("Decompress Error; Wrong bytes length; {}", s)) //given:2 length, should be: 1 length
            },
            Self::BinariesToIntErr(s) => {
                DecompressError::BinariesToIntErr(format!("Decompress Error; Error parsing the binaries given to integer; {}", s))
            },
            Self::FromBytesErr(_) => {
                DecompressError::FromBytesErr(format!("Parsing err: the vector given cannot be parse into a Compressed Bytes"))
            },
            Self::FromBinariesErr(_) => {
                DecompressError::FromBinariesErr(format!("Parsin err: the vector given cannot be parsed into a Compressed Binaries"))
            }
            Self::Unknown(s) => {
                DecompressError::Unknown(s)
            }
        }
    }
}

