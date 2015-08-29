
use ::base::bool_flag;
use super::{Descriptor,bits_remaining,repeated_element,repeated_sub_element,read_tla};
use encoding::all::{ISO_8859_1,ISO_8859_2,ISO_8859_3,ISO_8859_4,ISO_8859_5,ISO_8859_6,ISO_8859_7,ISO_8859_8,ISO_8859_10,ISO_8859_13,ISO_8859_14,ISO_8859_15,UTF_16BE,GBK,UTF_8,WINDOWS_949};
use encoding::{Encoding,DecoderTrap};
use bitreader;



// 0x40 NetworkNameDescriptor
// 0x41 ServiceListDescriptor
// 0x42 StuffingDescriptor


// 0x43 SatelliteDeliverySystemDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct SatelliteDeliverySystemDescriptor {
        pub frequency: u32,
        pub orbital_position: u16,
        pub west_east: bool,
        pub polarization: u8,
        pub roll_off: u8,
        pub modulation_system: bool,
        pub modulation_type: u8,
        pub symbol_rate: u32,
        pub fec_inner: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x43 },
        descriptor_length: { 8 },
        frequency: { 32 },
        orbital_position: { 16 },
        west_east: { 1, map: bool_flag },
        polarization: { 2 },
        roll_off_tmp: { 2 },
        modulation_system: { 1, map: bool_flag },
        roll_off: { value: if modulation_system { roll_off_tmp } else { 0 } },
        modulation_type: { 2 },
        symbol_rate: { 28 },
        fec_inner: { 4 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for SatelliteDeliverySystemDescriptor {}


// 0x44 CableDeliverySystemDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct CableDeliverySystemDescriptor {
        pub frequency: u32,
        pub fec_outer: u8,
        pub modulation: u8,
        pub symbol_rate: u32,
        pub fec_inner: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x44 },
        descriptor_length: { 8 },
        frequency: { 32 },
        reserved: { 12 },
        fec_outer: { 4 },
        modulation: { 8 },
        symbol_rate: { 28 },
        fec_inner: { 4 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for CableDeliverySystemDescriptor {}


// 0x45 VbiDataDescriptor
// 0x46 VbiTeletextDescriptor
// 0x47 BouquetNameDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct BouquetNameDescriptor {
        pub name: String
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x47 },
        descriptor_length: { 8 },
        name: { value: try!(remainder_as_string(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for BouquetNameDescriptor {}

// 0x48 ServiceDescriptor
// 0x49 CountryAvailabilityDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct CountryAvailabilityDescriptor {
        pub country_availability: bool,
        pub country_codes: Vec<String>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x49 },
        descriptor_length: { 8 },
        country_availability: { 1, map: bool_flag },
        reserved: { 7 },
        country_codes: { value: {
            let mut codes = vec![];
            while bits_remaining(descriptor_length, reader) >= 24 {
                codes.push(try!(read_tla(reader)));
            }
            codes
        } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for CountryAvailabilityDescriptor {}

// 0x4a LinkageDescriptor
// 0x4b NvodReferenceDescriptor
// 0x4c TimeShiftedServiceDescriptor
// 0x4d ShortEventDescriptor


bit_struct!(
    #[derive(Debug)]
    pub struct ExtendedEventItem {
        pub item_description: String,
        pub item_text: String
    }
    deserialize(reader) {
        item_description_length: { 8, type: u8 },
        item_description: { value: {
            let bytes = try!(repeated_sub_element(item_description_length, reader));
            bytes_to_string(&bytes[..])
        } },
        item_length: { 8, type: u8 },
        item_text: { value: {
            let bytes = try!(repeated_sub_element(item_length, reader));
            bytes_to_string(&bytes[..])
        } }
    }
);

// 0x4e ExtendedEventDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ExtendedEventDescriptor {
        pub descriptor_number: u8,
        pub last_descriptor_number: u8,
        pub iso_639_language_code: String,
        pub items: Vec<ExtendedEventItem>,
        pub text: String
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x4e },
        descriptor_length: { 8 },
        descriptor_number: { 4 },
        last_descriptor_number: { 4 },
        iso_639_language_code: { value: try!(read_tla(reader)) },
        length_of_items: { 8 },
        items: { value: try!(repeated_sub_element(length_of_items, reader)) },
        text: { value: try!(remainder_as_string(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ExtendedEventDescriptor {}


// 0x4f TimeShiftedEventDescriptor

// 0x50 ComponentDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ComponentDescriptor {
        pub stream_content: u8,
        pub component_type: u8,
        pub component_tag: u8,
        pub iso_639_language_code: String,
        pub description: String
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x50 },
        descriptor_length: { 8 },
        reserved: { 4 },
        stream_content: { 4 },
        component_type: { 8 },
        component_tag: { 8 },
        iso_639_language_code: { value: try!(read_tla(reader)) },
        description: { value: try!(remainder_as_string(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ComponentDescriptor {}

// 0x51 MosaicDescriptor
// 0x52 StreamIdentifierDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct StreamIdentifierDescriptor {
        pub component_tag: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x52 },
        descriptor_length: { 8 },
        component_tag: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for StreamIdentifierDescriptor {}

// 0x53 CaIdentifierDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct CaIdentifierDescriptor {
        pub ca_system_ids: Vec<u16>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x53 },
        descriptor_length: { 8 },
        ca_system_ids: { value: {
            let mut ids = vec![];
            while bits_remaining(descriptor_length, reader) >= 16 {
                ids.push(try!(reader.read_u16(16)));
            }
            ids
        } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for CaIdentifierDescriptor {}

bit_struct!(
    #[derive(Debug)]
    pub struct ContentIdentifier {
        pub content_nibble_level_1: u8,
        pub content_nibble_level_2: u8,
        pub user_byte: u8
    }
    deserialize(reader) {
        content_nibble_level_1: { 4 },
        content_nibble_level_2: { 4 },
        user_byte: { 8 }
    }
);

// 0x54 ContentDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ContentDescriptor {
        pub content_idenfiers: Vec<ContentIdentifier>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x54 },
        descriptor_length: { 8 },
        content_idenfiers: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ContentDescriptor {}

// 0x55 ParentalRatingDescriptor
// 0x56 TeletextDescriptor
// 0x57 TelephoneDescriptor
// 0x58 LocalTimeOffsetDescriptor
// 0x59 SubtitlingDescriptor


// 0x5a TerrestrialDeliverySystemDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct TerrestrialDeliverySystemDescriptor {
        pub centre_frequency: u32,
        pub bandwidth: u8,
        pub priority: bool,
        pub time_slicing_indicator: bool,
        pub mpe_fec_indicator: bool,
        pub constellation: u8,
        pub hierarchy_information: u8,
        pub code_rate_hp_stream: u8,
        pub code_rate_lp_stream: u8,
        pub guard_interval: u8,
        pub transmission_mode: u8,
        pub other_frequency_flag: bool
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x5a },
        descriptor_length: { 8 },
        centre_frequency: { 32 },
        bandwidth: { 3 },
        priority: { 1, map: bool_flag },
        time_slicing_indicator: { 1, map: bool_flag },
        mpe_fec_indicator: { 1, map: bool_flag },
        reserved: { 2 },
        constellation: { 2 },
        hierarchy_information: { 3 },
        code_rate_hp_stream: { 3 },
        code_rate_lp_stream: { 3 },
        guard_interval: { 2 },
        transmission_mode: { 2 },
        other_frequency_flag: { 1, map: bool_flag },
        reserved: { 32 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TerrestrialDeliverySystemDescriptor {}


// 0x5b MultilingualNetworkNameDescriptor
// 0x5c MultilingualBouquetNameDescriptor
// 0x5d MultilingualServiceNameDescriptor
// 0x5e MultilingualComponentDescriptor
// 0x5f PrivateDataSpecifierDescriptor
// 0x60 ServiceMoveDescriptor
// 0x61 ShortSmoothingBufferDescriptor
// 0x62 FrequencyListDescriptor
// 0x63 PartialTransportStreamDescriptor


// 0x64 DataBroadcastDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct DataBroadcastDescriptor {
        pub data_broadcast_id: u16,
        pub component_tag: u8,
        pub selector_bytes: Vec<u8>,
        pub iso_639_language_code: String,
        pub text: String
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x64 },
        descriptor_length: { 8 },
        data_broadcast_id: { 16 },
        component_tag: { 8 },
        selector_length: { 8 },
        selector_bytes: { value: try!(repeated_sub_element(selector_length, reader)) },
        iso_639_language_code: { value: try!(read_tla(reader)) },
        text: { value: try!(remainder_as_string(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for DataBroadcastDescriptor {}


// 0x65 CaSystemDescriptor


// 0x66 DataBroadcastIdDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct DataBroadcastIdDescriptor {
        pub data_broadcast_id: u16,
        pub selector_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x64 },
        descriptor_length: { 8 },
        data_broadcast_id: { 16 },
        selector_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for DataBroadcastIdDescriptor {}


// 0x67 TransportStreamDescriptor


// 0x68 DsngDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct DsngDescriptor {
        pub bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x68 },
        descriptor_length: { 8 },
        bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for DsngDescriptor {}


// 0x69 PdcDescriptor
// 0x6a Ac3Descriptor
// 0x6b AncillaryDataDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct AncillaryDataDescriptor {
        pub data: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x6b },
        descriptor_length: { 8 },
        data: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for AncillaryDataDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct SubcellInfo {
        pub cell_id_extension: u8,
        pub subcell_latitude: u16,
        pub subcell_longitude: u16,
        pub subcell_extent_of_latitude: u16,
        pub subcell_extent_of_longitude: u16
    }
    deserialize(reader) {
        cell_id_extension: { 8 },
        subcell_latitude: { 16 },
        subcell_longitude: { 16 },
        subcell_extent_of_latitude: { 12 },
        subcell_extent_of_longitude: { 12 }
    }
);

bit_struct!(
    #[derive(Debug)]
    pub struct CellInfo {
        pub cell_id: u16,
        pub cell_latitude: u16,
        pub cell_longitude: u16,
        pub cell_extent_of_latitude: u16,
        pub cell_extent_of_longitude: u16,
        pub subcells: Vec<SubcellInfo>
    }
    deserialize(reader) {
        cell_id: { 16 },
        cell_latitude: { 16 },
        cell_longitude: { 16 },
        cell_extent_of_latitude: { 12 },
        cell_extent_of_longitude: { 12 },
        subcell_info_loop_length: { 8, type: u8 },
        subcells: { value: try!(repeated_sub_element(subcell_info_loop_length, reader)) }
    }
);

// 0x6c CellListDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct CellListDescriptor {
        pub cells: Vec<CellInfo>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x6c },
        descriptor_length: { 8 },
        cells: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for CellListDescriptor {}

bit_struct!(
    #[derive(Debug)]
    pub struct SubcellFrequencyInfo {
        pub cell_id_extension: u8,
        pub transposer_frequency: u32
    }
    deserialize(reader) {
        cell_id_extension: { 8 },
        transposer_frequency: { 32 }
    }
);

bit_struct!(
    #[derive(Debug)]
    pub struct CellFrequencyInfo {
        pub cell_id: u16,
        pub frequency: u32,
        pub subcells: Vec<SubcellFrequencyInfo>
    }
    deserialize(reader) {
        cell_id: { 16 },
        frequency: { 32 },
        subcell_info_loop_length: { 8, type: u8 },
        subcells: { value: try!(repeated_sub_element(subcell_info_loop_length, reader)) }
    }
);

// 0x6d CellFrequencyLinkDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct CellFrequencyLinkDescriptor {
        pub cells: Vec<CellFrequencyInfo>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x6d },
        descriptor_length: { 8 },
        cells: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for CellFrequencyLinkDescriptor {}


#[derive(Debug)]
pub struct AnnouncementService {
        pub original_network_id: u16,
        pub transport_stream_id: u16,
        pub service_id: u16,
        pub component_tag: u8,
}

bit_struct!(
    #[derive(Debug)]
    pub struct AnnouncementInfo {
        pub announcement_type: u8,
        pub reference_type: u8,
        pub service: Option<AnnouncementService>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x6e },
        descriptor_length: { 8 },
        announcement_type: { 4 },
        reserved: { 1 },
        reference_type: { 3 },
        service: { value: {
            if reference_type == 1 || reference_type == 2 || reference_type == 3 {
                Some(AnnouncementService {
                    original_network_id: try!(reader.read_u16(16)),
                    transport_stream_id: try!(reader.read_u16(16)),
                    service_id: try!(reader.read_u16(16)),
                    component_tag: try!(reader.read_u8(8)),
                })
            } else {
                None
            }
        } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);

// 0x6e AnnouncementSupportDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct AnnouncementSupportDescriptor {
        pub announcement_support: u16,
        pub services: Vec<AnnouncementInfo>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x6e },
        descriptor_length: { 8 },
        announcement_support: { 16 },
        services: { value: { try!(repeated_element(descriptor_length, reader)) } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for AnnouncementSupportDescriptor {}

// 0x6f ApplicationSignallingDescriptor
// 0x70 AdaptationFieldDataDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct AdaptationFieldDataDescriptor {
        pub data: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x70 },
        descriptor_length: { 8 },
        data: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for AdaptationFieldDataDescriptor {}

// 0x71 ServiceIdentifierDescriptor
// 0x72 ServiceAvailabilityDescriptor
// 0x73 DefaultAuthorityDescriptor
// 0x74 RelatedContentDescriptor
// 0x75 TvaIdDescriptor
// 0x76 ContentIdentifierDescriptor
// 0x77 TimeSliceFecIdentifierDescriptor
// 0x78 EcmRepetitionRateDescriptor


// 0x79 S2SatelliteDeliverySystemDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct S2SatelliteDeliverySystemDescriptor {
        pub scrambling_sequence_selector: bool,
        pub multiple_input_stream: bool,
        pub backwards_compatibility_indicator: bool,
        pub scrambling_sequence_index: Option<u32>,
        pub input_stream_identifier: Option<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x79 },
        descriptor_length: { 8 },
        scrambling_sequence_selector: { 1, map: bool_flag },
        multiple_input_stream: { 1, map: bool_flag },
        backwards_compatibility_indicator: { 1, map: bool_flag },
        reserved: { 5 },
        scrambling_sequence_index: { value: if scrambling_sequence_selector {
            try!(::base::reserved(reader, 6));
            Some(try!(reader.read_u32(18)))
        } else { None } },
        input_stream_identifier: { value: if multiple_input_stream {
            Some(try!(reader.read_u8(8)))
        } else { None } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for S2SatelliteDeliverySystemDescriptor {}


// 0x7a EnhancedAc3Descriptor
// 0x7b DtsDescriptor
// 0x7c AacDescriptor
// 0x7d XaitDescriptor
// 0x7e FtaDescriptor


// 0x7f ExtensionDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ExtensionDescriptor {
        pub descriptor_tag_extension: u8,
        pub selector_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x7f },
        descriptor_length: { 8 },
        descriptor_tag_extension: { 8 },
        selector_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ExtensionDescriptor {}



fn remainder_as_string(descriptor_length: u8, reader: &mut bitreader::BitReader) -> bitreader::Result<String> {
    let mut bytes = vec![];
    while bits_remaining(descriptor_length, reader) >= 8 {
        bytes.push(try!(reader.read_u8(8)));
    }
    Ok(bytes_to_string(&bytes[..]))
}

fn bytes_to_string(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    let mut name = String::new();
    match bytes[0] {
        0x20...0xff => { ::encodings::iso_6937(bytes, &mut name); true },
        0x1 => ISO_8859_5.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        0x2 => ISO_8859_6.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        0x3 => ISO_8859_7.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        0x4 => ISO_8859_8.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        0x5 => { ::encodings::iso_8859_9(&bytes[1..], &mut name); true },
        0x6 => ISO_8859_10.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        //0x7 => ISO_8859_11.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        0x9 => ISO_8859_13.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        0xa => ISO_8859_14.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        0xb => ISO_8859_15.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        0x10 => {
            if bytes[1] == 0 {
                match bytes[2] {
                    0x1 => ISO_8859_1.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0x2 => ISO_8859_2.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0x3 => ISO_8859_3.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0x4 => ISO_8859_4.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0x5 => ISO_8859_5.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0x6 => ISO_8859_6.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0x7 => ISO_8859_7.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0x8 => ISO_8859_8.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0x9 => { ::encodings::iso_8859_9(&bytes[3..], &mut name); true },
                    0xa => ISO_8859_10.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    //0xb => ISO_8859_11.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    //0xc => "reserved for future use",
                    0xd => ISO_8859_13.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0xe => ISO_8859_14.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    0xf => ISO_8859_15.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
                    _ => false
                };
            }
            true
        }
        // Approximation of UCS
        0x11 => UTF_16BE.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        // KS X 1001 is possibly encoded as EUC-KR/WINDOWS-949
        0x12 => WINDOWS_949.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        // GBK is compatible with GB-2312
        0x13 => GBK.decode_to(&bytes[3..], DecoderTrap::Ignore, &mut name).is_ok(),
        // Big5 subset of ISO/IEC 10646 -> UCS
        0x14 => UTF_16BE.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        // Approximating again - assuming UCS and Unicode to be compatible
        0x15 => UTF_8.decode_to(&bytes[1..], DecoderTrap::Ignore, &mut name).is_ok(),
        _ => false
    };
    name
}
