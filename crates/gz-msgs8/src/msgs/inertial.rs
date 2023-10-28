// This file is generated by rust-protobuf 3.3.0. Do not edit
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

//! Generated file from `ignition/msgs/inertial.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.Inertial)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Inertial {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.mass)
    pub mass: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.ixx)
    pub ixx: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.ixy)
    pub ixy: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.ixz)
    pub ixz: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.iyy)
    pub iyy: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.iyz)
    pub iyz: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Inertial.izz)
    pub izz: f64,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.Inertial.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Inertial {
    fn default() -> &'a Inertial {
        <Inertial as ::protobuf::Message>::default_instance()
    }
}

impl Inertial {
    pub fn new() -> Inertial {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(9);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Inertial| { &m.header },
            |m: &mut Inertial| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mass",
            |m: &Inertial| { &m.mass },
            |m: &mut Inertial| { &mut m.mass },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &Inertial| { &m.pose },
            |m: &mut Inertial| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ixx",
            |m: &Inertial| { &m.ixx },
            |m: &mut Inertial| { &mut m.ixx },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ixy",
            |m: &Inertial| { &m.ixy },
            |m: &mut Inertial| { &mut m.ixy },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ixz",
            |m: &Inertial| { &m.ixz },
            |m: &mut Inertial| { &mut m.ixz },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "iyy",
            |m: &Inertial| { &m.iyy },
            |m: &mut Inertial| { &mut m.iyy },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "iyz",
            |m: &Inertial| { &m.iyz },
            |m: &mut Inertial| { &mut m.iyz },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "izz",
            |m: &Inertial| { &m.izz },
            |m: &mut Inertial| { &mut m.izz },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Inertial>(
            "Inertial",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Inertial {
    const NAME: &'static str = "Inertial";

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
                    self.mass = is.read_double()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                33 => {
                    self.ixx = is.read_double()?;
                },
                41 => {
                    self.ixy = is.read_double()?;
                },
                49 => {
                    self.ixz = is.read_double()?;
                },
                57 => {
                    self.iyy = is.read_double()?;
                },
                65 => {
                    self.iyz = is.read_double()?;
                },
                73 => {
                    self.izz = is.read_double()?;
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
        if self.mass != 0. {
            my_size += 1 + 8;
        }
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ixx != 0. {
            my_size += 1 + 8;
        }
        if self.ixy != 0. {
            my_size += 1 + 8;
        }
        if self.ixz != 0. {
            my_size += 1 + 8;
        }
        if self.iyy != 0. {
            my_size += 1 + 8;
        }
        if self.iyz != 0. {
            my_size += 1 + 8;
        }
        if self.izz != 0. {
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
        if self.mass != 0. {
            os.write_double(2, self.mass)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if self.ixx != 0. {
            os.write_double(4, self.ixx)?;
        }
        if self.ixy != 0. {
            os.write_double(5, self.ixy)?;
        }
        if self.ixz != 0. {
            os.write_double(6, self.ixz)?;
        }
        if self.iyy != 0. {
            os.write_double(7, self.iyy)?;
        }
        if self.iyz != 0. {
            os.write_double(8, self.iyz)?;
        }
        if self.izz != 0. {
            os.write_double(9, self.izz)?;
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

    fn new() -> Inertial {
        Inertial::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.mass = 0.;
        self.pose.clear();
        self.ixx = 0.;
        self.ixy = 0.;
        self.ixz = 0.;
        self.iyy = 0.;
        self.iyz = 0.;
        self.izz = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Inertial {
        static instance: Inertial = Inertial {
            header: ::protobuf::MessageField::none(),
            mass: 0.,
            pose: ::protobuf::MessageField::none(),
            ixx: 0.,
            ixy: 0.,
            ixz: 0.,
            iyy: 0.,
            iyz: 0.,
            izz: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Inertial {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Inertial").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Inertial {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Inertial {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cignition/msgs/inertial.proto\x12\rignition.msgs\x1a\x18ignition/ms\
    gs/pose.proto\x1a\x1aignition/msgs/header.proto\"\xe2\x01\n\x08Inertial\
    \x12-\n\x06header\x18\x01\x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06hea\
    der\x12\x12\n\x04mass\x18\x02\x20\x01(\x01R\x04mass\x12'\n\x04pose\x18\
    \x03\x20\x01(\x0b2\x13.ignition.msgs.PoseR\x04pose\x12\x10\n\x03ixx\x18\
    \x04\x20\x01(\x01R\x03ixx\x12\x10\n\x03ixy\x18\x05\x20\x01(\x01R\x03ixy\
    \x12\x10\n\x03ixz\x18\x06\x20\x01(\x01R\x03ixz\x12\x10\n\x03iyy\x18\x07\
    \x20\x01(\x01R\x03iyy\x12\x10\n\x03iyz\x18\x08\x20\x01(\x01R\x03iyz\x12\
    \x10\n\x03izz\x18\t\x20\x01(\x01R\x03izzB#\n\x11com.ignition.msgsB\x0eIn\
    ertialProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::pose::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Inertial::generated_message_descriptor_data());
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
