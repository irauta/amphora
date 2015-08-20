
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
