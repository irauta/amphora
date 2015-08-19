
use super::base::Deserialize;

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

/// Takes the section length and a BitReader and tells how many bytes there are left
/// in the section, the four CRC32 bytes excluded.
///
/// The result is rounded down if the reader is not at byte-aligned position.
///
/// The reader must have its position relative to the start of the section!
/// That is, its position() method must return 0 right before the table_id byte was read from the
/// beginning of the section. (This note is mostly relevant when there are several sections in
/// row or similar in a single byte buffer.)
pub fn section_bytes_left(section_length: u16, reader: &::bitreader::BitReader) -> u16 {
    // table_id(8 bits) + section_syntax_indicator(1 bit) + 1 bit + 2 reserved bits
    // + section_length(12 bits) = 24 bits = 3 bytes
    let header_length = 3;
    // Length of the CRC32 field at the end of section
    let crc_length = 4;
    header_length + section_length - crc_length - (reader.position() / 8) as u16
}
