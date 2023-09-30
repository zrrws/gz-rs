// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc 3.12.4
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `ignition/msgs/parameter_error.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.ParameterError)
pub struct ParameterError {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.ParameterError.data)
    pub data: ::protobuf::EnumOrUnknown<parameter_error::Type>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.ParameterError.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ParameterError {
    fn default() -> &'a ParameterError {
        <ParameterError as ::protobuf::Message>::default_instance()
    }
}

impl ParameterError {
    pub fn new() -> ParameterError {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &ParameterError| { &m.data },
            |m: &mut ParameterError| { &mut m.data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ParameterError>(
            "ParameterError",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ParameterError {
    const NAME: &'static str = "ParameterError";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.data = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.data != ::protobuf::EnumOrUnknown::new(parameter_error::Type::SUCCESS) {
            my_size += ::protobuf::rt::int32_size(1, self.data.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.data != ::protobuf::EnumOrUnknown::new(parameter_error::Type::SUCCESS) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.data))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> ParameterError {
        ParameterError::new()
    }

    fn clear(&mut self) {
        self.data = ::protobuf::EnumOrUnknown::new(parameter_error::Type::SUCCESS);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ParameterError {
        static instance: ParameterError = ParameterError {
            data: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ParameterError {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ParameterError").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ParameterError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ParameterError {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `ParameterError`
pub mod parameter_error {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:ignition.msgs.ParameterError.Type)
    pub enum Type {
        // @@protoc_insertion_point(enum_value:ignition.msgs.ParameterError.Type.SUCCESS)
        SUCCESS = 0,
        // @@protoc_insertion_point(enum_value:ignition.msgs.ParameterError.Type.ALREADY_DECLARED)
        ALREADY_DECLARED = 1,
        // @@protoc_insertion_point(enum_value:ignition.msgs.ParameterError.Type.INVALID_TYPE)
        INVALID_TYPE = 2,
        // @@protoc_insertion_point(enum_value:ignition.msgs.ParameterError.Type.NOT_DECLARED)
        NOT_DECLARED = 3,
    }

    impl ::protobuf::Enum for Type {
        const NAME: &'static str = "Type";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Type> {
            match value {
                0 => ::std::option::Option::Some(Type::SUCCESS),
                1 => ::std::option::Option::Some(Type::ALREADY_DECLARED),
                2 => ::std::option::Option::Some(Type::INVALID_TYPE),
                3 => ::std::option::Option::Some(Type::NOT_DECLARED),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Type] = &[
            Type::SUCCESS,
            Type::ALREADY_DECLARED,
            Type::INVALID_TYPE,
            Type::NOT_DECLARED,
        ];
    }

    impl ::protobuf::EnumFull for Type {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("ParameterError.Type").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Type {
        fn default() -> Self {
            Type::SUCCESS
        }
    }

    impl Type {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Type>("ParameterError.Type")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#ignition/msgs/parameter_error.proto\x12\rignition.msgs\"\x97\x01\n\
    \x0eParameterError\x126\n\x04data\x18\x01\x20\x01(\x0e2\".ignition.msgs.\
    ParameterError.TypeR\x04data\"M\n\x04Type\x12\x0b\n\x07SUCCESS\x10\0\x12\
    \x14\n\x10ALREADY_DECLARED\x10\x01\x12\x10\n\x0cINVALID_TYPE\x10\x02\x12\
    \x10\n\x0cNOT_DECLARED\x10\x03B)\n\x11com.ignition.msgsB\x14ParameterErr\
    orProtosb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ParameterError::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(parameter_error::Type::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
