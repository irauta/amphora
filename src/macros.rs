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

macro_rules! bit_struct {
    (
        $(#[$attr:meta])*
        pub struct $struct_name:ident {
            $(pub $field_name:ident : $field_type:ty),+
        }
        deserialize($reader:ident) {
            $($field:ident : { $($tokens:tt)+ }),+
        }
    ) => (
        $(#[$attr])*
        pub struct $struct_name {
            $(pub $field_name: $field_type),+
        }
        impl ::base::Deserialize for $struct_name {
            fn deserialize(original_reader: &mut ::bitreader::BitReader) -> ::base::DeserializationResult<$struct_name> {
                let mut relative_reader = original_reader.relative_reader();
                let $reader = &mut relative_reader;
                $( bit_struct!(field $field : $reader : { $($tokens)+ } ); )+
                try!(original_reader.skip($reader.position() as u32));
                Ok($struct_name {
                    $($field_name: $field_name),+
                })
            }
        }
    );

    (field reserved : $reader:ident : { $bits:expr }) => (
        try!(::base::reserved($reader, $bits))
    );

    (field expect : $reader:ident : { bits: $bits:expr, reference: $value:expr }) => (
        try!(::base::expect($reader, $bits, $value))
    );

    (field crc : $reader:ident : { 32 }) => (
        try!($reader.skip(32));
    );

    (field skip : $reader:ident : { $bits:expr }) => (
        {
            // Evaluate $bits before using the result, as $bits might use $reader which would
            // cause a borrow conflict.
            let bits = $bits;
            try!($reader.skip(bits))
        }
    );

    (field $field:ident : $reader:ident : { value : $e:expr }) => (
        let $field = $e;
    );

    (field $field:ident : $reader:ident : { $bits:expr }) => (
        let $field = try!(::bitreader::ReadInto::read($reader, $bits));
    );

    (field $field:ident : $reader:ident : { $bits:expr, map: $closure:expr }) => (
        let $field = ($closure)(try!(::bitreader::ReadInto::read($reader, $bits)));
    );

    (field $field:ident : $reader:ident : { $bits:expr, type: $T:ty }) => (
        let $field: $T = try!(::bitreader::ReadInto::read($reader, $bits));
    );
}
