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

use ::base::{Deserialize,bool_flag};
use super::{Descriptor,bits_remaining,repeated_element,repeated_sub_element,read_tla};
use encoding::all::{ISO_8859_1,ISO_8859_2,ISO_8859_3,ISO_8859_4,ISO_8859_5,ISO_8859_6,ISO_8859_7,ISO_8859_8,ISO_8859_10,ISO_8859_13,ISO_8859_14,ISO_8859_15,UTF_16BE,GBK,UTF_8,WINDOWS_949};
use encoding::{Encoding,DecoderTrap};
use bitreader;



// 0x40 NetworkNameDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct NetworkNameDescriptor {
        pub name: String
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x44 },
        descriptor_length: { 8 },
        name: { value: try!(remainder_as_string(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for NetworkNameDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct Service {
        pub service_id: u16,
        pub service_type: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x44 },
        descriptor_length: { 8 },
        service_id: { 16 },
        service_type: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);

// 0x41 ServiceListDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ServiceListDescriptor {
        pub services: Vec<Service>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x41 },
        descriptor_length: { 8 },
        services: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ServiceListDescriptor {}


// 0x42 StuffingDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct StuffingDescriptor {
        // Hacky solution here, might want to impl Deserialize manually for this
        pub void: ()
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x42 },
        descriptor_length: { 8 },
        void: { value: () },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for StuffingDescriptor {}


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


bit_struct!(
    #[derive(Debug)]
    pub struct VbiDataLine {
        pub field_parity: u8,
        pub line_offset: u8
    }
    deserialize(reader) {
        reserved: { 2 },
        field_parity: { 1 },
        line_offset: { 5 }
    }
);

bit_struct!(
    #[derive(Debug)]
    pub struct VbiDataService {
        pub data_service_id: u8,
        pub vbi_data_lines: Vec<VbiDataLine>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x44 },
        descriptor_length: { 8 },
        data_service_id: { 8 },
        data_service_descriptor_length: { 8 },
        vbi_data_lines: { value: if data_service_id >= 1 && data_service_id <= 7 {
            try!(repeated_sub_element(data_service_descriptor_length, reader))
        } else { vec![] } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);

// 0x45 VbiDataDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct VbiDataDescriptor {
        pub vbi_services: Vec<VbiDataService>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x45 },
        descriptor_length: { 8 },
        vbi_services: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for VbiDataDescriptor {}


// 0x46 VbiTeletextDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct VbiTeletextDescriptor {
        pub vbi_teletext_pages: Vec<TeletextPage>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x46 },
        descriptor_length: { 8 },
        vbi_teletext_pages: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for VbiTeletextDescriptor {}


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
bit_struct!(
    #[derive(Debug)]
    pub struct ServiceDescriptor {
        pub service_type: u8,
        pub service_provider_name: String,
        pub service_name: String
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x48 },
        descriptor_length: { 8 },
        service_type: { 8 },
        service_provider_name_length: { 8 },
        service_provider_name: { value: try!(read_string(service_provider_name_length, reader)) },
        service_name_length: { 8 },
        service_name: { value: try!(read_string(service_name_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ServiceDescriptor {}


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


bit_struct!(
    #[derive(Debug)]
    pub struct MobileHandover {
        pub hand_over_type: u8,
        pub origin_type: u8,
        pub network_id: Option<u16>,
        pub initial_service_id: Option<u16>
    }
    deserialize(reader) {
        hand_over_type: { 4 },
        reserved: { 3 },
        origin_type: { 1 },
        network_id: { value: if hand_over_type == 1 || hand_over_type == 2 || hand_over_type == 3 {
            Some(try!(reader.read_u16(16)))
        } else { None } },
        initial_service_id: { value: if origin_type == 0 {
            Some(try!(reader.read_u16(16)))
        } else { None } }
    }
);

bit_struct!(
    #[derive(Debug)]
    pub struct EventLinkage {
        pub target_event_id: u16,
        pub target_listed: bool,
        pub event_simulcast: bool
    }
    deserialize(reader) {
        target_event_id: { 16 },
        target_listed: { 1, map: bool_flag },
        event_simulcast: { 1, map: bool_flag },
        reserved: { 6 }
    }
);

// 0x4a LinkageDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct LinkageDescriptor {
        pub transport_stream_id: u16,
        pub original_network_id: u16,
        pub service_id: u16,
        pub linkage_type: u8,
        pub mobile_handover: Option<MobileHandover>,
        pub event_linkage: Option<EventLinkage>,
        pub data: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x4a },
        descriptor_length: { 8 },
        transport_stream_id: { 16 },
        original_network_id: { 16 },
        service_id: { 16 },
        linkage_type: { 8 },
        mobile_handover: { value: if linkage_type == 8 {
            Some(try!(Deserialize::deserialize(reader)))
        } else { None } },
        event_linkage: { value: if linkage_type == 0x0d {
            Some(try!(Deserialize::deserialize(reader)))
        } else { None } },
        data: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for LinkageDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct NvodReference {
        pub transport_stream_id: u16,
        pub original_network_id: u16,
        pub service_id: u16
    }
    deserialize(reader) {
        transport_stream_id: { 16 },
        original_network_id: { 16 },
        service_id: { 16 }
    }
);

// 0x4b NvodReferenceDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct NvodReferenceDescriptor {
        pub nvod_references: Vec<NvodReference>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x4b },
        descriptor_length: { 8 },
        nvod_references: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for NvodReferenceDescriptor {}


// 0x4c TimeShiftedServiceDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct TimeShiftedServiceDescriptor {
        pub reference_service_id: u16
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x4c },
        descriptor_length: { 8 },
        reference_service_id: { 16 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TimeShiftedServiceDescriptor {}


// 0x4d ShortEventDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ShortEventDescriptor {
        pub iso_639_language_code: String,
        pub event_name: String,
        pub text: String
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x4d },
        descriptor_length: { 8 },
        iso_639_language_code: { value: try!(read_tla(reader)) },
        event_name_length: { 8 },
        event_name: { value: try!(read_string(event_name_length, reader)) },
        text_length: { 8 },
        text: { value: try!(read_string(text_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ShortEventDescriptor {}


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
bit_struct!(
    #[derive(Debug)]
    pub struct TimeShiftedEventDescriptor {
        pub reference_service_id: u16,
        pub reference_event_id: u16
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x4f },
        descriptor_length: { 8 },
        reference_service_id: { 16 },
        reference_event_id: { 16 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TimeShiftedEventDescriptor {}


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


#[derive(Debug)]
pub enum MosaicCellLinkage {
    Undefined,
    BouquetRelated {
        bouquet_id: u16,
    },
    ServiceRelated {
        original_network_id: u16,
        transport_stream_id: u16,
        service_id: u16,
    },
    OtherMosaicRelated {
        original_network_id: u16,
        transport_stream_id: u16,
        service_id: u16,
    },
    EventRelated {
        original_network_id: u16,
        transport_stream_id: u16,
        service_id: u16,
        event_id: u16,
    },
    Unrecognized(u8)
}
fn read_mosaic_cell_linkage(cell_linkage_info: u8, reader: &mut bitreader::BitReader) -> bitreader::Result<MosaicCellLinkage> {
    let mut r16 = || reader.read_u16(16);
    let linkage = match cell_linkage_info {
        0 => MosaicCellLinkage::Undefined,
        1 => MosaicCellLinkage::BouquetRelated {
            bouquet_id: try!(r16()),
        },
        2 => MosaicCellLinkage::ServiceRelated {
            original_network_id: try!(r16()),
            transport_stream_id: try!(r16()),
            service_id: try!(r16()),
        },
        3 => MosaicCellLinkage::OtherMosaicRelated {
            original_network_id: try!(r16()),
            transport_stream_id: try!(r16()),
            service_id: try!(r16()),
        },
        4 => MosaicCellLinkage::EventRelated {
            original_network_id: try!(r16()),
            transport_stream_id: try!(r16()),
            service_id: try!(r16()),
            event_id: try!(r16())
        },
        _ => MosaicCellLinkage::Unrecognized(cell_linkage_info),
    };
    Ok(linkage)
}

bit_struct!(
    #[derive(Debug)]
    pub struct MosaicElementaryCell {
        pub logical_cell_id: u8,
        pub logical_cell_presentation_info: u8,
        pub elementary_cell_ids: Vec<u8>,
        pub cell_linkage: MosaicCellLinkage
    }
    deserialize(reader) {
        logical_cell_id: { 8 },
        reserved: { 1 },
        logical_cell_presentation_info: { 3 },
        elementary_cell_lenght_field: { 8 },
        elementary_cell_ids: { value: {
            let mut ids = vec![];
            for _ in 0..elementary_cell_lenght_field {
                try!(::base::reserved(reader, 2));
                ids.push(try!(reader.read_u8(6)));
            }
            ids
        } },
        cell_linkage_info: { 8 },
        cell_linkage: { value: try!(read_mosaic_cell_linkage(cell_linkage_info, reader)) }
    }
);

// 0x51 MosaicDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct MosaicDescriptor {
        pub mosaic_entry_point: bool,
        pub number_of_horizontal_elementary_cells: u8,
        pub number_of_vertical_elementary_cells: u8,
        pub logical_cells: Vec<MosaicElementaryCell>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x51 },
        descriptor_length: { 8 },
        mosaic_entry_point: { 1, map: bool_flag },
        number_of_horizontal_elementary_cells: { 3 },
        reserved: { 1 },
        number_of_vertical_elementary_cells: { 3 },
        logical_cells: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MosaicDescriptor {}


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


bit_struct!(
    #[derive(Debug)]
    pub struct ParentalRating {
        pub country_code: String,
        pub rating: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x54 },
        descriptor_length: { 8 },
        country_code: { value: try!(read_tla(reader)) },
        rating: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);

// 0x55 ParentalRatingDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ParentalRatingDescriptor {
        pub ratings: Vec<ParentalRating>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x55 },
        descriptor_length: { 8 },
        ratings: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ParentalRatingDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct TeletextPage {
        pub iso_639_language_code: String,
        pub teletext_type: u8,
        pub teletext_magazine_number: u8,
        pub teletext_page_number: u8
    }
    deserialize(reader) {
        iso_639_language_code: { value: try!(read_tla(reader)) },
        teletext_type: { 5 },
        teletext_magazine_number: { 3 },
        teletext_page_number: { 8 }
    }
);

// 0x56 TeletextDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct TeletextDescriptor {
        pub teletext_pages: Vec<TeletextPage>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x56 },
        descriptor_length: { 8 },
        teletext_pages: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TeletextDescriptor {}


// 0x57 TelephoneDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct TelephoneDescriptor {
        pub foreign_availability: bool,
        pub connection_type: u8,
        pub country_prefix: String,
        pub international_area_code: String,
        pub operator_code: String,
        pub national_area_code: String,
        pub core_number: String
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x57 },
        descriptor_length: { 8 },
        reserved: { 2 },
        foreign_availability: { 1, map: bool_flag },
        connection_type: { 5 },
        reserved: { 1 },
        country_prefix_length: { 2 },
        international_area_code_length: { 3 },
        operator_code_length: { 2 },
        reserved: { 1 },
        national_area_code_length: { 3 },
        core_number_length: { 4 },
        country_prefix: { value: try!(read_string_latin1(country_prefix_length, reader)) },
        international_area_code: { value: try!(read_string_latin1(international_area_code_length, reader)) },
        operator_code: { value: try!(read_string_latin1(operator_code_length, reader)) },
        national_area_code: { value: try!(read_string_latin1(national_area_code_length, reader)) },
        core_number: { value: try!(read_string_latin1(core_number_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TelephoneDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct LocalTimeOffset {
        pub country_code: String,
        pub country_region_id: u8,
        pub local_time_offset_polarity: u8,
        pub local_time_offset: u16,
        pub time_of_change: u64,
        pub next_time_offset: u16
    }
    deserialize(reader) {
        country_code: { value: try!(read_tla(reader)) },
        country_region_id: { 6 },
        reserved: { 1 },
        local_time_offset_polarity: { 1 },
        local_time_offset: { 16 },
        time_of_change: { 40 },
        next_time_offset: { 16 }
    }
);

// 0x58 LocalTimeOffsetDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct LocalTimeOffsetDescriptor {
        pub offsets: Vec<LocalTimeOffset>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x58 },
        descriptor_length: { 8 },
        offsets: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for LocalTimeOffsetDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct Subtitling {
        pub iso_639_language_code: String,
        pub subtitling_page: u8,
        pub composition_page_id: u16,
        pub ancillary_page_id: u16
    }
    deserialize(reader) {
        iso_639_language_code: { value: try!(read_tla(reader)) },
        subtitling_page: { 8 },
        composition_page_id: { 16 },
        ancillary_page_id: { 16 }
    }
);

// 0x59 SubtitlingDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct SubtitlingDescriptor {
        pub subtitles: Vec<Subtitling>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x59 },
        descriptor_length: { 8 },
        subtitles: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for SubtitlingDescriptor {}


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



bit_struct!(
    #[derive(Debug)]
    pub struct LocalizedText {
        pub iso_639_language_code: String,
        pub text: String
    }
    deserialize(reader) {
        iso_639_language_code: { value: try!(read_tla(reader)) },
        text_length: { 8 },
        text: { value: try!(read_string(text_length, reader)) }
    }
);

// 0x5b MultilingualNetworkNameDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct MultilingualNetworkNameDescriptor {
        pub network_names: Vec<LocalizedText>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x5b },
        descriptor_length: { 8 },
        network_names: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MultilingualNetworkNameDescriptor {}


// 0x5c MultilingualBouquetNameDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct MultilingualBouquetNameDescriptor {
        pub bouquet_names: Vec<LocalizedText>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x5c },
        descriptor_length: { 8 },
        bouquet_names: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MultilingualBouquetNameDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct MultilingualServiceName {
        pub iso_639_language_code: String,
        pub service_provider_name: String,
        pub service_name: String
    }
    deserialize(reader) {
        iso_639_language_code: { value: try!(read_tla(reader)) },
        service_provider_name_length: { 8 },
        service_provider_name: { value: try!(read_string(service_provider_name_length, reader)) },
        service_name_length: { 8 },
        service_name: { value: try!(read_string(service_name_length, reader)) }
    }
);

// 0x5d MultilingualServiceNameDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct MultilingualServiceNameDescriptor {
        pub service_names: Vec<LocalizedText>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x5d },
        descriptor_length: { 8 },
        service_names: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MultilingualServiceNameDescriptor {}


// 0x5e MultilingualComponentDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct MultilingualComponentDescriptor {
        pub component_tag: u8,
        pub text_descriptions: Vec<LocalizedText>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x5e },
        descriptor_length: { 8 },
        component_tag: { 8 },
        text_descriptions: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MultilingualComponentDescriptor {}


// 0x5f PrivateDataSpecifierDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct PrivateDataSpecifierDescriptor {
        pub private_data_specifier: u32
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x5f },
        descriptor_length: { 8 },
        private_data_specifier: { 32 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for PrivateDataSpecifierDescriptor {}


// 0x60 ServiceMoveDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ServiceMoveDescriptor {
        pub new_original_network_id: u16,
        pub new_transport_stream_id: u16,
        pub new_service_id: u16
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x60 },
        descriptor_length: { 8 },
        new_original_network_id: { 16 },
        new_transport_stream_id: { 16 },
        new_service_id: { 16 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ServiceMoveDescriptor {}


// 0x61 ShortSmoothingBufferDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ShortSmoothingBufferDescriptor {
        pub sb_size: u8,
        pub sb_leak_rate: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x60 },
        descriptor_length: { 8 },
        sb_size: { 2 },
        sb_leak_rate: { 6 },
        // The spec lists N reserved bytes here explicitly, we will just rely on the default skip here:
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ShortSmoothingBufferDescriptor {}


// 0x62 FrequencyListDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct FrequencyListDescriptor {
        pub coding_type: u8,
        pub centre_frequencies: Vec<u32>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x62 },
        descriptor_length: { 8 },
        reserved: { 6 },
        coding_type: { 2 },
        centre_frequencies: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for FrequencyListDescriptor {}


// 0x63 PartialTransportStreamDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct PartialTransportStreamDescriptor {
        pub peak_rate: u32,
        pub minimum_overall_smoothing_rate: u32,
        pub maximum_overall_smoothing_rate: u16
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x63 },
        descriptor_length: { 8 },
        reserved: { 2 },
        peak_rate: { 22 },
        reserved: { 2 },
        minimum_overall_smoothing_rate: { 22 },
        reserved: { 2 },
        maximum_overall_smoothing_rate: { 14 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for PartialTransportStreamDescriptor {}


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


// 0x65 ScramblingDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ScramblingDescriptor {
        pub scrambling_mode: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x65 },
        descriptor_length: { 8 },
        scrambling_mode: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ScramblingDescriptor {}


// 0x66 DataBroadcastIdDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct DataBroadcastIdDescriptor {
        pub data_broadcast_id: u16,
        pub selector_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x66 },
        descriptor_length: { 8 },
        data_broadcast_id: { 16 },
        selector_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for DataBroadcastIdDescriptor {}


// 0x67 TransportStreamDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct TransportStreamDescriptor {
        pub bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x67 },
        descriptor_length: { 8 },
        bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TransportStreamDescriptor {}


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
bit_struct!(
    #[derive(Debug)]
    pub struct PdcDescriptor {
        pub programme_identification_label: u32
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x69 },
        descriptor_length: { 8 },
        reserved: { 4 },
        programme_identification_label: { 20 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for PdcDescriptor {}


// 0x6a Ac3Descriptor
bit_struct!(
    #[derive(Debug)]
    pub struct Ac3Descriptor {
        pub component_type: Option<u8>,
        pub bsid: Option<u8>,
        pub mainid: Option<u8>,
        pub asvc: Option<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x6a },
        descriptor_length: { 8 },
        component_type_flag: { 1, type: u8 },
        bsid_flag: { 1, type: u8 },
        mainid_flag: { 1, type: u8 },
        asvc_flag: { 1, type: u8 },
        reserved: { 4 },
        component_type: { value: if component_type_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        bsid: { value: if bsid_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        mainid: { value: if mainid_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        asvc: { value: if asvc_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for Ac3Descriptor {}


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


bit_struct!(
    #[derive(Debug)]
    pub struct ApplicationVersionInfo {
        pub application_type: u16,
        pub ait_version_number: u8
    }
    deserialize(reader) {
        reserved: { 1 },
        application_type: { 15 },
        reserved: { 3 },
        ait_version_number: { 5 }
    }
);

// 0x6f ApplicationSignallingDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ApplicationSignallingDescriptor {
        pub application_versions: Vec<ApplicationVersionInfo>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x6f },
        descriptor_length: { 8 },
        application_versions: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ApplicationSignallingDescriptor {}


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
bit_struct!(
    #[derive(Debug)]
    pub struct ServiceIdentifierDescriptor {
        pub textual_service_identifier_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x71 },
        descriptor_length: { 8 },
        textual_service_identifier_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ServiceIdentifierDescriptor {}


// 0x72 ServiceAvailabilityDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ServiceAvailabilityDescriptor {
        pub available: bool,
        pub cell_ids: Vec<u16>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x70 },
        descriptor_length: { 8 },
        available: { 1, map: bool_flag },
        reserved: { 7 },
        cell_ids: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ServiceAvailabilityDescriptor {}


// 0x73 DefaultAuthorityDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct DefaultAuthorityDescriptor {
        pub default_authority_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x73 },
        descriptor_length: { 8 },
        default_authority_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for DefaultAuthorityDescriptor {}


// 0x74 RelatedContentDescriptor
// Huh? The TS 102 323 V1.5.1 doesn't define any other fields than descriptor_tag
// and descriptor_length?
bit_struct!(
    #[derive(Debug)]
    pub struct RelatedContentDescriptor {
        // Hacky solution here, might want to impl Deserialize manually for this
        pub void: ()
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x74 },
        descriptor_length: { 8 },
        void: { value: () },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for RelatedContentDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct TvaId {
        pub tva_id: u16,
        pub running_status: u8
    }
    deserialize(reader) {
        tva_id: { 16 },
        reserved: { 5 },
        running_status: { 3 }
    }
);

// 0x75 TvaIdDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct TvaIdDescriptor {
        pub tva_ids: Vec<TvaId>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x75 },
        descriptor_length: { 8 },
        tva_ids: {  value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TvaIdDescriptor {}


#[derive(Debug)]
pub enum Crid {
    Explicit(Vec<u8>),
    Reference(u16),
    UnrecognizedCridType(u8)
}

bit_struct!(
    #[derive(Debug)]
    pub struct TypedCrid {
        pub crid_type: u8,
        pub crid: Crid
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x76 },
        descriptor_length: { 8 },
        crid_type: { 6 },
        crid_location: { 2, type: u8 },
        crid: { value: match crid_location {
            0 => Crid::Explicit({
                let crid_length = try!(reader.read_u8(8));
                try!(repeated_sub_element(crid_length, reader))
            }),
            1 => Crid::Reference(try!(reader.read_u16(16))),
            other => Crid::UnrecognizedCridType(other)
        } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);

// 0x76 ContentIdentifierDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct ContentIdentifierDescriptor {
        pub crids: Vec<TypedCrid>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x76 },
        descriptor_length: { 8 },
        crids: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ContentIdentifierDescriptor {}


// 0x77 TimeSliceFecIdentifierDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct TimeSliceFecIdentifierDescriptor {
        pub time_slicing: bool,
        pub mpe_fec: u8,
        pub frame_size: u8,
        pub max_burst_duration: u8,
        pub max_average_rate: u8,
        pub time_slice_fec_id: u8,
        pub id_selector_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x77 },
        descriptor_length: { 8 },
        time_slicing: { 1, map: bool_flag },
        mpe_fec: { 2 },
        reserved: { 2 },
        frame_size: { 3 },
        max_burst_duration: { 8 },
        max_average_rate: { 4 },
        time_slice_fec_id: { 4 },
        id_selector_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TimeSliceFecIdentifierDescriptor {}


// 0x78 EcmRepetitionRateDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct EcmRepetitionRateDescriptor {
        pub ca_system_id: u16,
        pub ecm_repetition_rate: u16,
        pub private_data_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x78 },
        descriptor_length: { 8 },
        ca_system_id: { 16 },
        ecm_repetition_rate: { 16 },
        private_data_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for EcmRepetitionRateDescriptor {}


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
bit_struct!(
    #[derive(Debug)]
    pub struct EnhancedAc3Descriptor {
        pub component_type: Option<u8>,
        pub bsid: Option<u8>,
        pub mainid: Option<u8>,
        pub asvc: Option<u8>,
        pub mix_info_exists: bool,
        pub substream_1: Option<u8>,
        pub substream_2: Option<u8>,
        pub substream_3: Option<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x7a },
        descriptor_length: { 8 },
        component_type_flag: { 1, type: u8 },
        bsid_flag: { 1, type: u8 },
        mainid_flag: { 1, type: u8 },
        asvc_flag: { 1, type: u8 },
        mix_info_exists: { 1, map: bool_flag },
        substream_1_flag: { 1, type: u8 },
        substream_2_flag: { 1, type: u8 },
        substream_3_flag: { 1, type: u8 },
        component_type: { value: if component_type_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        bsid: { value: if bsid_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        mainid: { value: if mainid_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        asvc: { value: if asvc_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        substream_1: { value: if substream_1_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        substream_2: { value: if substream_2_flag== 1 { Some(try!(reader.read_u8(8))) } else { None } },
        substream_3: { value: if substream_3_flag == 1 { Some(try!(reader.read_u8(8))) } else { None } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for EnhancedAc3Descriptor {}


// 0x7b DtsDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct DtsDescriptor {
        pub sample_rate_code: u8,
        pub bit_rate_code: u8,
        pub nblks: u8,
        pub fsize: u16,
        pub surround_mode: u8,
        pub lfe: bool,
        pub extended_surround_flag: u8,
        pub additional_info_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x7b },
        descriptor_length: { 8 },
        sample_rate_code: { 4 },
        bit_rate_code: { 6 },
        nblks: { 7 },
        fsize: { 14 },
        surround_mode: { 6 },
        lfe: { 1, map: bool_flag },
        extended_surround_flag: { 2 },
        additional_info_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for DtsDescriptor {}


// 0x7c AacDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct AacDescriptor {
        pub profile_and_level: u8,
        pub aac_type: Option<u8>,
        pub additional_info_bytes: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x7c },
        descriptor_length: { 8 },
        profile_and_level: { 8 },
        aac_type: { value: if descriptor_length > 1 {
            let aac_type_flag = try!(reader.read_u8(1));
            try!(::base::reserved(reader, 7));
            if aac_type_flag == 1 {
                Some(try!(reader.read_u8(8)))
            } else { None }
        } else { None } },
        additional_info_bytes: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for AacDescriptor {}


// 0x7d XaitLocationDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct XaitLocationDescriptor {
        pub xait_original_network_id: u16,
        pub xait_service_id: u16,
        pub xait_version_number: u8,
        pub xait_update_policy: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x7d },
        descriptor_length: { 8 },
        xait_original_network_id: { 16 },
        xait_service_id: { 16 },
        xait_version_number: { 5 },
        xait_update_policy: { 3 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for XaitLocationDescriptor {}


// 0x7e FtaDescriptor
bit_struct!(
    #[derive(Debug)]
    pub struct FtaDescriptor {
        pub do_not_scramble: bool,
        pub control_remote_access_over_internet: u8,
        pub do_not_apply_revocation: bool
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 0x7e },
        descriptor_length: { 8 },
        reserved: { 4 },
        do_not_scramble: { 1, map: bool_flag },
        control_remote_access_over_internet: { 2 },
        do_not_apply_revocation: { 1, map: bool_flag },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for FtaDescriptor {}


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


fn read_string(length: u8, reader: &mut bitreader::BitReader) -> bitreader::Result<String> {
    let mut bytes = Vec::with_capacity(length as usize);
    for _ in 0..length {
        bytes.push(try!(reader.read_u8(8)));
    }
    Ok(bytes_to_string(&bytes[..]))
}

fn read_string_latin1(length: u8, reader: &mut bitreader::BitReader) -> bitreader::Result<String> {
    let mut string = String::with_capacity(length as usize);
    for _ in 0..length {
        string.push(try!(reader.read_u8(8)) as char);
    }
    Ok(string)
}

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
