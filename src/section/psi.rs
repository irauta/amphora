
//! # Program Specific Information (PSI)

use ::base::*;
use super::bits_remaining;
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
        current_next_indicator: { 1, map: bool_flag },
        section_number: { 8 },
        last_section_number: { 8 },
        associations: { value: {
            let association_size = 32; // How many bits for single program association
            let mut associations = vec![];
            while bits_remaining(section_length, reader) >= association_size {
                associations.push(try!(Deserialize::deserialize(reader)));
            }
            associations
        } },
        skip: { bits_remaining(section_length, reader) },
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
    fn deserialize(original_reader: &mut ::bitreader::BitReader) -> DeserializationResult<Self> {
        let mut reader = original_reader.relative_reader();
        let program_number = try!(reader.read_u16(16));
        try!(reserved(&mut reader, 3));
        let pid = try!(reader.read_u16(13));
        let association = if program_number == 0 {
            ProgramAssociation::NetworkPid(pid)
        } else {
            ProgramAssociation::ProgramMapPid {
                program_number: program_number,
                program_map_pid: pid,
            }
        };
        try!(original_reader.skip(reader.position() as u32));
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
        current_next_indicator: { 1, map: bool_flag },
        section_number: { 8 },
        last_section_number: { 8 },
        descriptors: { value: {
            let mut descriptors = vec![];
            while bits_remaining(section_length, reader) > 0 {
                descriptors.push(try!(deserialize_descriptor(reader)));
            }
            descriptors
        } },
        skip: { bits_remaining(section_length, reader) },
        crc: { 32 }
    }
);


bit_struct!(
    #[derive(Debug)]
    pub struct ElementaryStreamInfo {
        pub stream_type: u8,
        pub elementary_pid: u16,
        pub es_info: Vec<Box<Descriptor>>
    }
    deserialize(reader) {
        stream_type: { 8 },
        reserved: { 3 },
        elementary_pid: { 13 },
        reserved: { 4 },
        es_info_length: { 12, type: u64 },
        es_info: { value: {
            let mut info = vec![];
            let mut bits_remaining = es_info_length * 8;
            while bits_remaining > 0 {
                let start_pos = reader.position();
                info.push(try!(deserialize_descriptor(reader)));
                let bits = reader.position() - start_pos;
                bits_remaining = bits_remaining - bits;
            }
            info
        } }
    }
);

bit_struct!(
    #[derive(Debug)]
    pub struct ProgramMapSection {
        pub program_number: u16,
        pub version_number: u8,
        pub current_next_indicator: bool,
        pub section_number: u8,
        pub last_section_number: u8,
        pub pcr_pid: u16,
        pub descriptors: Vec<Box<Descriptor>>,
        pub programs: Vec<ElementaryStreamInfo>
    }
    deserialize(reader) {
        expect: { bits: 2, reference: 2 }, // table_id
        expect: { bits: 1, reference: 1 }, // section_syntax_indicator
        expect: { bits: 1, reference: 0 }, // 0
        reserved: { 2 },
        section_length: { 12, type: u16 },
        program_number: { 16 },
        reserved: { 2 },
        version_number: { 5 },
        current_next_indicator: { 1, map: bool_flag },
        section_number: { 8 },
        last_section_number: { 8 },
        reserved: { 3 },
        pcr_pid: { 13 },
        reserved: { 4 },
        program_info_length: { 12, type: u64 },
        descriptors: { value: {
            let mut descriptors = vec![];
            let mut bits_remaining = program_info_length * 8;
            while bits_remaining > 0 {
                let start_pos = reader.position();
                descriptors.push(try!(deserialize_descriptor(reader)));
                let bits = reader.position() - start_pos;
                bits_remaining = bits_remaining - bits;
            }
            descriptors
        } },
        programs: { value: {
            let mut programs = vec![];
            while bits_remaining(section_length, reader) > 0 {
                programs.push(try!(Deserialize::deserialize(reader)));
            }
            programs
        }},
        skip: { bits_remaining(section_length, reader) },
        crc: { 32 }
    }
);
