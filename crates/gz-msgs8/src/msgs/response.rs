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

//! Generated file from `ignition/msgs/response.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.Response)
pub struct Response {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.Response.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.Response.id)
    pub id: i32,
    // @@protoc_insertion_point(field:ignition.msgs.Response.request)
    pub request: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.Response.response)
    pub response: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.Response.type)
    pub type_: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.Response.serialized_data)
    pub serialized_data: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.Response.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Response {
    fn default() -> &'a Response {
        <Response as ::protobuf::Message>::default_instance()
    }
}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Response| { &m.header },
            |m: &mut Response| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Response| { &m.id },
            |m: &mut Response| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "request",
            |m: &Response| { &m.request },
            |m: &mut Response| { &mut m.request },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "response",
            |m: &Response| { &m.response },
            |m: &mut Response| { &mut m.response },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Response| { &m.type_ },
            |m: &mut Response| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "serialized_data",
            |m: &Response| { &m.serialized_data },
            |m: &mut Response| { &mut m.serialized_data },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Response>(
            "Response",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Response {
    const NAME: &'static str = "Response";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                16 => {
                    self.id = is.read_int32()?;
                },
                26 => {
                    self.request = is.read_string()?;
                },
                34 => {
                    self.response = is.read_string()?;
                },
                42 => {
                    self.type_ = is.read_string()?;
                },
                50 => {
                    self.serialized_data = is.read_bytes()?;
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
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.id);
        }
        if !self.request.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.request);
        }
        if !self.response.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.response);
        }
        if !self.type_.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.type_);
        }
        if !self.serialized_data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.serialized_data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.id != 0 {
            os.write_int32(2, self.id)?;
        }
        if !self.request.is_empty() {
            os.write_string(3, &self.request)?;
        }
        if !self.response.is_empty() {
            os.write_string(4, &self.response)?;
        }
        if !self.type_.is_empty() {
            os.write_string(5, &self.type_)?;
        }
        if !self.serialized_data.is_empty() {
            os.write_bytes(6, &self.serialized_data)?;
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

    fn new() -> Response {
        Response::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.id = 0;
        self.request.clear();
        self.response.clear();
        self.type_.clear();
        self.serialized_data.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Response {
        static instance: Response = Response {
            header: ::protobuf::MessageField::none(),
            id: 0,
            request: ::std::string::String::new(),
            response: ::std::string::String::new(),
            type_: ::std::string::String::new(),
            serialized_data: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Response {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Response").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cignition/msgs/response.proto\x12\rignition.msgs\x1a\x1aignition/ms\
    gs/header.proto\"\xbc\x01\n\x08Response\x12-\n\x06header\x18\x01\x20\x01\
    (\x0b2\x15.ignition.msgs.HeaderR\x06header\x12\x0e\n\x02id\x18\x02\x20\
    \x01(\x05R\x02id\x12\x18\n\x07request\x18\x03\x20\x01(\tR\x07request\x12\
    \x1a\n\x08response\x18\x04\x20\x01(\tR\x08response\x12\x12\n\x04type\x18\
    \x05\x20\x01(\tR\x04type\x12'\n\x0fserialized_data\x18\x06\x20\x01(\x0cR\
    \x0eserializedDataB#\n\x11com.ignition.msgsB\x0eResponseProtosb\x06proto\
    3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Response::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
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
