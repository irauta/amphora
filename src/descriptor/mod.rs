
use std::fmt::Debug;

use ::base::*;
use bitreader::BitReader;

pub trait Descriptor: Debug {}

pub mod basic;

macro_rules! descriptor_match {
    (
        match $descriptor_tag:ident with $reader:ident {
            $($tag:pat => $T:ty),+
        }
    ) => (
        match $descriptor_tag {
            $(
                $tag => Ok(Box::new(try!(<$T as Deserialize>::deserialize($reader))))
            ),+
        }
    );
}

pub fn deserialize_descriptor(reader: &mut BitReader) -> DeserializationResult<Box<Descriptor>> {
    let mut tag_reader = reader.relative_reader();
    let descriptor_tag = try!(tag_reader.read_u8(8));
    descriptor_match!(
        match descriptor_tag with reader {
            2 => basic::VideoStreamDescriptor,
            3 => basic::AudioStreamDescriptor,
            4 => basic::HierarchyDescriptor,
            5 => basic::RegistrationDescriptor,
            6 => basic::DataStreamAlignmentDescriptor,
            7 => basic::TargetBackgroundGridDescriptor,
            8 => basic::VideoWindowDescriptor,
            9 => basic::CaDescriptor,
            10 => basic::Iso639LanguageDescriptor,
            11 => basic::SystemClockDescriptor,
            12 => basic::MultiplexBufferUtilizationDescriptor,
            13 => basic::CopyrightDescriptor,
            14 => basic::MaximumBitrateDescriptor,
            15 => basic::PrivateDataIndicatorDescriptor,
            16 => basic::SmoothingBufferDescriptor,
            17 => basic::StdDescriptor,
            18 => basic::IbpDescriptor,

            27 => basic::Mpeg4VideoDescriptor,
            28 => basic::Mpeg4AudioDescriptor,
            29 => basic::IodDescriptor,
            30 => basic::SlDescriptor,
            31 => basic::FmcDescriptor,
            32 => basic::ExternalEsIdDescriptor,
            33 => basic::MuxCodeDescriptor,
            34 => basic::FmxBufferSizeDescriptor,
            35 => basic::MultiplexBufferDescriptor,
            _ => basic::UnknownDescriptor
        }
    )
}

fn bits_remaining(descriptor_length: u8, reader: &BitReader) -> u32 {
    let intro_bits = 16; // descriptor_tag (8 bits) + descriptor_length (8 bits)
    let data_bits = descriptor_length as u32 * 8; // How many data bits after intro bits
    let total_descriptor_bits = intro_bits + data_bits;
    total_descriptor_bits - reader.position() as u32
}
