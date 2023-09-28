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

//! Generated file from `ignition/msgs/imu.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.IMU)
pub struct IMU {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.IMU.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.IMU.entity_name)
    pub entity_name: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.IMU.orientation)
    pub orientation: ::protobuf::MessageField<super::quaternion::Quaternion>,
    // @@protoc_insertion_point(field:ignition.msgs.IMU.angular_velocity)
    pub angular_velocity: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:ignition.msgs.IMU.linear_acceleration)
    pub linear_acceleration: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.IMU.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IMU {
    fn default() -> &'a IMU {
        <IMU as ::protobuf::Message>::default_instance()
    }
}

impl IMU {
    pub fn new() -> IMU {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &IMU| { &m.header },
            |m: &mut IMU| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "entity_name",
            |m: &IMU| { &m.entity_name },
            |m: &mut IMU| { &mut m.entity_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::quaternion::Quaternion>(
            "orientation",
            |m: &IMU| { &m.orientation },
            |m: &mut IMU| { &mut m.orientation },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "angular_velocity",
            |m: &IMU| { &m.angular_velocity },
            |m: &mut IMU| { &mut m.angular_velocity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "linear_acceleration",
            |m: &IMU| { &m.linear_acceleration },
            |m: &mut IMU| { &mut m.linear_acceleration },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IMU>(
            "IMU",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IMU {
    const NAME: &'static str = "IMU";

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
                    self.entity_name = is.read_string()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.orientation)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.angular_velocity)?;
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.linear_acceleration)?;
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
        if !self.entity_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.entity_name);
        }
        if let Some(v) = self.orientation.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.angular_velocity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.linear_acceleration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.entity_name.is_empty() {
            os.write_string(2, &self.entity_name)?;
        }
        if let Some(v) = self.orientation.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.angular_velocity.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if let Some(v) = self.linear_acceleration.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> IMU {
        IMU::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.entity_name.clear();
        self.orientation.clear();
        self.angular_velocity.clear();
        self.linear_acceleration.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IMU {
        static instance: IMU = IMU {
            header: ::protobuf::MessageField::none(),
            entity_name: ::std::string::String::new(),
            orientation: ::protobuf::MessageField::none(),
            angular_velocity: ::protobuf::MessageField::none(),
            linear_acceleration: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IMU {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IMU").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IMU {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IMU {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ignition/msgs/imu.proto\x12\rignition.msgs\x1a\x1aignition/msgs/he\
    ader.proto\x1a\x1cignition/msgs/vector3d.proto\x1a\x1eignition/msgs/quat\
    ernion.proto\"\xa0\x02\n\x03IMU\x12-\n\x06header\x18\x01\x20\x01(\x0b2\
    \x15.ignition.msgs.HeaderR\x06header\x12\x1f\n\x0bentity_name\x18\x02\
    \x20\x01(\tR\nentityName\x12;\n\x0borientation\x18\x03\x20\x01(\x0b2\x19\
    .ignition.msgs.QuaternionR\x0borientation\x12B\n\x10angular_velocity\x18\
    \x04\x20\x01(\x0b2\x17.ignition.msgs.Vector3dR\x0fangularVelocity\x12H\n\
    \x13linear_acceleration\x18\x05\x20\x01(\x0b2\x17.ignition.msgs.Vector3d\
    R\x12linearAccelerationB\x1e\n\x11com.ignition.msgsB\tIMUProtosb\x06prot\
    o3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::vector3d::file_descriptor().clone());
            deps.push(super::quaternion::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(IMU::generated_message_descriptor_data());
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
