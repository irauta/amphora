
//! # Program Specific Information (PSI)

use ::base::*;
use ::section::section_bytes_left;
use ::descriptor::{Descriptor,deserialize_descriptor};

bit_struct!(
    #[derive(Debug,Clone)]
    pub struct ProgramAssociationSection {
        pub transport_stream_id: u16,
        pub version_number: u8,
        pub current_next_indicator: bool,
        pub section_number: u8,
        pub last_section_number: u8,
        pub associations: Vec<ProgramAssociation>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0 }, // table_id
        expect: { bits: 1, reference: 1 }, // section_syntax_indicator
        expect: { bits: 1, reference: 0 }, // just a constant zero bit
        reserved: { 2 },
        section_length: { 12, type: u16 },
        transport_stream_id: { 16 },
        reserved: { 2 },
        version_number: { 5 },
        current_next_indicator: { 1, map: |b: u8| b == 1 },
        section_number: { 8 },
        last_section_number: { 8 },
        associations: { value: {
            let association_size = 4; // How many bytes for single program association
            let mut associations = vec![];
            while section_bytes_left(section_length, reader) >= association_size {
                associations.push(try!(Deserialize::deserialize(reader)));
            }
            associations
        } },
        crc: { 32 }
    }
);

#[derive(Debug,Copy,Clone)]
pub enum ProgramAssociation {
    NetworkPid(u16),
    ProgramMapPid{
        program_number: u16,
        program_map_pid: u16,
    },
}

impl Deserialize for ProgramAssociation {
    fn deserialize(reader: &mut ::bitreader::BitReader) -> DeserializationResult<Self> {
        let program_number = try!(reader.read_u16(16));
        try!(reserved(reader, 3));
        let pid = try!(reader.read_u16(13));
        let association = if program_number == 0 {
            ProgramAssociation::NetworkPid(pid)
        } else {
            ProgramAssociation::ProgramMapPid {
                program_number: program_number,
                program_map_pid: pid,
            }
        };
        Ok(association)
    }
}


bit_struct!(
    #[derive(Debug)]
    pub struct ConditionalAccessSection {
        pub version_number: u8,
        pub current_next_indicator: bool,
        pub section_number: u8,
        pub last_section_number: u8,
        pub descriptors: Vec<Box<Descriptor>>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 1 }, // table_id
        expect: { bits: 1, reference: 1 }, // section_syntax_indicator
        expect: { bits: 1, reference: 0 }, // just a constant zero bit
        reserved: { 2 },
        section_length: { 12, type: u16 },
        reserved: { 18 },
        version_number: { 5 },
        current_next_indicator: { 1, map: |b: u8| b == 1 },
        section_number: { 8 },
        last_section_number: { 8 },
        descriptors: { value: {
            let mut descriptors = vec![];
            while section_bytes_left(section_length, reader) > 0 {
                descriptors.push(try!(deserialize_descriptor(reader)));
            }
            descriptors
        } },
        crc: { 32 }
    }
);
