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

//! Generated file from `gz/msgs/log_control.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.LogControl)
pub struct LogControl {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.LogControl.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.LogControl.start)
    pub start: bool,
    // @@protoc_insertion_point(field:gz.msgs.LogControl.stop)
    pub stop: bool,
    // @@protoc_insertion_point(field:gz.msgs.LogControl.paused)
    pub paused: bool,
    // @@protoc_insertion_point(field:gz.msgs.LogControl.base_path)
    pub base_path: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.LogControl.encoding)
    pub encoding: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.LogControl.record_resources)
    pub record_resources: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.LogControl.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LogControl {
    fn default() -> &'a LogControl {
        <LogControl as ::protobuf::Message>::default_instance()
    }
}

impl LogControl {
    pub fn new() -> LogControl {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &LogControl| { &m.header },
            |m: &mut LogControl| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "start",
            |m: &LogControl| { &m.start },
            |m: &mut LogControl| { &mut m.start },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stop",
            |m: &LogControl| { &m.stop },
            |m: &mut LogControl| { &mut m.stop },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "paused",
            |m: &LogControl| { &m.paused },
            |m: &mut LogControl| { &mut m.paused },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_path",
            |m: &LogControl| { &m.base_path },
            |m: &mut LogControl| { &mut m.base_path },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "encoding",
            |m: &LogControl| { &m.encoding },
            |m: &mut LogControl| { &mut m.encoding },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "record_resources",
            |m: &LogControl| { &m.record_resources },
            |m: &mut LogControl| { &mut m.record_resources },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LogControl>(
            "LogControl",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LogControl {
    const NAME: &'static str = "LogControl";

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
                    self.start = is.read_bool()?;
                },
                24 => {
                    self.stop = is.read_bool()?;
                },
                32 => {
                    self.paused = is.read_bool()?;
                },
                42 => {
                    self.base_path = is.read_string()?;
                },
                50 => {
                    self.encoding = is.read_string()?;
                },
                58 => {
                    self.record_resources = is.read_string()?;
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
        if self.start != false {
            my_size += 1 + 1;
        }
        if self.stop != false {
            my_size += 1 + 1;
        }
        if self.paused != false {
            my_size += 1 + 1;
        }
        if !self.base_path.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.base_path);
        }
        if !self.encoding.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.encoding);
        }
        if !self.record_resources.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.record_resources);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.start != false {
            os.write_bool(2, self.start)?;
        }
        if self.stop != false {
            os.write_bool(3, self.stop)?;
        }
        if self.paused != false {
            os.write_bool(4, self.paused)?;
        }
        if !self.base_path.is_empty() {
            os.write_string(5, &self.base_path)?;
        }
        if !self.encoding.is_empty() {
            os.write_string(6, &self.encoding)?;
        }
        if !self.record_resources.is_empty() {
            os.write_string(7, &self.record_resources)?;
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

    fn new() -> LogControl {
        LogControl::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.start = false;
        self.stop = false;
        self.paused = false;
        self.base_path.clear();
        self.encoding.clear();
        self.record_resources.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LogControl {
        static instance: LogControl = LogControl {
            header: ::protobuf::MessageField::none(),
            start: false,
            stop: false,
            paused: false,
            base_path: ::std::string::String::new(),
            encoding: ::std::string::String::new(),
            record_resources: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LogControl {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LogControl").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LogControl {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LogControl {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19gz/msgs/log_control.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.pro\
    to\"\xdb\x01\n\nLogControl\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz\
    .msgs.HeaderR\x06header\x12\x14\n\x05start\x18\x02\x20\x01(\x08R\x05star\
    t\x12\x12\n\x04stop\x18\x03\x20\x01(\x08R\x04stop\x12\x16\n\x06paused\
    \x18\x04\x20\x01(\x08R\x06paused\x12\x1b\n\tbase_path\x18\x05\x20\x01(\t\
    R\x08basePath\x12\x1a\n\x08encoding\x18\x06\x20\x01(\tR\x08encoding\x12)\
    \n\x10record_resources\x18\x07\x20\x01(\tR\x0frecordResourcesB\x1f\n\x0b\
    com.gz.msgsB\x10LogControlProtosb\x06proto3\
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
            messages.push(LogControl::generated_message_descriptor_data());
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
