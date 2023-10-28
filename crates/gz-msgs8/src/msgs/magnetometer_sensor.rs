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

//! Generated file from `ignition/msgs/magnetometer_sensor.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.MagnetometerSensor)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MagnetometerSensor {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.MagnetometerSensor.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.MagnetometerSensor.x_noise)
    pub x_noise: ::protobuf::MessageField<super::sensor_noise::SensorNoise>,
    // @@protoc_insertion_point(field:ignition.msgs.MagnetometerSensor.y_noise)
    pub y_noise: ::protobuf::MessageField<super::sensor_noise::SensorNoise>,
    // @@protoc_insertion_point(field:ignition.msgs.MagnetometerSensor.z_noise)
    pub z_noise: ::protobuf::MessageField<super::sensor_noise::SensorNoise>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.MagnetometerSensor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MagnetometerSensor {
    fn default() -> &'a MagnetometerSensor {
        <MagnetometerSensor as ::protobuf::Message>::default_instance()
    }
}

impl MagnetometerSensor {
    pub fn new() -> MagnetometerSensor {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &MagnetometerSensor| { &m.header },
            |m: &mut MagnetometerSensor| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::sensor_noise::SensorNoise>(
            "x_noise",
            |m: &MagnetometerSensor| { &m.x_noise },
            |m: &mut MagnetometerSensor| { &mut m.x_noise },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::sensor_noise::SensorNoise>(
            "y_noise",
            |m: &MagnetometerSensor| { &m.y_noise },
            |m: &mut MagnetometerSensor| { &mut m.y_noise },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::sensor_noise::SensorNoise>(
            "z_noise",
            |m: &MagnetometerSensor| { &m.z_noise },
            |m: &mut MagnetometerSensor| { &mut m.z_noise },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MagnetometerSensor>(
            "MagnetometerSensor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MagnetometerSensor {
    const NAME: &'static str = "MagnetometerSensor";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.x_noise)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.y_noise)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.z_noise)?;
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
        if let Some(v) = self.x_noise.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.y_noise.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.z_noise.as_ref() {
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
        if let Some(v) = self.x_noise.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.y_noise.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.z_noise.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> MagnetometerSensor {
        MagnetometerSensor::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.x_noise.clear();
        self.y_noise.clear();
        self.z_noise.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MagnetometerSensor {
        static instance: MagnetometerSensor = MagnetometerSensor {
            header: ::protobuf::MessageField::none(),
            x_noise: ::protobuf::MessageField::none(),
            y_noise: ::protobuf::MessageField::none(),
            z_noise: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MagnetometerSensor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MagnetometerSensor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MagnetometerSensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MagnetometerSensor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'ignition/msgs/magnetometer_sensor.proto\x12\rignition.msgs\x1a\x1aign\
    ition/msgs/header.proto\x1a\x20ignition/msgs/sensor_noise.proto\"\xe2\
    \x01\n\x12MagnetometerSensor\x12-\n\x06header\x18\x01\x20\x01(\x0b2\x15.\
    ignition.msgs.HeaderR\x06header\x123\n\x07x_noise\x18\x02\x20\x01(\x0b2\
    \x1a.ignition.msgs.SensorNoiseR\x06xNoise\x123\n\x07y_noise\x18\x03\x20\
    \x01(\x0b2\x1a.ignition.msgs.SensorNoiseR\x06yNoise\x123\n\x07z_noise\
    \x18\x04\x20\x01(\x0b2\x1a.ignition.msgs.SensorNoiseR\x06zNoiseB-\n\x11c\
    om.ignition.msgsB\x18MagnetometerSensorProtosb\x06proto3\
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
            deps.push(super::sensor_noise::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MagnetometerSensor::generated_message_descriptor_data());
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
