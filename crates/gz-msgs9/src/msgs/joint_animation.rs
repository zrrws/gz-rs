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

//! Generated file from `gz/msgs/joint_animation.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.JointAnimation)
pub struct JointAnimation {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.JointAnimation.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.JointAnimation.model_name)
    pub model_name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.JointAnimation.joint)
    pub joint: ::std::vec::Vec<joint_animation::Joint>,
    // @@protoc_insertion_point(field:gz.msgs.JointAnimation.time)
    pub time: ::std::vec::Vec<super::time::Time>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.JointAnimation.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a JointAnimation {
    fn default() -> &'a JointAnimation {
        <JointAnimation as ::protobuf::Message>::default_instance()
    }
}

impl JointAnimation {
    pub fn new() -> JointAnimation {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &JointAnimation| { &m.header },
            |m: &mut JointAnimation| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "model_name",
            |m: &JointAnimation| { &m.model_name },
            |m: &mut JointAnimation| { &mut m.model_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "joint",
            |m: &JointAnimation| { &m.joint },
            |m: &mut JointAnimation| { &mut m.joint },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "time",
            |m: &JointAnimation| { &m.time },
            |m: &mut JointAnimation| { &mut m.time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<JointAnimation>(
            "JointAnimation",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for JointAnimation {
    const NAME: &'static str = "JointAnimation";

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
                    self.model_name = is.read_string()?;
                },
                26 => {
                    self.joint.push(is.read_message()?);
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
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if !self.model_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.model_name);
        }
        for value in &self.joint {
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
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.model_name.is_empty() {
            os.write_string(2, &self.model_name)?;
        }
        for v in &self.joint {
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

    fn new() -> JointAnimation {
        JointAnimation::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.model_name.clear();
        self.joint.clear();
        self.time.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static JointAnimation {
        static instance: JointAnimation = JointAnimation {
            header: ::protobuf::MessageField::none(),
            model_name: ::std::string::String::new(),
            joint: ::std::vec::Vec::new(),
            time: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for JointAnimation {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("JointAnimation").unwrap()).clone()
    }
}

impl ::std::fmt::Display for JointAnimation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JointAnimation {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `JointAnimation`
pub mod joint_animation {
    #[derive(PartialEq,Clone,Default,Debug)]
    #[derive(::gz_msgs_derive::GzMessage)]
    // @@protoc_insertion_point(message:gz.msgs.JointAnimation.Joint)
    pub struct Joint {
        // message fields
        // @@protoc_insertion_point(field:gz.msgs.JointAnimation.Joint.name)
        pub name: ::std::vec::Vec<::std::string::String>,
        // @@protoc_insertion_point(field:gz.msgs.JointAnimation.Joint.angle)
        pub angle: ::std::vec::Vec<f64>,
        // special fields
        // @@protoc_insertion_point(special_field:gz.msgs.JointAnimation.Joint.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Joint {
        fn default() -> &'a Joint {
            <Joint as ::protobuf::Message>::default_instance()
        }
    }

    impl Joint {
        pub fn new() -> Joint {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
                "name",
                |m: &Joint| { &m.name },
                |m: &mut Joint| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
                "angle",
                |m: &Joint| { &m.angle },
                |m: &mut Joint| { &mut m.angle },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Joint>(
                "JointAnimation.Joint",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Joint {
        const NAME: &'static str = "Joint";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.name.push(is.read_string()?);
                    },
                    18 => {
                        is.read_repeated_packed_double_into(&mut self.angle)?;
                    },
                    17 => {
                        self.angle.push(is.read_double()?);
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
            for value in &self.name {
                my_size += ::protobuf::rt::string_size(1, &value);
            };
            my_size += 9 * self.angle.len() as u64;
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            for v in &self.name {
                os.write_string(1, &v)?;
            };
            for v in &self.angle {
                os.write_double(2, *v)?;
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

        fn new() -> Joint {
            Joint::new()
        }

        fn clear(&mut self) {
            self.name.clear();
            self.angle.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Joint {
            static instance: Joint = Joint {
                name: ::std::vec::Vec::new(),
                angle: ::std::vec::Vec::new(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Joint {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("JointAnimation.Joint").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Joint {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Joint {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1dgz/msgs/joint_animation.proto\x12\x07gz.msgs\x1a\x12gz/msgs/time.p\
    roto\x1a\x14gz/msgs/header.proto\"\xe3\x01\n\x0eJointAnimation\x12'\n\
    \x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12\x1d\n\
    \nmodel_name\x18\x02\x20\x01(\tR\tmodelName\x123\n\x05joint\x18\x03\x20\
    \x03(\x0b2\x1d.gz.msgs.JointAnimation.JointR\x05joint\x12!\n\x04time\x18\
    \x04\x20\x03(\x0b2\r.gz.msgs.TimeR\x04time\x1a1\n\x05Joint\x12\x12\n\x04\
    name\x18\x01\x20\x03(\tR\x04name\x12\x14\n\x05angle\x18\x02\x20\x03(\x01\
    R\x05angleB#\n\x0bcom.gz.msgsB\x14JointAnimationProtosb\x06proto3\
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
            deps.push(super::time::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(JointAnimation::generated_message_descriptor_data());
            messages.push(joint_animation::Joint::generated_message_descriptor_data());
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
