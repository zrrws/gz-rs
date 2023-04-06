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

//! Generated file from `gz/msgs/rest_login.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:gz.msgs.RestLogin)
pub struct RestLogin {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.RestLogin.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.RestLogin.id)
    pub id: u32,
    // @@protoc_insertion_point(field:gz.msgs.RestLogin.url)
    pub url: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.RestLogin.username)
    pub username: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.RestLogin.password)
    pub password: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.RestLogin.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RestLogin {
    fn default() -> &'a RestLogin {
        <RestLogin as ::protobuf::Message>::default_instance()
    }
}

impl RestLogin {
    pub fn new() -> RestLogin {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &RestLogin| { &m.header },
            |m: &mut RestLogin| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &RestLogin| { &m.id },
            |m: &mut RestLogin| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "url",
            |m: &RestLogin| { &m.url },
            |m: &mut RestLogin| { &mut m.url },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "username",
            |m: &RestLogin| { &m.username },
            |m: &mut RestLogin| { &mut m.username },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "password",
            |m: &RestLogin| { &m.password },
            |m: &mut RestLogin| { &mut m.password },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RestLogin>(
            "RestLogin",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RestLogin {
    const NAME: &'static str = "RestLogin";

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
                    self.id = is.read_uint32()?;
                },
                26 => {
                    self.url = is.read_string()?;
                },
                34 => {
                    self.username = is.read_string()?;
                },
                42 => {
                    self.password = is.read_string()?;
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
            my_size += ::protobuf::rt::uint32_size(2, self.id);
        }
        if !self.url.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.url);
        }
        if !self.username.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.username);
        }
        if !self.password.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.password);
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
            os.write_uint32(2, self.id)?;
        }
        if !self.url.is_empty() {
            os.write_string(3, &self.url)?;
        }
        if !self.username.is_empty() {
            os.write_string(4, &self.username)?;
        }
        if !self.password.is_empty() {
            os.write_string(5, &self.password)?;
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

    fn new() -> RestLogin {
        RestLogin::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.id = 0;
        self.url.clear();
        self.username.clear();
        self.password.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RestLogin {
        static instance: RestLogin = RestLogin {
            header: ::protobuf::MessageField::none(),
            id: 0,
            url: ::std::string::String::new(),
            username: ::std::string::String::new(),
            password: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RestLogin {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RestLogin").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RestLogin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RestLogin {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18gz/msgs/rest_login.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.prot\
    o\"\x8e\x01\n\tRestLogin\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.m\
    sgs.HeaderR\x06header\x12\x0e\n\x02id\x18\x02\x20\x01(\rR\x02id\x12\x10\
    \n\x03url\x18\x03\x20\x01(\tR\x03url\x12\x1a\n\x08username\x18\x04\x20\
    \x01(\tR\x08username\x12\x1a\n\x08password\x18\x05\x20\x01(\tR\x08passwo\
    rdB\x1e\n\x0bcom.gz.msgsB\x0fRestLoginProtosb\x06proto3\
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
            messages.push(RestLogin::generated_message_descriptor_data());
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
