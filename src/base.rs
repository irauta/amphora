
use std::fmt;
use std::error::Error;

use bitreader::{BitReader,BitReaderError};

pub type DeserializationResult<T> = Result<T, DeserializationError>;

#[derive(Debug,Clone,Copy)]
pub enum DeserializationError {
    InvalidSectionHeader,
    UnexpectedValue {
        position: u64,
        expected: i64,
        got: i64,
    },
    BitReaderError(BitReaderError)
}

impl Error for DeserializationError {
    fn description(&self) -> &str {
        match *self {
            DeserializationError::InvalidSectionHeader => "Invalid section header",
            DeserializationError::UnexpectedValue{..} => "Invalid value in the source data",
            DeserializationError::BitReaderError(ref err) => err.description(),
        }
    }
}

impl fmt::Display for DeserializationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializationError::InvalidSectionHeader => self.description().fmt(fmt),
            DeserializationError::UnexpectedValue{ position, expected, got } =>
                write!(fmt, "Expected {} at position {}, but got {}", expected, position, got),
            DeserializationError::BitReaderError(ref err) => err.fmt(fmt),
        }
    }
}

impl From<BitReaderError> for DeserializationError {
    fn from(err: BitReaderError) -> DeserializationError {
        DeserializationError::BitReaderError(err)
    }
}

pub trait Deserialize: Sized {
    fn deserialize(reader: &mut BitReader) -> DeserializationResult<Self>;

    fn from_bytes(bytes: &[u8]) -> DeserializationResult<Self> {
        let mut reader = BitReader::new(bytes);
        Deserialize::deserialize(&mut reader)
    }
}

pub fn reserved(reader: &mut BitReader, bits: u8) -> DeserializationResult<()> {
    expect(reader, bits, -1)
}

pub fn expect(reader: &mut BitReader, bits: u8, reference_value: i64) -> DeserializationResult<()> {
    let value = try!(reader.read_i64(bits));
    if value != reference_value {
        return Err(DeserializationError::UnexpectedValue {
            position: reader.position(),
            expected: reference_value,
            got: value,
        });
    }
    Ok(())
}
