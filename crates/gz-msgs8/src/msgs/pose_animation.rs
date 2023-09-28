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

//! Generated file from `ignition/msgs/pose_animation.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.PoseAnimation)
pub struct PoseAnimation {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.PoseAnimation.model_name)
    pub model_name: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.PoseAnimation.model_id)
    pub model_id: u32,
    // @@protoc_insertion_point(field:ignition.msgs.PoseAnimation.pose)
    pub pose: ::std::vec::Vec<super::pose::Pose>,
    // @@protoc_insertion_point(field:ignition.msgs.PoseAnimation.time)
    pub time: ::std::vec::Vec<super::time::Time>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.PoseAnimation.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PoseAnimation {
    fn default() -> &'a PoseAnimation {
        <PoseAnimation as ::protobuf::Message>::default_instance()
    }
}

impl PoseAnimation {
    pub fn new() -> PoseAnimation {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "model_name",
            |m: &PoseAnimation| { &m.model_name },
            |m: &mut PoseAnimation| { &mut m.model_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "model_id",
            |m: &PoseAnimation| { &m.model_id },
            |m: &mut PoseAnimation| { &mut m.model_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "pose",
            |m: &PoseAnimation| { &m.pose },
            |m: &mut PoseAnimation| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "time",
            |m: &PoseAnimation| { &m.time },
            |m: &mut PoseAnimation| { &mut m.time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PoseAnimation>(
            "PoseAnimation",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PoseAnimation {
    const NAME: &'static str = "PoseAnimation";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.model_name = is.read_string()?;
                },
                16 => {
                    self.model_id = is.read_uint32()?;
                },
                26 => {
                    self.pose.push(is.read_message()?);
                },
                34 => {
                    self.time.push(is.read_message()?);
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
        if !self.model_name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.model_name);
        }
        if self.model_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.model_id);
        }
        for value in &self.pose {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.model_name.is_empty() {
            os.write_string(1, &self.model_name)?;
        }
        if self.model_id != 0 {
            os.write_uint32(2, self.model_id)?;
        }
        for v in &self.pose {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        for v in &self.time {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> PoseAnimation {
        PoseAnimation::new()
    }

    fn clear(&mut self) {
        self.model_name.clear();
        self.model_id = 0;
        self.pose.clear();
        self.time.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PoseAnimation {
        static instance: PoseAnimation = PoseAnimation {
            model_name: ::std::string::String::new(),
            model_id: 0,
            pose: ::std::vec::Vec::new(),
            time: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PoseAnimation {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PoseAnimation").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PoseAnimation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PoseAnimation {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"ignition/msgs/pose_animation.proto\x12\rignition.msgs\x1a\x18ignitio\
    n/msgs/pose.proto\x1a\x18ignition/msgs/time.proto\"\x9b\x01\n\rPoseAnima\
    tion\x12\x1d\n\nmodel_name\x18\x01\x20\x01(\tR\tmodelName\x12\x19\n\x08m\
    odel_id\x18\x02\x20\x01(\rR\x07modelId\x12'\n\x04pose\x18\x03\x20\x03(\
    \x0b2\x13.ignition.msgs.PoseR\x04pose\x12'\n\x04time\x18\x04\x20\x03(\
    \x0b2\x13.ignition.msgs.TimeR\x04timeB(\n\x11com.ignition.msgsB\x13PoseA\
    nimationProtosb\x06proto3\
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
            deps.push(super::time::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PoseAnimation::generated_message_descriptor_data());
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
