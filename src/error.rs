
#[derive(Debug)]
pub enum CompressError {
    Overflow(String),
    Unknown(String),
    NotEven(String),
    EncodeErr(String),
}

impl CompressError {
    pub fn create(self) -> Self {
        match self {
            Self::Overflow(s) => {
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
        }
    }
}

#[derive(Debug)]
pub enum DecompressError {
    WrongBytesLength(String),
    Unknown(String),
}

impl DecompressError {
    pub fn create(self) -> Self {
        match self {
            Self::WrongBytesLength(s) => {
                DecompressError::WrongBytesLength(format!("Decompress Error; Wrong bytes length; {}", s)) //given:2 length, should be: 1 length
            },
            Self::Unknown(s) => {
                DecompressError::Unknown(s)
            }
        }
    }
}

