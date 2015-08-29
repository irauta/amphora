
use ::base::{Deserialize,bool_flag};
use super::{Descriptor,bits_remaining,repeated_element,read_tla};

bit_struct!(
    #[derive(Debug)]
    pub struct UnknownDescriptor {
        pub descriptor_tag: u8,
        pub descriptor_length: u8,
        pub data: Vec<u8>
    }
    deserialize(reader) {
        descriptor_tag: { 8 },
        descriptor_length: { 8 },
        data: { value: {
            let mut data = vec![];
            for _ in 0..descriptor_length {
                data.push(try!(reader.read_u8(8)));
            }
            data
        } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for UnknownDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct VideoStreamDescriptorExtension {
        pub profile_and_level_indication: u8,
        pub chroma_format: u8,
        pub frame_rate_extension: bool
    }
    deserialize(reader) {
        profile_and_level_indication: { 8 },
        chroma_format: { 2 },
        frame_rate_extension: { 1, map: bool_flag }
    }
);

bit_struct!(
    #[derive(Debug)]
    pub struct VideoStreamDescriptor {
        pub multiple_frame_rate: bool,
        pub frame_rate_code: u8,
        pub mpeg_1_only: bool,
        pub constrained_parameter: bool,
        pub still_picture: bool,
        pub extension :Option<VideoStreamDescriptorExtension>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 2 },
        descriptor_length: { 8 },
        multiple_frame_rate: { 1, map: bool_flag },
        frame_rate_code: { 4 },
        mpeg_1_only: { 1, map: bool_flag },
        constrained_parameter: { 1, map: bool_flag },
        still_picture: { 1, map: bool_flag },
        extension: { value: if mpeg_1_only {
                Some(try!(Deserialize::deserialize(reader)))
            } else {
                None
            }
        },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for VideoStreamDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct AudioStreamDescriptor {
        pub free_format: bool,
        pub id: bool,
        pub layer: u8,
        pub variable_rate_audio: bool
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 3 },
        descriptor_length: { 8 },
        free_format: { 1, map: bool_flag },
        id: { 1, map: bool_flag },
        layer: { 2 },
        variable_rate_audio: { 1, map: bool_flag },
        reserved: { 3 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for AudioStreamDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct HierarchyDescriptor {
        pub hierarchy_type: u8,
        pub hierarchy_layer_index: u8,
        pub hierarchy_embedded_layer_index: u8,
        pub hierarchy_channel: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 4 },
        descriptor_length: { 8 },
        reserved: { 4 },
        hierarchy_type: { 4 },
        reserved: { 2 },
        hierarchy_layer_index: { 6 },
        reserved: { 2 },
        hierarchy_embedded_layer_index: { 6 },
        reserved: { 2 },
        hierarchy_channel: { 6 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for HierarchyDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct RegistrationDescriptor {
        pub format_identifier: u32,
        pub additional_identification_info: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits:8, reference: 5 },
        descriptor_length: { 8 },
        format_identifier: { 32 },
        additional_identification_info: { value: {
            let mut data = vec![];
            while bits_remaining(descriptor_length, reader) >= 8 {
                data.push(try!(reader.read_u8(8)));
            }
            data
        } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for RegistrationDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct DataStreamAlignmentDescriptor {
        pub alignment_type: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 6 },
        descriptor_length: { 8 },
        alignment_type: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for DataStreamAlignmentDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct TargetBackgroundGridDescriptor {
        pub horizontal_size: u16,
        pub vertical_size: u16,
        pub aspect_ration_information: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 7 },
        descriptor_length: { 8 },
        horizontal_size: { 14 },
        vertical_size: { 14 },
        aspect_ration_information: { 4 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for TargetBackgroundGridDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct VideoWindowDescriptor {
        pub horizontal_offset: u16,
        pub vertical_offset: u16,
        pub window_priority: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 8 },
        descriptor_length: { 8 },
        horizontal_offset: { 14 },
        vertical_offset: { 14 },
        window_priority: { 4 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for VideoWindowDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct CaDescriptor {
        pub ca_system_id: u16,
        pub ca_pid: u16,
        pub data: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits:8, reference: 9 },
        descriptor_length: { 8 },
        ca_system_id: { 16 },
        reserved: { 3 },
        ca_pid: { 13 },
        data: { value: {
            let mut data = vec![];
            while bits_remaining(descriptor_length, reader) >= 8 {
                data.push(try!(reader.read_u8(8)));
            }
            data
        } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for CaDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct Iso639LanguageDescriptorLanguage {
        pub language: String,
        pub audio_type: u8
    }
    deserialize(reader) {
        language: { value: try!(read_tla(reader)) },
        audio_type: { 8 }
    }
);

bit_struct!(
    #[derive(Debug)]
    pub struct Iso639LanguageDescriptor {
        pub languages: Vec<Iso639LanguageDescriptorLanguage>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 10 },
        descriptor_length: { 8 },
        languages: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for Iso639LanguageDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct SystemClockDescriptor {
        pub external_clock_reference: bool,
        pub clock_accuracy_integer: u8,
        pub clock_accuracy_exponent: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 11 },
        descriptor_length: { 8 },
        external_clock_reference: { 1, map: bool_flag },
        reserved: { 1 },
        clock_accuracy_integer: { 6 },
        clock_accuracy_exponent: { 3 },
        reserved: { 5 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for SystemClockDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct MultiplexBufferUtilizationDescriptor {
        pub bound_valid: bool,
        pub ltw_offset_lower_bound: u16,
        pub ltw_offset_upper_bound: u16
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 12 },
        descriptor_length: { 8 },
        bound_valid: { 1, map: bool_flag },
        ltw_offset_lower_bound: { 15 },
        reserved: { 1 },
        ltw_offset_upper_bound: { 15 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MultiplexBufferUtilizationDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct CopyrightDescriptor {
        pub copyright_identifier: u8,
        pub additional_copyright_info: Vec<u8>
    }
    deserialize(reader) {
        expect: { bits:8, reference: 13 },
        descriptor_length: { 8 },
        copyright_identifier: { 8 },
        additional_copyright_info: { value: {
            let mut data = vec![];
            while bits_remaining(descriptor_length, reader) >= 8 {
                data.push(try!(reader.read_u8(8)));
            }
            data
        } },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for CopyrightDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct MaximumBitrateDescriptor {
        pub maximum_bitrate: u32
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 14 },
        descriptor_length: { 8 },
        reserved: { 2 },
        maximum_bitrate: { 22 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MaximumBitrateDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct PrivateDataIndicatorDescriptor {
        pub private_data_indicator: u32
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 15 },
        descriptor_length: { 8 },
        private_data_indicator: { 32 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for PrivateDataIndicatorDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct SmoothingBufferDescriptor {
        pub leak_rate: u32,
        pub size: u32
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 16 },
        descriptor_length: { 8 },
        reserved: { 2 },
        leak_rate: { 22 },
        reserved: { 2 },
        size: { 22 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for SmoothingBufferDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct StdDescriptor {
        pub leak_valid: bool
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 17 },
        descriptor_length: { 8 },
        reserved: { 7 },
        leak_valid: { 1, map: bool_flag },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for StdDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct IbpDescriptor {
        pub closed_gop: bool,
        pub identical_gop: bool,
        pub max_gop_length: u16
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 18 },
        descriptor_length: { 8 },
        closed_gop: { 1, map: bool_flag },
        identical_gop: { 1, map: bool_flag },
        max_gop_length: { 14 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for IbpDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct Mpeg4VideoDescriptor {
        pub visual_profile_and_level: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 27 },
        descriptor_length: { 8 },
        visual_profile_and_level: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for Mpeg4VideoDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct Mpeg4AudioDescriptor {
        pub audio_profile_and_level: u8
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 28 },
        descriptor_length: { 8 },
        audio_profile_and_level: { 8 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for Mpeg4AudioDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct IodDescriptor {
        pub scope_of_iod_label: u8,
        pub iod_label: u8
        // TODO: Missing initial object descriptor fields
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 29 },
        descriptor_length: { 8 },
        scope_of_iod_label: { 8 },
        iod_label: { 8 },
        // TODO: Missing initial object descriptor fields
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for IodDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct SlDescriptor {
        pub es_id: u16
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 30 },
        descriptor_length: { 8 },
        es_id: { 16 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for SlDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct FmcDescriptorChannel {
        pub es_id: u16,
        pub flexmux_channel: u8
    }
    deserialize(reader) {
        es_id: { 16 },
        flexmux_channel: { 8 }
    }
);

bit_struct!(
    #[derive(Debug)]
    pub struct FmcDescriptor {
        pub channels: Vec<FmcDescriptorChannel>
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 31 },
        descriptor_length: { 8 },
        channels: { value: try!(repeated_element(descriptor_length, reader)) },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for FmcDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct ExternalEsIdDescriptor {
        pub external_es_id: u16
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 32 },
        descriptor_length: { 8 },
        external_es_id: { 16 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for ExternalEsIdDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct MuxCodeDescriptor {
        pub empty: ()
        // TODO: Missing descriptor fields
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 33 },
        descriptor_length: { 8 },
        empty: { value: () },
        // TODO: Missing descriptor fields
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MuxCodeDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct FmxBufferSizeDescriptor {
        pub empty: ()
        // TODO: Missing descriptor fields
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 34 },
        descriptor_length: { 8 },
        empty: { value: () },
        // TODO: Missing descriptor fields
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for FmxBufferSizeDescriptor {}


bit_struct!(
    #[derive(Debug)]
    pub struct MultiplexBufferDescriptor {
        pub mb_buffer_size: u32,
        pub tb_leak_rate: u32
    }
    deserialize(reader) {
        expect: { bits: 8, reference: 35 },
        descriptor_length: { 8 },
        mb_buffer_size: { 24 },
        tb_leak_rate: { 24 },
        skip: { bits_remaining(descriptor_length, reader) }
    }
);
impl Descriptor for MultiplexBufferDescriptor {}
