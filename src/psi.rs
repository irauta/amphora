
//! # Program Specific Information (PSI)

use super::base::*;

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
            let fixed_bytes = 12; // How many bytes for header and CRC field
            let association_size = 4; // How many bytes for single program association
            let data_bytes = section_length - fixed_bytes;
            let num_associations = data_bytes / association_size;

            let mut associations = vec![];
            for _ in 0..num_associations {
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
