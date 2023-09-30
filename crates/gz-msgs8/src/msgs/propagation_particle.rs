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

//! Generated file from `ignition/msgs/propagation_particle.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.PropagationParticle)
pub struct PropagationParticle {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.PropagationParticle.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.PropagationParticle.x)
    pub x: f64,
    // @@protoc_insertion_point(field:ignition.msgs.PropagationParticle.y)
    pub y: f64,
    // @@protoc_insertion_point(field:ignition.msgs.PropagationParticle.signal_level)
    pub signal_level: f64,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.PropagationParticle.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PropagationParticle {
    fn default() -> &'a PropagationParticle {
        <PropagationParticle as ::protobuf::Message>::default_instance()
    }
}

impl PropagationParticle {
    pub fn new() -> PropagationParticle {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &PropagationParticle| { &m.header },
            |m: &mut PropagationParticle| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "x",
            |m: &PropagationParticle| { &m.x },
            |m: &mut PropagationParticle| { &mut m.x },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "y",
            |m: &PropagationParticle| { &m.y },
            |m: &mut PropagationParticle| { &mut m.y },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "signal_level",
            |m: &PropagationParticle| { &m.signal_level },
            |m: &mut PropagationParticle| { &mut m.signal_level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PropagationParticle>(
            "PropagationParticle",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PropagationParticle {
    const NAME: &'static str = "PropagationParticle";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                17 => {
                    self.x = is.read_double()?;
                },
                25 => {
                    self.y = is.read_double()?;
                },
                33 => {
                    self.signal_level = is.read_double()?;
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
        if self.x != 0. {
            my_size += 1 + 8;
        }
        if self.y != 0. {
            my_size += 1 + 8;
        }
        if self.signal_level != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.x != 0. {
            os.write_double(2, self.x)?;
        }
        if self.y != 0. {
            os.write_double(3, self.y)?;
        }
        if self.signal_level != 0. {
            os.write_double(4, self.signal_level)?;
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

    fn new() -> PropagationParticle {
        PropagationParticle::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.x = 0.;
        self.y = 0.;
        self.signal_level = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PropagationParticle {
        static instance: PropagationParticle = PropagationParticle {
            header: ::protobuf::MessageField::none(),
            x: 0.,
            y: 0.,
            signal_level: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PropagationParticle {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PropagationParticle").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PropagationParticle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PropagationParticle {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(ignition/msgs/propagation_particle.proto\x12\rignition.msgs\x1a\x1aig\
    nition/msgs/header.proto\"\x83\x01\n\x13PropagationParticle\x12-\n\x06he\
    ader\x18\x01\x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\x12\x0c\n\
    \x01x\x18\x02\x20\x01(\x01R\x01x\x12\x0c\n\x01y\x18\x03\x20\x01(\x01R\
    \x01y\x12!\n\x0csignal_level\x18\x04\x20\x01(\x01R\x0bsignalLevelB.\n\
    \x11com.ignition.msgsB\x19PropagationParticleProtosb\x06proto3\
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
            messages.push(PropagationParticle::generated_message_descriptor_data());
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
