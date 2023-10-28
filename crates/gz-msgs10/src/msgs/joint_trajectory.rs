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

//! Generated file from `gz/msgs/joint_trajectory.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.JointTrajectory)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct JointTrajectory {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.JointTrajectory.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.JointTrajectory.joint_names)
    pub joint_names: ::std::vec::Vec<::std::string::String>,
    // @@protoc_insertion_point(field:gz.msgs.JointTrajectory.points)
    pub points: ::std::vec::Vec<super::joint_trajectory_point::JointTrajectoryPoint>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.JointTrajectory.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JointTrajectory {
    fn default() -> &'a JointTrajectory {
        <JointTrajectory as ::protobuf::Message>::default_instance()
    }
}

impl JointTrajectory {
    pub fn new() -> JointTrajectory {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &JointTrajectory| { &m.header },
            |m: &mut JointTrajectory| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "joint_names",
            |m: &JointTrajectory| { &m.joint_names },
            |m: &mut JointTrajectory| { &mut m.joint_names },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "points",
            |m: &JointTrajectory| { &m.points },
            |m: &mut JointTrajectory| { &mut m.points },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JointTrajectory>(
            "JointTrajectory",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JointTrajectory {
    const NAME: &'static str = "JointTrajectory";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                18 => {
                    self.joint_names.push(is.read_string()?);
                },
                26 => {
                    self.points.push(is.read_message()?);
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
        for value in &self.joint_names {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.points {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        for v in &self.joint_names {
            os.write_string(2, &v)?;
        };
        for v in &self.points {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> JointTrajectory {
        JointTrajectory::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.joint_names.clear();
        self.points.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JointTrajectory {
        static instance: JointTrajectory = JointTrajectory {
            header: ::protobuf::MessageField::none(),
            joint_names: ::std::vec::Vec::new(),
            points: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JointTrajectory {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JointTrajectory").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JointTrajectory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JointTrajectory {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1egz/msgs/joint_trajectory.proto\x12\x07gz.msgs\x1a\x14gz/msgs/heade\
    r.proto\x1a$gz/msgs/joint_trajectory_point.proto\"\x92\x01\n\x0fJointTra\
    jectory\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06he\
    ader\x12\x1f\n\x0bjoint_names\x18\x02\x20\x03(\tR\njointNames\x125\n\x06\
    points\x18\x03\x20\x03(\x0b2\x1d.gz.msgs.JointTrajectoryPointR\x06points\
    B$\n\x0bcom.gz.msgsB\x15JointTrajectoryProtosb\x06proto3\
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
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::joint_trajectory_point::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(JointTrajectory::generated_message_descriptor_data());
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
