// Copyright 2015 Ilkka Rauta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt;
use std::error::Error;

use bitreader::{BitReader,BitReaderError};

pub type DeserializationResult<T> = Result<T, DeserializationError>;

#[derive(Debug,Clone,Copy)]
pub enum DeserializationError {
    UnexpectedValue {
        position: u64,
        length: u8,
        expected: u64,
        got: u64,
    },
    BitReaderError(BitReaderError),
    ReadTooMuch {
        position: u64,
        max_position: u64,
    },
}

impl Error for DeserializationError {
    fn description(&self) -> &str {
        match *self {
            DeserializationError::UnexpectedValue{..} => "Invalid value in the source data",
            DeserializationError::BitReaderError(ref err) => err.description(),
            DeserializationError::ReadTooMuch{..} => "Read more data than allowed",
        }
    }
}

impl fmt::Display for DeserializationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializationError::UnexpectedValue{ position, length, expected, got } =>
                write!(fmt, "Expected {} at position {}..{}, but got {}", expected, position, position + length as u64, got),
            DeserializationError::BitReaderError(ref err) => err.fmt(fmt),
            DeserializationError::ReadTooMuch{ position, max_position } =>
                write!(fmt, "Read to position {}, when maximum allowed position was {}", position, max_position),
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

impl Deserialize for u8 {
    fn deserialize(reader: &mut BitReader) -> DeserializationResult<u8> {
        Ok(try!(reader.read_u8(8)))
    }
}

impl Deserialize for u16 {
    fn deserialize(reader: &mut BitReader) -> DeserializationResult<u16> {
        Ok(try!(reader.read_u16(8)))
    }
}

impl Deserialize for u32 {
    fn deserialize(reader: &mut BitReader) -> DeserializationResult<u32> {
        Ok(try!(reader.read_u32(8)))
    }
}

pub fn read_repeated<T: Deserialize>(max_bytes: usize, reader: &mut BitReader) -> DeserializationResult<Vec<T>> {
    let max_bits = max_bytes as u64 * 8;
    let mut repeat_reader = reader.relative_reader();
    let mut items = vec![];
    while repeat_reader.position() < max_bits {
        items.push(try!(Deserialize::deserialize(&mut repeat_reader)));
        if repeat_reader.position() > max_bits {
            return Err(DeserializationError::ReadTooMuch {
                position: repeat_reader.position(),
                max_position: max_bits,
            });
        }
    }
    try!(reader.skip(repeat_reader.position() as u32));
    Ok(items)
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
