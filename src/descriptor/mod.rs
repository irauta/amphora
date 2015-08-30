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

use std::fmt::Debug;

use ::base::*;
use bitreader::BitReader;
use bitreader::Result as BitReaderResult;

pub trait Descriptor: Debug {}

pub mod basic;
pub mod dvb;

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

            0x40 => dvb::NetworkNameDescriptor,
            0x41 => dvb::ServiceListDescriptor,
            0x42 => dvb::StuffingDescriptor,
            0x43 => dvb::SatelliteDeliverySystemDescriptor,
            0x44 => dvb::CableDeliverySystemDescriptor,
            //0x45 => dvb::VbiDataDescriptor,
            //0x46 => dvb::VbiTeletextDescriptor,
            0x47 => dvb::BouquetNameDescriptor,
            0x48 => dvb::ServiceDescriptor,
            0x49 => dvb::CountryAvailabilityDescriptor,
            0x4a => dvb::LinkageDescriptor,
            0x4b => dvb::NvodReferenceDescriptor,
            //0x4c => dvb::TimeShiftedServiceDescriptor,
            0x4d => dvb::ShortEventDescriptor,
            0x4e => dvb::ExtendedEventDescriptor,
            //0x4f => dvb::TimeShiftedEventDescriptor,
            0x50 => dvb::ComponentDescriptor,
            0x51 => dvb::MosaicDescriptor,
            0x52 => dvb::StreamIdentifierDescriptor,
            0x53 => dvb::CaIdentifierDescriptor,
            0x54 => dvb::ContentDescriptor,
            0x55 => dvb::ParentalRatingDescriptor,
            0x56 => dvb::TeletextDescriptor,
            0x57 => dvb::TelephoneDescriptor,
            0x58 => dvb::LocalTimeOffsetDescriptor,
            0x59 => dvb::SubtitlingDescriptor,
            0x5a => dvb::TerrestrialDeliverySystemDescriptor,
            0x5b => dvb::MultilingualNetworkNameDescriptor,
            0x5c => dvb::MultilingualBouquetNameDescriptor,
            0x5d => dvb::MultilingualServiceNameDescriptor,
            0x5e => dvb::MultilingualComponentDescriptor,
            0x5f => dvb::PrivateDataSpecifierDescriptor,
            0x60 => dvb::ServiceMoveDescriptor,
            0x61 => dvb::ShortSmoothingBufferDescriptor,
            0x62 => dvb::FrequencyListDescriptor,
            0x63 => dvb::PartialTransportStreamDescriptor,
            0x64 => dvb::DataBroadcastDescriptor,
            0x65 => dvb::ScramblingDescriptor,
            0x66 => dvb::DataBroadcastIdDescriptor,
            //0x67 => dvb::TransportStreamDescriptor,
            //0x68 => dvb::DsngDescriptor,
            0x69 => dvb::PdcDescriptor,
            //0x6a => dvb::Ac3Descriptor,
            0x6b => dvb::AncillaryDataDescriptor,
            //0x6c => dvb::CellListDescriptor,
            //0x6d => dvb::CellFrequencyLinkDescriptor,
            0x6e => dvb::AnnouncementSupportDescriptor,
            //0x6f => dvb::ApplicationSignallingDescriptor,
            0x70 => dvb::AdaptationFieldDataDescriptor,
            //0x71 => dvb::ServiceIdentifierDescriptor,
            0x72 => dvb::ServiceAvailabilityDescriptor,
            //0x73 => dvb::DefaultAuthorityDescriptor,
            //0x74 => dvb::RelatedContentDescriptor,
            //0x75 => dvb::TvaIdDescriptor,
            //0x76 => dvb::ContentIdentifierDescriptor,
            //0x77 => dvb::TimeSliceFecIdentifierDescriptor,
            //0x78 => dvb::EcmRepetitionRateDescriptor,
            0x79 => dvb::S2SatelliteDeliverySystemDescriptor,
            //0x7a => dvb::EnhancedAc3Descriptor,
            //0x7b => dvb::DtsDescriptor,
            //0x7c => dvb::AacDescriptor,
            //0x7d => dvb::XaitDescriptor,
            0x7e => dvb::FtaDescriptor,
            0x7f => dvb::ExtensionDescriptor,

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

fn repeated_element<T: Deserialize>(descriptor_length: u8, reader: &mut BitReader) -> DeserializationResult<Vec<T>> {
    let bytes_remaining = bits_remaining(descriptor_length, reader) / 8;
    ::base::read_repeated(bytes_remaining as usize, reader)
}

fn repeated_sub_element<T: Deserialize>(element_bytes: u8, reader: &mut BitReader) -> DeserializationResult<Vec<T>> {
    ::base::read_repeated(element_bytes as usize, reader)
}

fn read_tla(reader: &mut BitReader) -> BitReaderResult<String> {
    let mut tla = String::with_capacity(3);
    tla.push(try!(reader.read_u8(8)) as char);
    tla.push(try!(reader.read_u8(8)) as char);
    tla.push(try!(reader.read_u8(8)) as char);
    Ok(tla)
}
