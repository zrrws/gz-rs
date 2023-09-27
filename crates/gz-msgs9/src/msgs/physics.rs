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

//! Generated file from `gz/msgs/physics.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Physics)
pub struct Physics {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Physics.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Physics.type)
    pub type_: ::protobuf::EnumOrUnknown<physics::Type>,
    // @@protoc_insertion_point(field:gz.msgs.Physics.solver_type)
    pub solver_type: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Physics.min_step_size)
    pub min_step_size: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.precon_iters)
    pub precon_iters: i32,
    // @@protoc_insertion_point(field:gz.msgs.Physics.iters)
    pub iters: i32,
    // @@protoc_insertion_point(field:gz.msgs.Physics.sor)
    pub sor: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.cfm)
    pub cfm: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.erp)
    pub erp: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.contact_max_correcting_vel)
    pub contact_max_correcting_vel: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.contact_surface_layer)
    pub contact_surface_layer: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.gravity)
    pub gravity: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:gz.msgs.Physics.enable_physics)
    pub enable_physics: bool,
    // @@protoc_insertion_point(field:gz.msgs.Physics.real_time_factor)
    pub real_time_factor: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.real_time_update_rate)
    pub real_time_update_rate: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.max_step_size)
    pub max_step_size: f64,
    // @@protoc_insertion_point(field:gz.msgs.Physics.profile_name)
    pub profile_name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Physics.magnetic_field)
    pub magnetic_field: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Physics.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Physics {
    fn default() -> &'a Physics {
        <Physics as ::protobuf::Message>::default_instance()
    }
}

impl Physics {
    pub fn new() -> Physics {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(18);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Physics| { &m.header },
            |m: &mut Physics| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Physics| { &m.type_ },
            |m: &mut Physics| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "solver_type",
            |m: &Physics| { &m.solver_type },
            |m: &mut Physics| { &mut m.solver_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "min_step_size",
            |m: &Physics| { &m.min_step_size },
            |m: &mut Physics| { &mut m.min_step_size },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "precon_iters",
            |m: &Physics| { &m.precon_iters },
            |m: &mut Physics| { &mut m.precon_iters },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "iters",
            |m: &Physics| { &m.iters },
            |m: &mut Physics| { &mut m.iters },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sor",
            |m: &Physics| { &m.sor },
            |m: &mut Physics| { &mut m.sor },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cfm",
            |m: &Physics| { &m.cfm },
            |m: &mut Physics| { &mut m.cfm },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "erp",
            |m: &Physics| { &m.erp },
            |m: &mut Physics| { &mut m.erp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "contact_max_correcting_vel",
            |m: &Physics| { &m.contact_max_correcting_vel },
            |m: &mut Physics| { &mut m.contact_max_correcting_vel },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "contact_surface_layer",
            |m: &Physics| { &m.contact_surface_layer },
            |m: &mut Physics| { &mut m.contact_surface_layer },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "gravity",
            |m: &Physics| { &m.gravity },
            |m: &mut Physics| { &mut m.gravity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "enable_physics",
            |m: &Physics| { &m.enable_physics },
            |m: &mut Physics| { &mut m.enable_physics },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "real_time_factor",
            |m: &Physics| { &m.real_time_factor },
            |m: &mut Physics| { &mut m.real_time_factor },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "real_time_update_rate",
            |m: &Physics| { &m.real_time_update_rate },
            |m: &mut Physics| { &mut m.real_time_update_rate },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "max_step_size",
            |m: &Physics| { &m.max_step_size },
            |m: &mut Physics| { &mut m.max_step_size },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "profile_name",
            |m: &Physics| { &m.profile_name },
            |m: &mut Physics| { &mut m.profile_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "magnetic_field",
            |m: &Physics| { &m.magnetic_field },
            |m: &mut Physics| { &mut m.magnetic_field },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Physics>(
            "Physics",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Physics {
    const NAME: &'static str = "Physics";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                16 => {
                    self.type_ = is.read_enum_or_unknown()?;
                },
                26 => {
                    self.solver_type = is.read_string()?;
                },
                33 => {
                    self.min_step_size = is.read_double()?;
                },
                40 => {
                    self.precon_iters = is.read_int32()?;
                },
                48 => {
                    self.iters = is.read_int32()?;
                },
                57 => {
                    self.sor = is.read_double()?;
                },
                65 => {
                    self.cfm = is.read_double()?;
                },
                73 => {
                    self.erp = is.read_double()?;
                },
                81 => {
                    self.contact_max_correcting_vel = is.read_double()?;
                },
                89 => {
                    self.contact_surface_layer = is.read_double()?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.gravity)?;
                },
                104 => {
                    self.enable_physics = is.read_bool()?;
                },
                113 => {
                    self.real_time_factor = is.read_double()?;
                },
                121 => {
                    self.real_time_update_rate = is.read_double()?;
                },
                129 => {
                    self.max_step_size = is.read_double()?;
                },
                138 => {
                    self.profile_name = is.read_string()?;
                },
                146 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.magnetic_field)?;
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
        if self.type_ != ::protobuf::EnumOrUnknown::new(physics::Type::ODE) {
            my_size += ::protobuf::rt::int32_size(2, self.type_.value());
        }
        if !self.solver_type.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.solver_type);
        }
        if self.min_step_size != 0. {
            my_size += 1 + 8;
        }
        if self.precon_iters != 0 {
            my_size += ::protobuf::rt::int32_size(5, self.precon_iters);
        }
        if self.iters != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.iters);
        }
        if self.sor != 0. {
            my_size += 1 + 8;
        }
        if self.cfm != 0. {
            my_size += 1 + 8;
        }
        if self.erp != 0. {
            my_size += 1 + 8;
        }
        if self.contact_max_correcting_vel != 0. {
            my_size += 1 + 8;
        }
        if self.contact_surface_layer != 0. {
            my_size += 1 + 8;
        }
        if let Some(v) = self.gravity.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.enable_physics != false {
            my_size += 1 + 1;
        }
        if self.real_time_factor != 0. {
            my_size += 1 + 8;
        }
        if self.real_time_update_rate != 0. {
            my_size += 1 + 8;
        }
        if self.max_step_size != 0. {
            my_size += 2 + 8;
        }
        if !self.profile_name.is_empty() {
            my_size += ::protobuf::rt::string_size(17, &self.profile_name);
        }
        if let Some(v) = self.magnetic_field.as_ref() {
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
        if self.type_ != ::protobuf::EnumOrUnknown::new(physics::Type::ODE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.type_))?;
        }
        if !self.solver_type.is_empty() {
            os.write_string(3, &self.solver_type)?;
        }
        if self.min_step_size != 0. {
            os.write_double(4, self.min_step_size)?;
        }
        if self.precon_iters != 0 {
            os.write_int32(5, self.precon_iters)?;
        }
        if self.iters != 0 {
            os.write_int32(6, self.iters)?;
        }
        if self.sor != 0. {
            os.write_double(7, self.sor)?;
        }
        if self.cfm != 0. {
            os.write_double(8, self.cfm)?;
        }
        if self.erp != 0. {
            os.write_double(9, self.erp)?;
        }
        if self.contact_max_correcting_vel != 0. {
            os.write_double(10, self.contact_max_correcting_vel)?;
        }
        if self.contact_surface_layer != 0. {
            os.write_double(11, self.contact_surface_layer)?;
        }
        if let Some(v) = self.gravity.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.enable_physics != false {
            os.write_bool(13, self.enable_physics)?;
        }
        if self.real_time_factor != 0. {
            os.write_double(14, self.real_time_factor)?;
        }
        if self.real_time_update_rate != 0. {
            os.write_double(15, self.real_time_update_rate)?;
        }
        if self.max_step_size != 0. {
            os.write_double(16, self.max_step_size)?;
        }
        if !self.profile_name.is_empty() {
            os.write_string(17, &self.profile_name)?;
        }
        if let Some(v) = self.magnetic_field.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(18, v, os)?;
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

    fn new() -> Physics {
        Physics::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.type_ = ::protobuf::EnumOrUnknown::new(physics::Type::ODE);
        self.solver_type.clear();
        self.min_step_size = 0.;
        self.precon_iters = 0;
        self.iters = 0;
        self.sor = 0.;
        self.cfm = 0.;
        self.erp = 0.;
        self.contact_max_correcting_vel = 0.;
        self.contact_surface_layer = 0.;
        self.gravity.clear();
        self.enable_physics = false;
        self.real_time_factor = 0.;
        self.real_time_update_rate = 0.;
        self.max_step_size = 0.;
        self.profile_name.clear();
        self.magnetic_field.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Physics {
        static instance: Physics = Physics {
            header: ::protobuf::MessageField::none(),
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            solver_type: ::std::string::String::new(),
            min_step_size: 0.,
            precon_iters: 0,
            iters: 0,
            sor: 0.,
            cfm: 0.,
            erp: 0.,
            contact_max_correcting_vel: 0.,
            contact_surface_layer: 0.,
            gravity: ::protobuf::MessageField::none(),
            enable_physics: false,
            real_time_factor: 0.,
            real_time_update_rate: 0.,
            max_step_size: 0.,
            profile_name: ::std::string::String::new(),
            magnetic_field: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Physics {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Physics").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Physics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Physics {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Physics`
pub mod physics {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.Physics.Type)
    pub enum Type {
        // @@protoc_insertion_point(enum_value:gz.msgs.Physics.Type.ODE)
        ODE = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.Physics.Type.BULLET)
        BULLET = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.Physics.Type.SIMBODY)
        SIMBODY = 2,
        // @@protoc_insertion_point(enum_value:gz.msgs.Physics.Type.DART)
        DART = 3,
    }

    impl ::protobuf::Enum for Type {
        const NAME: &'static str = "Type";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Type> {
            match value {
                0 => ::std::option::Option::Some(Type::ODE),
                1 => ::std::option::Option::Some(Type::BULLET),
                2 => ::std::option::Option::Some(Type::SIMBODY),
                3 => ::std::option::Option::Some(Type::DART),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Type] = &[
            Type::ODE,
            Type::BULLET,
            Type::SIMBODY,
            Type::DART,
        ];
    }

    impl ::protobuf::EnumFull for Type {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Physics.Type").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Type {
        fn default() -> Self {
            Type::ODE
        }
    }

    impl Type {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Type>("Physics.Type")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15gz/msgs/physics.proto\x12\x07gz.msgs\x1a\x16gz/msgs/vector3d.proto\
    \x1a\x14gz/msgs/header.proto\"\xe8\x05\n\x07Physics\x12'\n\x06header\x18\
    \x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12)\n\x04type\x18\x02\
    \x20\x01(\x0e2\x15.gz.msgs.Physics.TypeR\x04type\x12\x1f\n\x0bsolver_typ\
    e\x18\x03\x20\x01(\tR\nsolverType\x12\"\n\rmin_step_size\x18\x04\x20\x01\
    (\x01R\x0bminStepSize\x12!\n\x0cprecon_iters\x18\x05\x20\x01(\x05R\x0bpr\
    econIters\x12\x14\n\x05iters\x18\x06\x20\x01(\x05R\x05iters\x12\x10\n\
    \x03sor\x18\x07\x20\x01(\x01R\x03sor\x12\x10\n\x03cfm\x18\x08\x20\x01(\
    \x01R\x03cfm\x12\x10\n\x03erp\x18\t\x20\x01(\x01R\x03erp\x12;\n\x1aconta\
    ct_max_correcting_vel\x18\n\x20\x01(\x01R\x17contactMaxCorrectingVel\x12\
    2\n\x15contact_surface_layer\x18\x0b\x20\x01(\x01R\x13contactSurfaceLaye\
    r\x12+\n\x07gravity\x18\x0c\x20\x01(\x0b2\x11.gz.msgs.Vector3dR\x07gravi\
    ty\x12%\n\x0eenable_physics\x18\r\x20\x01(\x08R\renablePhysics\x12(\n\
    \x10real_time_factor\x18\x0e\x20\x01(\x01R\x0erealTimeFactor\x121\n\x15r\
    eal_time_update_rate\x18\x0f\x20\x01(\x01R\x12realTimeUpdateRate\x12\"\n\
    \rmax_step_size\x18\x10\x20\x01(\x01R\x0bmaxStepSize\x12!\n\x0cprofile_n\
    ame\x18\x11\x20\x01(\tR\x0bprofileName\x128\n\x0emagnetic_field\x18\x12\
    \x20\x01(\x0b2\x11.gz.msgs.Vector3dR\rmagneticField\"2\n\x04Type\x12\x07\
    \n\x03ODE\x10\0\x12\n\n\x06BULLET\x10\x01\x12\x0b\n\x07SIMBODY\x10\x02\
    \x12\x08\n\x04DART\x10\x03B\x1c\n\x0bcom.gz.msgsB\rPhysicsProtosb\x06pro\
    to3\
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
            deps.push(super::vector3d::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Physics::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(physics::Type::generated_enum_descriptor_data());
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
