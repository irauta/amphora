
use std::fmt;
use std::error::Error;

use bitreader::{BitReader,BitReaderError};

pub type DeserializationResult<T> = Result<T, DeserializationError>;

#[derive(Debug,Clone,Copy)]
pub enum DeserializationError {
    InvalidSectionHeader,
    UnexpectedValue {
        position: u64,
        length: u8,
        expected: u64,
        got: u64,
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
            DeserializationError::UnexpectedValue{ position, length, expected, got } =>
                write!(fmt, "Expected {} at position {}..{}, but got {}", expected, position, position + length as u64, got),
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
    let all_on: u64 = !0;
    let expected = all_on >> (64 - bits);
    expect(reader, bits, expected)
}

pub fn expect(reader: &mut BitReader, bits: u8, reference_value: u64) -> DeserializationResult<()> {
    let position = reader.position();
    let value = try!(reader.read_u64(bits));
    if value != reference_value {
        return Err(DeserializationError::UnexpectedValue {
            position: position,
            length: bits,
            expected: reference_value,
            got: value,
        });
    }
    Ok(())
}

pub fn bool_flag(value: u8) -> bool {
    value == 1
}
