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

//! Generated file from `ignition/msgs/imu_sensor.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.IMUSensor)
pub struct IMUSensor {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.angular_velocity)
    pub angular_velocity: ::protobuf::MessageField<imusensor::AngularVelocity>,
    // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.linear_acceleration)
    pub linear_acceleration: ::protobuf::MessageField<imusensor::LinearAcceleration>,
    // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.orientation_ref_frame)
    pub orientation_ref_frame: ::protobuf::MessageField<imusensor::OrientationReferenceFrame>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.IMUSensor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a IMUSensor {
    fn default() -> &'a IMUSensor {
        <IMUSensor as ::protobuf::Message>::default_instance()
    }
}

impl IMUSensor {
    pub fn new() -> IMUSensor {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &IMUSensor| { &m.header },
            |m: &mut IMUSensor| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, imusensor::AngularVelocity>(
            "angular_velocity",
            |m: &IMUSensor| { &m.angular_velocity },
            |m: &mut IMUSensor| { &mut m.angular_velocity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, imusensor::LinearAcceleration>(
            "linear_acceleration",
            |m: &IMUSensor| { &m.linear_acceleration },
            |m: &mut IMUSensor| { &mut m.linear_acceleration },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, imusensor::OrientationReferenceFrame>(
            "orientation_ref_frame",
            |m: &IMUSensor| { &m.orientation_ref_frame },
            |m: &mut IMUSensor| { &mut m.orientation_ref_frame },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<IMUSensor>(
            "IMUSensor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for IMUSensor {
    const NAME: &'static str = "IMUSensor";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.angular_velocity)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.linear_acceleration)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.orientation_ref_frame)?;
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
        if let Some(v) = self.angular_velocity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.linear_acceleration.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.orientation_ref_frame.as_ref() {
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
        if let Some(v) = self.angular_velocity.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.linear_acceleration.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.orientation_ref_frame.as_ref() {
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

    fn new() -> IMUSensor {
        IMUSensor::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.angular_velocity.clear();
        self.linear_acceleration.clear();
        self.orientation_ref_frame.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static IMUSensor {
        static instance: IMUSensor = IMUSensor {
            header: ::protobuf::MessageField::none(),
            angular_velocity: ::protobuf::MessageField::none(),
            linear_acceleration: ::protobuf::MessageField::none(),
            orientation_ref_frame: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for IMUSensor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("IMUSensor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for IMUSensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IMUSensor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `IMUSensor`
pub mod imusensor {
    #[derive(PartialEq,Clone,Default,Debug)]
    #[derive(::gz_msgs_common::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.IMUSensor.AngularVelocity)
    pub struct AngularVelocity {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.AngularVelocity.x_noise)
        pub x_noise: ::protobuf::MessageField<super::super::sensor_noise::SensorNoise>,
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.AngularVelocity.y_noise)
        pub y_noise: ::protobuf::MessageField<super::super::sensor_noise::SensorNoise>,
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.AngularVelocity.z_noise)
        pub z_noise: ::protobuf::MessageField<super::super::sensor_noise::SensorNoise>,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.IMUSensor.AngularVelocity.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a AngularVelocity {
        fn default() -> &'a AngularVelocity {
            <AngularVelocity as ::protobuf::Message>::default_instance()
        }
    }

    impl AngularVelocity {
        pub fn new() -> AngularVelocity {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(3);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::sensor_noise::SensorNoise>(
                "x_noise",
                |m: &AngularVelocity| { &m.x_noise },
                |m: &mut AngularVelocity| { &mut m.x_noise },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::sensor_noise::SensorNoise>(
                "y_noise",
                |m: &AngularVelocity| { &m.y_noise },
                |m: &mut AngularVelocity| { &mut m.y_noise },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::sensor_noise::SensorNoise>(
                "z_noise",
                |m: &AngularVelocity| { &m.z_noise },
                |m: &mut AngularVelocity| { &mut m.z_noise },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AngularVelocity>(
                "IMUSensor.AngularVelocity",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for AngularVelocity {
        const NAME: &'static str = "AngularVelocity";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.x_noise)?;
                    },
                    18 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.y_noise)?;
                    },
                    26 => {
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
            if let Some(v) = self.x_noise.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
            }
            if let Some(v) = self.y_noise.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
            }
            if let Some(v) = self.z_noise.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

        fn new() -> AngularVelocity {
            AngularVelocity::new()
        }

        fn clear(&mut self) {
            self.x_noise.clear();
            self.y_noise.clear();
            self.z_noise.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static AngularVelocity {
            static instance: AngularVelocity = AngularVelocity {
                x_noise: ::protobuf::MessageField::none(),
                y_noise: ::protobuf::MessageField::none(),
                z_noise: ::protobuf::MessageField::none(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for AngularVelocity {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("IMUSensor.AngularVelocity").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for AngularVelocity {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for AngularVelocity {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    #[derive(PartialEq,Clone,Default,Debug)]
    #[derive(::gz_msgs_common::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.IMUSensor.LinearAcceleration)
    pub struct LinearAcceleration {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.LinearAcceleration.x_noise)
        pub x_noise: ::protobuf::MessageField<super::super::sensor_noise::SensorNoise>,
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.LinearAcceleration.y_noise)
        pub y_noise: ::protobuf::MessageField<super::super::sensor_noise::SensorNoise>,
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.LinearAcceleration.z_noise)
        pub z_noise: ::protobuf::MessageField<super::super::sensor_noise::SensorNoise>,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.IMUSensor.LinearAcceleration.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a LinearAcceleration {
        fn default() -> &'a LinearAcceleration {
            <LinearAcceleration as ::protobuf::Message>::default_instance()
        }
    }

    impl LinearAcceleration {
        pub fn new() -> LinearAcceleration {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(3);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::sensor_noise::SensorNoise>(
                "x_noise",
                |m: &LinearAcceleration| { &m.x_noise },
                |m: &mut LinearAcceleration| { &mut m.x_noise },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::sensor_noise::SensorNoise>(
                "y_noise",
                |m: &LinearAcceleration| { &m.y_noise },
                |m: &mut LinearAcceleration| { &mut m.y_noise },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::sensor_noise::SensorNoise>(
                "z_noise",
                |m: &LinearAcceleration| { &m.z_noise },
                |m: &mut LinearAcceleration| { &mut m.z_noise },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LinearAcceleration>(
                "IMUSensor.LinearAcceleration",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for LinearAcceleration {
        const NAME: &'static str = "LinearAcceleration";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.x_noise)?;
                    },
                    18 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.y_noise)?;
                    },
                    26 => {
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
            if let Some(v) = self.x_noise.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
            }
            if let Some(v) = self.y_noise.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
            }
            if let Some(v) = self.z_noise.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

        fn new() -> LinearAcceleration {
            LinearAcceleration::new()
        }

        fn clear(&mut self) {
            self.x_noise.clear();
            self.y_noise.clear();
            self.z_noise.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static LinearAcceleration {
            static instance: LinearAcceleration = LinearAcceleration {
                x_noise: ::protobuf::MessageField::none(),
                y_noise: ::protobuf::MessageField::none(),
                z_noise: ::protobuf::MessageField::none(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for LinearAcceleration {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("IMUSensor.LinearAcceleration").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for LinearAcceleration {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for LinearAcceleration {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    #[derive(PartialEq,Clone,Default,Debug)]
    #[derive(::gz_msgs_common::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.IMUSensor.OrientationReferenceFrame)
    pub struct OrientationReferenceFrame {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.OrientationReferenceFrame.localization)
        pub localization: ::std::string::String,
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.OrientationReferenceFrame.custom_rpy)
        pub custom_rpy: ::protobuf::MessageField<super::super::vector3d::Vector3d>,
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.OrientationReferenceFrame.custom_rpy_parent_frame)
        pub custom_rpy_parent_frame: ::std::string::String,
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.OrientationReferenceFrame.gravity_dir_x)
        pub gravity_dir_x: ::protobuf::MessageField<super::super::vector3d::Vector3d>,
        // @@protoc_insertion_point(field:ignition.msgs.IMUSensor.OrientationReferenceFrame.gravity_dir_x_parent_frame)
        pub gravity_dir_x_parent_frame: ::std::string::String,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.IMUSensor.OrientationReferenceFrame.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a OrientationReferenceFrame {
        fn default() -> &'a OrientationReferenceFrame {
            <OrientationReferenceFrame as ::protobuf::Message>::default_instance()
        }
    }

    impl OrientationReferenceFrame {
        pub fn new() -> OrientationReferenceFrame {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(5);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "localization",
                |m: &OrientationReferenceFrame| { &m.localization },
                |m: &mut OrientationReferenceFrame| { &mut m.localization },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::vector3d::Vector3d>(
                "custom_rpy",
                |m: &OrientationReferenceFrame| { &m.custom_rpy },
                |m: &mut OrientationReferenceFrame| { &mut m.custom_rpy },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "custom_rpy_parent_frame",
                |m: &OrientationReferenceFrame| { &m.custom_rpy_parent_frame },
                |m: &mut OrientationReferenceFrame| { &mut m.custom_rpy_parent_frame },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::vector3d::Vector3d>(
                "gravity_dir_x",
                |m: &OrientationReferenceFrame| { &m.gravity_dir_x },
                |m: &mut OrientationReferenceFrame| { &mut m.gravity_dir_x },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "gravity_dir_x_parent_frame",
                |m: &OrientationReferenceFrame| { &m.gravity_dir_x_parent_frame },
                |m: &mut OrientationReferenceFrame| { &mut m.gravity_dir_x_parent_frame },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<OrientationReferenceFrame>(
                "IMUSensor.OrientationReferenceFrame",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for OrientationReferenceFrame {
        const NAME: &'static str = "OrientationReferenceFrame";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.localization = is.read_string()?;
                    },
                    18 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.custom_rpy)?;
                    },
                    26 => {
                        self.custom_rpy_parent_frame = is.read_string()?;
                    },
                    34 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.gravity_dir_x)?;
                    },
                    42 => {
                        self.gravity_dir_x_parent_frame = is.read_string()?;
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
            if !self.localization.is_empty() {
                my_size += ::protobuf::rt::string_size(1, &self.localization);
            }
            if let Some(v) = self.custom_rpy.as_ref() {
                let len = v.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            }
            if !self.custom_rpy_parent_frame.is_empty() {
                my_size += ::protobuf::rt::string_size(3, &self.custom_rpy_parent_frame);
            }
            if let Some(v) = self.gravity_dir_x.as_ref() {
                let len = v.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            }
            if !self.gravity_dir_x_parent_frame.is_empty() {
                my_size += ::protobuf::rt::string_size(5, &self.gravity_dir_x_parent_frame);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if !self.localization.is_empty() {
                os.write_string(1, &self.localization)?;
            }
            if let Some(v) = self.custom_rpy.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
            }
            if !self.custom_rpy_parent_frame.is_empty() {
                os.write_string(3, &self.custom_rpy_parent_frame)?;
            }
            if let Some(v) = self.gravity_dir_x.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
            }
            if !self.gravity_dir_x_parent_frame.is_empty() {
                os.write_string(5, &self.gravity_dir_x_parent_frame)?;
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

        fn new() -> OrientationReferenceFrame {
            OrientationReferenceFrame::new()
        }

        fn clear(&mut self) {
            self.localization.clear();
            self.custom_rpy.clear();
            self.custom_rpy_parent_frame.clear();
            self.gravity_dir_x.clear();
            self.gravity_dir_x_parent_frame.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static OrientationReferenceFrame {
            static instance: OrientationReferenceFrame = OrientationReferenceFrame {
                localization: ::std::string::String::new(),
                custom_rpy: ::protobuf::MessageField::none(),
                custom_rpy_parent_frame: ::std::string::String::new(),
                gravity_dir_x: ::protobuf::MessageField::none(),
                gravity_dir_x_parent_frame: ::std::string::String::new(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for OrientationReferenceFrame {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("IMUSensor.OrientationReferenceFrame").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for OrientationReferenceFrame {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for OrientationReferenceFrame {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eignition/msgs/imu_sensor.proto\x12\rignition.msgs\x1a\x20ignition/\
    msgs/sensor_noise.proto\x1a\x1aignition/msgs/header.proto\x1a\x1cignitio\
    n/msgs/vector3d.proto\"\xe8\x07\n\tIMUSensor\x12-\n\x06header\x18\x01\
    \x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\x12S\n\x10angular_vel\
    ocity\x18\x02\x20\x01(\x0b2(.ignition.msgs.IMUSensor.AngularVelocityR\
    \x0fangularVelocity\x12\\\n\x13linear_acceleration\x18\x03\x20\x01(\x0b2\
    +.ignition.msgs.IMUSensor.LinearAccelerationR\x12linearAcceleration\x12f\
    \n\x15orientation_ref_frame\x18\x04\x20\x01(\x0b22.ignition.msgs.IMUSens\
    or.OrientationReferenceFrameR\x13orientationRefFrame\x1a\xb0\x01\n\x0fAn\
    gularVelocity\x123\n\x07x_noise\x18\x01\x20\x01(\x0b2\x1a.ignition.msgs.\
    SensorNoiseR\x06xNoise\x123\n\x07y_noise\x18\x02\x20\x01(\x0b2\x1a.ignit\
    ion.msgs.SensorNoiseR\x06yNoise\x123\n\x07z_noise\x18\x03\x20\x01(\x0b2\
    \x1a.ignition.msgs.SensorNoiseR\x06zNoise\x1a\xb3\x01\n\x12LinearAcceler\
    ation\x123\n\x07x_noise\x18\x01\x20\x01(\x0b2\x1a.ignition.msgs.SensorNo\
    iseR\x06xNoise\x123\n\x07y_noise\x18\x02\x20\x01(\x0b2\x1a.ignition.msgs\
    .SensorNoiseR\x06yNoise\x123\n\x07z_noise\x18\x03\x20\x01(\x0b2\x1a.igni\
    tion.msgs.SensorNoiseR\x06zNoise\x1a\xa7\x02\n\x19OrientationReferenceFr\
    ame\x12\"\n\x0clocalization\x18\x01\x20\x01(\tR\x0clocalization\x126\n\n\
    custom_rpy\x18\x02\x20\x01(\x0b2\x17.ignition.msgs.Vector3dR\tcustomRpy\
    \x125\n\x17custom_rpy_parent_frame\x18\x03\x20\x01(\tR\x14customRpyParen\
    tFrame\x12;\n\rgravity_dir_x\x18\x04\x20\x01(\x0b2\x17.ignition.msgs.Vec\
    tor3dR\x0bgravityDirX\x12:\n\x1agravity_dir_x_parent_frame\x18\x05\x20\
    \x01(\tR\x16gravityDirXParentFrameB$\n\x11com.ignition.msgsB\x0fIMUSenso\
    rProtosb\x06proto3\
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
            deps.push(super::sensor_noise::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::vector3d::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(IMUSensor::generated_message_descriptor_data());
            messages.push(imusensor::AngularVelocity::generated_message_descriptor_data());
            messages.push(imusensor::LinearAcceleration::generated_message_descriptor_data());
            messages.push(imusensor::OrientationReferenceFrame::generated_message_descriptor_data());
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
