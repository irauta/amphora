
use ::base::Deserialize;
use bitreader::BitReader;

pub mod psi;

bit_struct!(
    #[derive(Debug,Copy,Clone)]
    pub struct PrivateSectionHeader {
        pub table_id: u8,
        pub private_indicator: u8,
        pub section_length: u16,
        pub extended_header: Option<ExtendedPrivateSectionHeader>
    }
    deserialize(reader) {
        table_id: { 8 },
        section_syntax_indicator: { 1, type: u8 },
        private_indicator: { 1 },
        reserved: { 2 },
        section_length: { 12 },
        extended_header: { value:
            if section_syntax_indicator == 1 {
                Some(try!(Deserialize::deserialize(reader)))
            } else {
                None
            }
        }
    }
);

bit_struct!(
    #[derive(Debug,Copy,Clone)]
    pub struct ExtendedPrivateSectionHeader {
        pub table_id_extension: u16,
        pub version_number: u8,
        pub current_next_indicator: bool,
        pub section_number: u8,
        pub last_section_number: u8
    }
    deserialize(reader) {
        table_id_extension: { 16 },
        reserved: { 2 },
        version_number: { 5 },
        current_next_indicator: { 1, map: |b: u8| b == 1 },
        section_number: { 8 },
        last_section_number: { 8 }
    }
);

fn bits_remaining(section_length: u16, reader: &BitReader) -> u32 {
    // table_id (8 bits) + section_syntax_indicator (1 bit) +
    // private_indicator (1 bit) + reseved (2 bits)
    let intro_bits = 12;
    let crc_length = 32;
    // How many data bits after intro bits - excluding CRC32
    let data_bits = section_length as u32 * 8 - crc_length;
    let total_descriptor_bits = intro_bits + data_bits;
    total_descriptor_bits - reader.position() as u32
}
