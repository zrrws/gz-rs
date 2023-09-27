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

//! Generated file from `ignition/msgs/joint.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.Joint)
pub struct Joint {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.Joint.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.id)
    pub id: u32,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.type)
    pub type_: ::protobuf::EnumOrUnknown<joint::Type>,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.parent)
    pub parent: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.parent_id)
    pub parent_id: u32,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.child)
    pub child: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.child_id)
    pub child_id: u32,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.axis1)
    pub axis1: ::protobuf::MessageField<super::axis::Axis>,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.axis2)
    pub axis2: ::protobuf::MessageField<super::axis::Axis>,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.cfm)
    pub cfm: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.bounce)
    pub bounce: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.fudge_factor)
    pub fudge_factor: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.limit_cfm)
    pub limit_cfm: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.limit_erp)
    pub limit_erp: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.suspension_cfm)
    pub suspension_cfm: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.suspension_erp)
    pub suspension_erp: f64,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.gearbox)
    pub gearbox: ::protobuf::MessageField<joint::Gearbox>,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.screw)
    pub screw: ::protobuf::MessageField<joint::Screw>,
    // @@protoc_insertion_point(field:ignition.msgs.Joint.sensor)
    pub sensor: ::std::vec::Vec<super::sensor::Sensor>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.Joint.special_fields)
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

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(21);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Joint| { &m.header },
            |m: &mut Joint| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Joint| { &m.name },
            |m: &mut Joint| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Joint| { &m.id },
            |m: &mut Joint| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Joint| { &m.type_ },
            |m: &mut Joint| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "parent",
            |m: &Joint| { &m.parent },
            |m: &mut Joint| { &mut m.parent },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "parent_id",
            |m: &Joint| { &m.parent_id },
            |m: &mut Joint| { &mut m.parent_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "child",
            |m: &Joint| { &m.child },
            |m: &mut Joint| { &mut m.child },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "child_id",
            |m: &Joint| { &m.child_id },
            |m: &mut Joint| { &mut m.child_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &Joint| { &m.pose },
            |m: &mut Joint| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::axis::Axis>(
            "axis1",
            |m: &Joint| { &m.axis1 },
            |m: &mut Joint| { &mut m.axis1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::axis::Axis>(
            "axis2",
            |m: &Joint| { &m.axis2 },
            |m: &mut Joint| { &mut m.axis2 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cfm",
            |m: &Joint| { &m.cfm },
            |m: &mut Joint| { &mut m.cfm },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "bounce",
            |m: &Joint| { &m.bounce },
            |m: &mut Joint| { &mut m.bounce },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fudge_factor",
            |m: &Joint| { &m.fudge_factor },
            |m: &mut Joint| { &mut m.fudge_factor },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "limit_cfm",
            |m: &Joint| { &m.limit_cfm },
            |m: &mut Joint| { &mut m.limit_cfm },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "limit_erp",
            |m: &Joint| { &m.limit_erp },
            |m: &mut Joint| { &mut m.limit_erp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "suspension_cfm",
            |m: &Joint| { &m.suspension_cfm },
            |m: &mut Joint| { &mut m.suspension_cfm },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "suspension_erp",
            |m: &Joint| { &m.suspension_erp },
            |m: &mut Joint| { &mut m.suspension_erp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, joint::Gearbox>(
            "gearbox",
            |m: &Joint| { &m.gearbox },
            |m: &mut Joint| { &mut m.gearbox },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, joint::Screw>(
            "screw",
            |m: &Joint| { &m.screw },
            |m: &mut Joint| { &mut m.screw },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "sensor",
            |m: &Joint| { &m.sensor },
            |m: &mut Joint| { &mut m.sensor },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Joint>(
            "Joint",
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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                18 => {
                    self.name = is.read_string()?;
                },
                24 => {
                    self.id = is.read_uint32()?;
                },
                32 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                42 => {
                    self.parent = is.read_string()?;
                },
                48 => {
                    self.parent_id = is.read_uint32()?;
                },
                58 => {
                    self.child = is.read_string()?;
                },
                64 => {
                    self.child_id = is.read_uint32()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.axis1)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.axis2)?;
                },
                97 => {
                    self.cfm = is.read_double()?;
                },
                105 => {
                    self.bounce = is.read_double()?;
                },
                113 => {
                    self.fudge_factor = is.read_double()?;
                },
                121 => {
                    self.limit_cfm = is.read_double()?;
                },
                129 => {
                    self.limit_erp = is.read_double()?;
                },
                137 => {
                    self.suspension_cfm = is.read_double()?;
                },
                145 => {
                    self.suspension_erp = is.read_double()?;
                },
                154 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.gearbox)?;
                },
                162 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.screw)?;
                },
                170 => {
                    self.sensor.push(is.read_message()?);
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
        if self.type_ != ::protobuf::EnumOrUnknown::new(joint::Type::REVOLUTE) {
            my_size += ::protobuf::rt::int32_size(4, self.type_.value());
        }
        if !self.parent.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.parent);
        }
        if self.parent_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.parent_id);
        }
        if !self.child.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.child);
        }
        if self.child_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.child_id);
        }
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.axis1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.axis2.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.cfm != 0. {
            my_size += 1 + 8;
        }
        if self.bounce != 0. {
            my_size += 1 + 8;
        }
        if self.fudge_factor != 0. {
            my_size += 1 + 8;
        }
        if self.limit_cfm != 0. {
            my_size += 1 + 8;
        }
        if self.limit_erp != 0. {
            my_size += 2 + 8;
        }
        if self.suspension_cfm != 0. {
            my_size += 2 + 8;
        }
        if self.suspension_erp != 0. {
            my_size += 2 + 8;
        }
        if let Some(v) = self.gearbox.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.screw.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.sensor {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
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
        if self.type_ != ::protobuf::EnumOrUnknown::new(joint::Type::REVOLUTE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if !self.parent.is_empty() {
            os.write_string(5, &self.parent)?;
        }
        if self.parent_id != 0 {
            os.write_uint32(6, self.parent_id)?;
        }
        if !self.child.is_empty() {
            os.write_string(7, &self.child)?;
        }
        if self.child_id != 0 {
            os.write_uint32(8, self.child_id)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.axis1.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.axis2.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.cfm != 0. {
            os.write_double(12, self.cfm)?;
        }
        if self.bounce != 0. {
            os.write_double(13, self.bounce)?;
        }
        if self.fudge_factor != 0. {
            os.write_double(14, self.fudge_factor)?;
        }
        if self.limit_cfm != 0. {
            os.write_double(15, self.limit_cfm)?;
        }
        if self.limit_erp != 0. {
            os.write_double(16, self.limit_erp)?;
        }
        if self.suspension_cfm != 0. {
            os.write_double(17, self.suspension_cfm)?;
        }
        if self.suspension_erp != 0. {
            os.write_double(18, self.suspension_erp)?;
        }
        if let Some(v) = self.gearbox.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(19, v, os)?;
        }
        if let Some(v) = self.screw.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(20, v, os)?;
        }
        for v in &self.sensor {
            ::protobuf::rt::write_message_field_with_cached_size(21, v, os)?;
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
        self.header.clear();
        self.name.clear();
        self.id = 0;
        self.type_ = ::protobuf::EnumOrUnknown::new(joint::Type::REVOLUTE);
        self.parent.clear();
        self.parent_id = 0;
        self.child.clear();
        self.child_id = 0;
        self.pose.clear();
        self.axis1.clear();
        self.axis2.clear();
        self.cfm = 0.;
        self.bounce = 0.;
        self.fudge_factor = 0.;
        self.limit_cfm = 0.;
        self.limit_erp = 0.;
        self.suspension_cfm = 0.;
        self.suspension_erp = 0.;
        self.gearbox.clear();
        self.screw.clear();
        self.sensor.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Joint {
        static instance: Joint = Joint {
            header: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            id: 0,
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            parent: ::std::string::String::new(),
            parent_id: 0,
            child: ::std::string::String::new(),
            child_id: 0,
            pose: ::protobuf::MessageField::none(),
            axis1: ::protobuf::MessageField::none(),
            axis2: ::protobuf::MessageField::none(),
            cfm: 0.,
            bounce: 0.,
            fudge_factor: 0.,
            limit_cfm: 0.,
            limit_erp: 0.,
            suspension_cfm: 0.,
            suspension_erp: 0.,
            gearbox: ::protobuf::MessageField::none(),
            screw: ::protobuf::MessageField::none(),
            sensor: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Joint {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Joint").unwrap()).clone()
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

/// Nested message and enums of message `Joint`
pub mod joint {
    #[derive(PartialEq,Clone,Default,Debug)]
    #[derive(::gz_msgs_derive::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.Joint.Gearbox)
    pub struct Gearbox {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.Joint.Gearbox.gearbox_reference_body)
        pub gearbox_reference_body: ::std::string::String,
        // @@protoc_insertion_point(field:ignition.msgs.Joint.Gearbox.gearbox_ratio)
        pub gearbox_ratio: f64,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.Joint.Gearbox.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Gearbox {
        fn default() -> &'a Gearbox {
            <Gearbox as ::protobuf::Message>::default_instance()
        }
    }

    impl Gearbox {
        pub fn new() -> Gearbox {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "gearbox_reference_body",
                |m: &Gearbox| { &m.gearbox_reference_body },
                |m: &mut Gearbox| { &mut m.gearbox_reference_body },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "gearbox_ratio",
                |m: &Gearbox| { &m.gearbox_ratio },
                |m: &mut Gearbox| { &mut m.gearbox_ratio },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Gearbox>(
                "Joint.Gearbox",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Gearbox {
        const NAME: &'static str = "Gearbox";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.gearbox_reference_body = is.read_string()?;
                    },
                    17 => {
                        self.gearbox_ratio = is.read_double()?;
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
            if !self.gearbox_reference_body.is_empty() {
                my_size += ::protobuf::rt::string_size(1, &self.gearbox_reference_body);
            }
            if self.gearbox_ratio != 0. {
                my_size += 1 + 8;
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if !self.gearbox_reference_body.is_empty() {
                os.write_string(1, &self.gearbox_reference_body)?;
            }
            if self.gearbox_ratio != 0. {
                os.write_double(2, self.gearbox_ratio)?;
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

        fn new() -> Gearbox {
            Gearbox::new()
        }

        fn clear(&mut self) {
            self.gearbox_reference_body.clear();
            self.gearbox_ratio = 0.;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Gearbox {
            static instance: Gearbox = Gearbox {
                gearbox_reference_body: ::std::string::String::new(),
                gearbox_ratio: 0.,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Gearbox {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("Joint.Gearbox").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Gearbox {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Gearbox {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    #[derive(PartialEq,Clone,Default,Debug)]
    #[derive(::gz_msgs_derive::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.Joint.Screw)
    pub struct Screw {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.Joint.Screw.thread_pitch)
        pub thread_pitch: f64,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.Joint.Screw.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Screw {
        fn default() -> &'a Screw {
            <Screw as ::protobuf::Message>::default_instance()
        }
    }

    impl Screw {
        pub fn new() -> Screw {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(1);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "thread_pitch",
                |m: &Screw| { &m.thread_pitch },
                |m: &mut Screw| { &mut m.thread_pitch },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Screw>(
                "Joint.Screw",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Screw {
        const NAME: &'static str = "Screw";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    9 => {
                        self.thread_pitch = is.read_double()?;
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
            if self.thread_pitch != 0. {
                my_size += 1 + 8;
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if self.thread_pitch != 0. {
                os.write_double(1, self.thread_pitch)?;
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

        fn new() -> Screw {
            Screw::new()
        }

        fn clear(&mut self) {
            self.thread_pitch = 0.;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Screw {
            static instance: Screw = Screw {
                thread_pitch: 0.,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Screw {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("Joint.Screw").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Screw {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Screw {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:ignition.msgs.Joint.Type)
    pub enum Type {
        // @@protoc_insertion_point(enum_value:ignition.msgs.Joint.Type.REVOLUTE)
        REVOLUTE = 0,
        // @@protoc_insertion_point(enum_value:ignition.msgs.Joint.Type.REVOLUTE2)
        REVOLUTE2 = 1,
        // @@protoc_insertion_point(enum_value:ignition.msgs.Joint.Type.PRISMATIC)
        PRISMATIC = 2,
        // @@protoc_insertion_point(enum_value:ignition.msgs.Joint.Type.UNIVERSAL)
        UNIVERSAL = 3,
        // @@protoc_insertion_point(enum_value:ignition.msgs.Joint.Type.BALL)
        BALL = 4,
        // @@protoc_insertion_point(enum_value:ignition.msgs.Joint.Type.SCREW)
        SCREW = 5,
        // @@protoc_insertion_point(enum_value:ignition.msgs.Joint.Type.GEARBOX)
        GEARBOX = 6,
        // @@protoc_insertion_point(enum_value:ignition.msgs.Joint.Type.FIXED)
        FIXED = 7,
    }

    impl ::protobuf::Enum for Type {
        const NAME: &'static str = "Type";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Type> {
            match value {
                0 => ::std::option::Option::Some(Type::REVOLUTE),
                1 => ::std::option::Option::Some(Type::REVOLUTE2),
                2 => ::std::option::Option::Some(Type::PRISMATIC),
                3 => ::std::option::Option::Some(Type::UNIVERSAL),
                4 => ::std::option::Option::Some(Type::BALL),
                5 => ::std::option::Option::Some(Type::SCREW),
                6 => ::std::option::Option::Some(Type::GEARBOX),
                7 => ::std::option::Option::Some(Type::FIXED),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Type] = &[
            Type::REVOLUTE,
            Type::REVOLUTE2,
            Type::PRISMATIC,
            Type::UNIVERSAL,
            Type::BALL,
            Type::SCREW,
            Type::GEARBOX,
            Type::FIXED,
        ];
    }

    impl ::protobuf::EnumFull for Type {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Joint.Type").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Type {
        fn default() -> Self {
            Type::REVOLUTE
        }
    }

    impl Type {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Type>("Joint.Type")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19ignition/msgs/joint.proto\x12\rignition.msgs\x1a\x1aignition/msgs/\
    header.proto\x1a\x18ignition/msgs/axis.proto\x1a\x18ignition/msgs/pose.p\
    roto\x1a\x1aignition/msgs/sensor.proto\"\xde\x07\n\x05Joint\x12-\n\x06he\
    ader\x18\x01\x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\x12\x12\n\
    \x04name\x18\x02\x20\x01(\tR\x04name\x12\x0e\n\x02id\x18\x03\x20\x01(\rR\
    \x02id\x12-\n\x04type\x18\x04\x20\x01(\x0e2\x19.ignition.msgs.Joint.Type\
    R\x04type\x12\x16\n\x06parent\x18\x05\x20\x01(\tR\x06parent\x12\x1b\n\tp\
    arent_id\x18\x06\x20\x01(\rR\x08parentId\x12\x14\n\x05child\x18\x07\x20\
    \x01(\tR\x05child\x12\x19\n\x08child_id\x18\x08\x20\x01(\rR\x07childId\
    \x12'\n\x04pose\x18\t\x20\x01(\x0b2\x13.ignition.msgs.PoseR\x04pose\x12)\
    \n\x05axis1\x18\n\x20\x01(\x0b2\x13.ignition.msgs.AxisR\x05axis1\x12)\n\
    \x05axis2\x18\x0b\x20\x01(\x0b2\x13.ignition.msgs.AxisR\x05axis2\x12\x10\
    \n\x03cfm\x18\x0c\x20\x01(\x01R\x03cfm\x12\x16\n\x06bounce\x18\r\x20\x01\
    (\x01R\x06bounce\x12!\n\x0cfudge_factor\x18\x0e\x20\x01(\x01R\x0bfudgeFa\
    ctor\x12\x1b\n\tlimit_cfm\x18\x0f\x20\x01(\x01R\x08limitCfm\x12\x1b\n\tl\
    imit_erp\x18\x10\x20\x01(\x01R\x08limitErp\x12%\n\x0esuspension_cfm\x18\
    \x11\x20\x01(\x01R\rsuspensionCfm\x12%\n\x0esuspension_erp\x18\x12\x20\
    \x01(\x01R\rsuspensionErp\x126\n\x07gearbox\x18\x13\x20\x01(\x0b2\x1c.ig\
    nition.msgs.Joint.GearboxR\x07gearbox\x120\n\x05screw\x18\x14\x20\x01(\
    \x0b2\x1a.ignition.msgs.Joint.ScrewR\x05screw\x12-\n\x06sensor\x18\x15\
    \x20\x03(\x0b2\x15.ignition.msgs.SensorR\x06sensor\x1ad\n\x07Gearbox\x12\
    4\n\x16gearbox_reference_body\x18\x01\x20\x01(\tR\x14gearboxReferenceBod\
    y\x12#\n\rgearbox_ratio\x18\x02\x20\x01(\x01R\x0cgearboxRatio\x1a*\n\x05\
    Screw\x12!\n\x0cthread_pitch\x18\x01\x20\x01(\x01R\x0bthreadPitch\"n\n\
    \x04Type\x12\x0c\n\x08REVOLUTE\x10\0\x12\r\n\tREVOLUTE2\x10\x01\x12\r\n\
    \tPRISMATIC\x10\x02\x12\r\n\tUNIVERSAL\x10\x03\x12\x08\n\x04BALL\x10\x04\
    \x12\t\n\x05SCREW\x10\x05\x12\x0b\n\x07GEARBOX\x10\x06\x12\t\n\x05FIXED\
    \x10\x07B\x20\n\x11com.ignition.msgsB\x0bJointProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::axis::file_descriptor().clone());
            deps.push(super::pose::file_descriptor().clone());
            deps.push(super::sensor::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(Joint::generated_message_descriptor_data());
            messages.push(joint::Gearbox::generated_message_descriptor_data());
            messages.push(joint::Screw::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(joint::Type::generated_enum_descriptor_data());
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
