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

//! Generated file from `gz/msgs/sensor.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Sensor)
pub struct Sensor {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Sensor.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.id)
    pub id: u32,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.parent)
    pub parent: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.parent_id)
    pub parent_id: u32,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.type)
    pub type_: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.always_on)
    pub always_on: bool,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.update_rate)
    pub update_rate: f64,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.camera)
    pub camera: ::protobuf::MessageField<super::camerasensor::CameraSensor>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.contact)
    pub contact: ::protobuf::MessageField<super::contactsensor::ContactSensor>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.visualize)
    pub visualize: bool,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.topic)
    pub topic: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.logical_camera)
    pub logical_camera: ::protobuf::MessageField<super::logical_camera_sensor::LogicalCameraSensor>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.gps)
    pub gps: ::protobuf::MessageField<super::gps_sensor::GPSSensor>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.imu)
    pub imu: ::protobuf::MessageField<super::imu_sensor::IMUSensor>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.magnetometer)
    pub magnetometer: ::protobuf::MessageField<super::magnetometer_sensor::MagnetometerSensor>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.altimeter)
    pub altimeter: ::protobuf::MessageField<super::altimeter_sensor::AltimeterSensor>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.air_pressure)
    pub air_pressure: ::protobuf::MessageField<super::air_pressure_sensor::AirPressureSensor>,
    // @@protoc_insertion_point(field:gz.msgs.Sensor.lidar)
    pub lidar: ::protobuf::MessageField<super::lidar_sensor::LidarSensor>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Sensor.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Sensor {
    fn default() -> &'a Sensor {
        <Sensor as ::protobuf::Message>::default_instance()
    }
}

impl Sensor {
    pub fn new() -> Sensor {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(20);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Sensor| { &m.header },
            |m: &mut Sensor| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Sensor| { &m.name },
            |m: &mut Sensor| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Sensor| { &m.id },
            |m: &mut Sensor| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "parent",
            |m: &Sensor| { &m.parent },
            |m: &mut Sensor| { &mut m.parent },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "parent_id",
            |m: &Sensor| { &m.parent_id },
            |m: &mut Sensor| { &mut m.parent_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Sensor| { &m.type_ },
            |m: &mut Sensor| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "always_on",
            |m: &Sensor| { &m.always_on },
            |m: &mut Sensor| { &mut m.always_on },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "update_rate",
            |m: &Sensor| { &m.update_rate },
            |m: &mut Sensor| { &mut m.update_rate },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &Sensor| { &m.pose },
            |m: &mut Sensor| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::camerasensor::CameraSensor>(
            "camera",
            |m: &Sensor| { &m.camera },
            |m: &mut Sensor| { &mut m.camera },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::contactsensor::ContactSensor>(
            "contact",
            |m: &Sensor| { &m.contact },
            |m: &mut Sensor| { &mut m.contact },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "visualize",
            |m: &Sensor| { &m.visualize },
            |m: &mut Sensor| { &mut m.visualize },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "topic",
            |m: &Sensor| { &m.topic },
            |m: &mut Sensor| { &mut m.topic },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::logical_camera_sensor::LogicalCameraSensor>(
            "logical_camera",
            |m: &Sensor| { &m.logical_camera },
            |m: &mut Sensor| { &mut m.logical_camera },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::gps_sensor::GPSSensor>(
            "gps",
            |m: &Sensor| { &m.gps },
            |m: &mut Sensor| { &mut m.gps },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::imu_sensor::IMUSensor>(
            "imu",
            |m: &Sensor| { &m.imu },
            |m: &mut Sensor| { &mut m.imu },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::magnetometer_sensor::MagnetometerSensor>(
            "magnetometer",
            |m: &Sensor| { &m.magnetometer },
            |m: &mut Sensor| { &mut m.magnetometer },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::altimeter_sensor::AltimeterSensor>(
            "altimeter",
            |m: &Sensor| { &m.altimeter },
            |m: &mut Sensor| { &mut m.altimeter },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::air_pressure_sensor::AirPressureSensor>(
            "air_pressure",
            |m: &Sensor| { &m.air_pressure },
            |m: &mut Sensor| { &mut m.air_pressure },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::lidar_sensor::LidarSensor>(
            "lidar",
            |m: &Sensor| { &m.lidar },
            |m: &mut Sensor| { &mut m.lidar },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Sensor>(
            "Sensor",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Sensor {
    const NAME: &'static str = "Sensor";

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
                    self.name = is.read_string()?;
                },
                24 => {
                    self.id = is.read_uint32()?;
                },
                34 => {
                    self.parent = is.read_string()?;
                },
                40 => {
                    self.parent_id = is.read_uint32()?;
                },
                50 => {
                    self.type_ = is.read_string()?;
                },
                56 => {
                    self.always_on = is.read_bool()?;
                },
                65 => {
                    self.update_rate = is.read_double()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.camera)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.contact)?;
                },
                96 => {
                    self.visualize = is.read_bool()?;
                },
                106 => {
                    self.topic = is.read_string()?;
                },
                114 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.logical_camera)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.gps)?;
                },
                130 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.imu)?;
                },
                138 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.magnetometer)?;
                },
                146 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.altimeter)?;
                },
                154 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.air_pressure)?;
                },
                162 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.lidar)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.id);
        }
        if !self.parent.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.parent);
        }
        if self.parent_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.parent_id);
        }
        if !self.type_.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.type_);
        }
        if self.always_on != false {
            my_size += 1 + 1;
        }
        if self.update_rate != 0. {
            my_size += 1 + 8;
        }
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.camera.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.contact.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.visualize != false {
            my_size += 1 + 1;
        }
        if !self.topic.is_empty() {
            my_size += ::protobuf::rt::string_size(13, &self.topic);
        }
        if let Some(v) = self.logical_camera.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.gps.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.imu.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.magnetometer.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.altimeter.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.air_pressure.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.lidar.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if self.id != 0 {
            os.write_uint32(3, self.id)?;
        }
        if !self.parent.is_empty() {
            os.write_string(4, &self.parent)?;
        }
        if self.parent_id != 0 {
            os.write_uint32(5, self.parent_id)?;
        }
        if !self.type_.is_empty() {
            os.write_string(6, &self.type_)?;
        }
        if self.always_on != false {
            os.write_bool(7, self.always_on)?;
        }
        if self.update_rate != 0. {
            os.write_double(8, self.update_rate)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.camera.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.contact.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.visualize != false {
            os.write_bool(12, self.visualize)?;
        }
        if !self.topic.is_empty() {
            os.write_string(13, &self.topic)?;
        }
        if let Some(v) = self.logical_camera.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(14, v, os)?;
        }
        if let Some(v) = self.gps.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if let Some(v) = self.imu.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(16, v, os)?;
        }
        if let Some(v) = self.magnetometer.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(17, v, os)?;
        }
        if let Some(v) = self.altimeter.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(18, v, os)?;
        }
        if let Some(v) = self.air_pressure.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(19, v, os)?;
        }
        if let Some(v) = self.lidar.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(20, v, os)?;
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

    fn new() -> Sensor {
        Sensor::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.name.clear();
        self.id = 0;
        self.parent.clear();
        self.parent_id = 0;
        self.type_.clear();
        self.always_on = false;
        self.update_rate = 0.;
        self.pose.clear();
        self.camera.clear();
        self.contact.clear();
        self.visualize = false;
        self.topic.clear();
        self.logical_camera.clear();
        self.gps.clear();
        self.imu.clear();
        self.magnetometer.clear();
        self.altimeter.clear();
        self.air_pressure.clear();
        self.lidar.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Sensor {
        static instance: Sensor = Sensor {
            header: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            id: 0,
            parent: ::std::string::String::new(),
            parent_id: 0,
            type_: ::std::string::String::new(),
            always_on: false,
            update_rate: 0.,
            pose: ::protobuf::MessageField::none(),
            camera: ::protobuf::MessageField::none(),
            contact: ::protobuf::MessageField::none(),
            visualize: false,
            topic: ::std::string::String::new(),
            logical_camera: ::protobuf::MessageField::none(),
            gps: ::protobuf::MessageField::none(),
            imu: ::protobuf::MessageField::none(),
            magnetometer: ::protobuf::MessageField::none(),
            altimeter: ::protobuf::MessageField::none(),
            air_pressure: ::protobuf::MessageField::none(),
            lidar: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Sensor {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Sensor").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Sensor {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Sensor {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14gz/msgs/sensor.proto\x12\x07gz.msgs\x1a\x1egz/msgs/altimeter_senso\
    r.proto\x1a\x1agz/msgs/camerasensor.proto\x1a\x1bgz/msgs/contactsensor.p\
    roto\x1a!gz/msgs/air_pressure_sensor.proto\x1a\x18gz/msgs/gps_sensor.pro\
    to\x1a\x14gz/msgs/header.proto\x1a\x18gz/msgs/imu_sensor.proto\x1a\x1agz\
    /msgs/lidar_sensor.proto\x1a#gz/msgs/logical_camera_sensor.proto\x1a!gz/\
    msgs/magnetometer_sensor.proto\x1a\x12gz/msgs/pose.proto\"\x89\x06\n\x06\
    Sensor\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06hea\
    der\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x0e\n\x02id\x18\
    \x03\x20\x01(\rR\x02id\x12\x16\n\x06parent\x18\x04\x20\x01(\tR\x06parent\
    \x12\x1b\n\tparent_id\x18\x05\x20\x01(\rR\x08parentId\x12\x12\n\x04type\
    \x18\x06\x20\x01(\tR\x04type\x12\x1b\n\talways_on\x18\x07\x20\x01(\x08R\
    \x08alwaysOn\x12\x1f\n\x0bupdate_rate\x18\x08\x20\x01(\x01R\nupdateRate\
    \x12!\n\x04pose\x18\t\x20\x01(\x0b2\r.gz.msgs.PoseR\x04pose\x12-\n\x06ca\
    mera\x18\n\x20\x01(\x0b2\x15.gz.msgs.CameraSensorR\x06camera\x120\n\x07c\
    ontact\x18\x0b\x20\x01(\x0b2\x16.gz.msgs.ContactSensorR\x07contact\x12\
    \x1c\n\tvisualize\x18\x0c\x20\x01(\x08R\tvisualize\x12\x14\n\x05topic\
    \x18\r\x20\x01(\tR\x05topic\x12C\n\x0elogical_camera\x18\x0e\x20\x01(\
    \x0b2\x1c.gz.msgs.LogicalCameraSensorR\rlogicalCamera\x12$\n\x03gps\x18\
    \x0f\x20\x01(\x0b2\x12.gz.msgs.GPSSensorR\x03gps\x12$\n\x03imu\x18\x10\
    \x20\x01(\x0b2\x12.gz.msgs.IMUSensorR\x03imu\x12?\n\x0cmagnetometer\x18\
    \x11\x20\x01(\x0b2\x1b.gz.msgs.MagnetometerSensorR\x0cmagnetometer\x126\
    \n\taltimeter\x18\x12\x20\x01(\x0b2\x18.gz.msgs.AltimeterSensorR\taltime\
    ter\x12=\n\x0cair_pressure\x18\x13\x20\x01(\x0b2\x1a.gz.msgs.AirPressure\
    SensorR\x0bairPressure\x12*\n\x05lidar\x18\x14\x20\x01(\x0b2\x14.gz.msgs\
    .LidarSensorR\x05lidarB\x1b\n\x0bcom.gz.msgsB\x0cSensorProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(11);
            deps.push(super::altimeter_sensor::file_descriptor().clone());
            deps.push(super::camerasensor::file_descriptor().clone());
            deps.push(super::contactsensor::file_descriptor().clone());
            deps.push(super::air_pressure_sensor::file_descriptor().clone());
            deps.push(super::gps_sensor::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::imu_sensor::file_descriptor().clone());
            deps.push(super::lidar_sensor::file_descriptor().clone());
            deps.push(super::logical_camera_sensor::file_descriptor().clone());
            deps.push(super::magnetometer_sensor::file_descriptor().clone());
            deps.push(super::pose::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Sensor::generated_message_descriptor_data());
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
