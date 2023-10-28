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

//! Generated file from `gz/msgs/visual.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Visual)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Visual {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Visual.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Visual.name)
    pub name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Visual.id)
    pub id: u32,
    // @@protoc_insertion_point(field:gz.msgs.Visual.parent_name)
    pub parent_name: ::std::string::String,
    // @@protoc_insertion_point(field:gz.msgs.Visual.parent_id)
    pub parent_id: u32,
    // @@protoc_insertion_point(field:gz.msgs.Visual.cast_shadows)
    pub cast_shadows: bool,
    // @@protoc_insertion_point(field:gz.msgs.Visual.transparency)
    pub transparency: f64,
    // @@protoc_insertion_point(field:gz.msgs.Visual.laser_retro)
    pub laser_retro: f64,
    // @@protoc_insertion_point(field:gz.msgs.Visual.pose)
    pub pose: ::protobuf::MessageField<super::pose::Pose>,
    // @@protoc_insertion_point(field:gz.msgs.Visual.geometry)
    pub geometry: ::protobuf::MessageField<super::geometry::Geometry>,
    // @@protoc_insertion_point(field:gz.msgs.Visual.material)
    pub material: ::protobuf::MessageField<super::material::Material>,
    // @@protoc_insertion_point(field:gz.msgs.Visual.visible)
    pub visible: bool,
    // @@protoc_insertion_point(field:gz.msgs.Visual.delete_me)
    pub delete_me: bool,
    // @@protoc_insertion_point(field:gz.msgs.Visual.is_static)
    pub is_static: bool,
    // @@protoc_insertion_point(field:gz.msgs.Visual.plugin)
    pub plugin: ::std::vec::Vec<super::plugin::Plugin>,
    // @@protoc_insertion_point(field:gz.msgs.Visual.scale)
    pub scale: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:gz.msgs.Visual.meta)
    pub meta: ::protobuf::MessageField<visual::Meta>,
    // @@protoc_insertion_point(field:gz.msgs.Visual.type)
    pub type_: ::protobuf::EnumOrUnknown<visual::Type>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Visual.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Visual {
    fn default() -> &'a Visual {
        <Visual as ::protobuf::Message>::default_instance()
    }
}

impl Visual {
    pub fn new() -> Visual {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(18);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Visual| { &m.header },
            |m: &mut Visual| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "name",
            |m: &Visual| { &m.name },
            |m: &mut Visual| { &mut m.name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &Visual| { &m.id },
            |m: &mut Visual| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "parent_name",
            |m: &Visual| { &m.parent_name },
            |m: &mut Visual| { &mut m.parent_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "parent_id",
            |m: &Visual| { &m.parent_id },
            |m: &mut Visual| { &mut m.parent_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cast_shadows",
            |m: &Visual| { &m.cast_shadows },
            |m: &mut Visual| { &mut m.cast_shadows },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "transparency",
            |m: &Visual| { &m.transparency },
            |m: &mut Visual| { &mut m.transparency },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "laser_retro",
            |m: &Visual| { &m.laser_retro },
            |m: &mut Visual| { &mut m.laser_retro },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::pose::Pose>(
            "pose",
            |m: &Visual| { &m.pose },
            |m: &mut Visual| { &mut m.pose },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::geometry::Geometry>(
            "geometry",
            |m: &Visual| { &m.geometry },
            |m: &mut Visual| { &mut m.geometry },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::material::Material>(
            "material",
            |m: &Visual| { &m.material },
            |m: &mut Visual| { &mut m.material },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "visible",
            |m: &Visual| { &m.visible },
            |m: &mut Visual| { &mut m.visible },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "delete_me",
            |m: &Visual| { &m.delete_me },
            |m: &mut Visual| { &mut m.delete_me },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_static",
            |m: &Visual| { &m.is_static },
            |m: &mut Visual| { &mut m.is_static },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "plugin",
            |m: &Visual| { &m.plugin },
            |m: &mut Visual| { &mut m.plugin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "scale",
            |m: &Visual| { &m.scale },
            |m: &mut Visual| { &mut m.scale },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, visual::Meta>(
            "meta",
            |m: &Visual| { &m.meta },
            |m: &mut Visual| { &mut m.meta },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &Visual| { &m.type_ },
            |m: &mut Visual| { &mut m.type_ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Visual>(
            "Visual",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Visual {
    const NAME: &'static str = "Visual";

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
                    self.parent_name = is.read_string()?;
                },
                40 => {
                    self.parent_id = is.read_uint32()?;
                },
                48 => {
                    self.cast_shadows = is.read_bool()?;
                },
                57 => {
                    self.transparency = is.read_double()?;
                },
                65 => {
                    self.laser_retro = is.read_double()?;
                },
                74 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.pose)?;
                },
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.geometry)?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.material)?;
                },
                96 => {
                    self.visible = is.read_bool()?;
                },
                104 => {
                    self.delete_me = is.read_bool()?;
                },
                112 => {
                    self.is_static = is.read_bool()?;
                },
                122 => {
                    self.plugin.push(is.read_message()?);
                },
                130 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.scale)?;
                },
                138 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.meta)?;
                },
                144 => {
                    self.type_ = is.read_enum_or_unknown()?;
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
        if !self.parent_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.parent_name);
        }
        if self.parent_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.parent_id);
        }
        if self.cast_shadows != false {
            my_size += 1 + 1;
        }
        if self.transparency != 0. {
            my_size += 1 + 8;
        }
        if self.laser_retro != 0. {
            my_size += 1 + 8;
        }
        if let Some(v) = self.pose.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.geometry.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.material.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.visible != false {
            my_size += 1 + 1;
        }
        if self.delete_me != false {
            my_size += 1 + 1;
        }
        if self.is_static != false {
            my_size += 1 + 1;
        }
        for value in &self.plugin {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.scale.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.meta.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(visual::Type::ENTITY) {
            my_size += ::protobuf::rt::int32_size(18, self.type_.value());
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
        if !self.parent_name.is_empty() {
            os.write_string(4, &self.parent_name)?;
        }
        if self.parent_id != 0 {
            os.write_uint32(5, self.parent_id)?;
        }
        if self.cast_shadows != false {
            os.write_bool(6, self.cast_shadows)?;
        }
        if self.transparency != 0. {
            os.write_double(7, self.transparency)?;
        }
        if self.laser_retro != 0. {
            os.write_double(8, self.laser_retro)?;
        }
        if let Some(v) = self.pose.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        }
        if let Some(v) = self.geometry.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.material.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        }
        if self.visible != false {
            os.write_bool(12, self.visible)?;
        }
        if self.delete_me != false {
            os.write_bool(13, self.delete_me)?;
        }
        if self.is_static != false {
            os.write_bool(14, self.is_static)?;
        }
        for v in &self.plugin {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if let Some(v) = self.scale.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(16, v, os)?;
        }
        if let Some(v) = self.meta.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(17, v, os)?;
        }
        if self.type_ != ::protobuf::EnumOrUnknown::new(visual::Type::ENTITY) {
            os.write_enum(18, ::protobuf::EnumOrUnknown::value(&self.type_))?;
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

    fn new() -> Visual {
        Visual::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.name.clear();
        self.id = 0;
        self.parent_name.clear();
        self.parent_id = 0;
        self.cast_shadows = false;
        self.transparency = 0.;
        self.laser_retro = 0.;
        self.pose.clear();
        self.geometry.clear();
        self.material.clear();
        self.visible = false;
        self.delete_me = false;
        self.is_static = false;
        self.plugin.clear();
        self.scale.clear();
        self.meta.clear();
        self.type_ = ::protobuf::EnumOrUnknown::new(visual::Type::ENTITY);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Visual {
        static instance: Visual = Visual {
            header: ::protobuf::MessageField::none(),
            name: ::std::string::String::new(),
            id: 0,
            parent_name: ::std::string::String::new(),
            parent_id: 0,
            cast_shadows: false,
            transparency: 0.,
            laser_retro: 0.,
            pose: ::protobuf::MessageField::none(),
            geometry: ::protobuf::MessageField::none(),
            material: ::protobuf::MessageField::none(),
            visible: false,
            delete_me: false,
            is_static: false,
            plugin: ::std::vec::Vec::new(),
            scale: ::protobuf::MessageField::none(),
            meta: ::protobuf::MessageField::none(),
            type_: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Visual {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Visual").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Visual {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Visual {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Visual`
pub mod visual {
    #[derive(::gz_msgs_common::GzMessage)]
    // @@protoc_insertion_point(message:gz.msgs.Visual.Meta)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct Meta {
        // message fields
        // @@protoc_insertion_point(field:gz.msgs.Visual.Meta.layer)
        pub layer: i32,
        // special fields
        // @@protoc_insertion_point(special_field:gz.msgs.Visual.Meta.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Meta {
        fn default() -> &'a Meta {
            <Meta as ::protobuf::Message>::default_instance()
        }
    }

    impl Meta {
        pub fn new() -> Meta {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(1);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "layer",
                |m: &Meta| { &m.layer },
                |m: &mut Meta| { &mut m.layer },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Meta>(
                "Visual.Meta",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Meta {
        const NAME: &'static str = "Meta";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    8 => {
                        self.layer = is.read_int32()?;
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
            if self.layer != 0 {
                my_size += ::protobuf::rt::int32_size(1, self.layer);
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if self.layer != 0 {
                os.write_int32(1, self.layer)?;
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

        fn new() -> Meta {
            Meta::new()
        }

        fn clear(&mut self) {
            self.layer = 0;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Meta {
            static instance: Meta = Meta {
                layer: 0,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Meta {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("Visual.Meta").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Meta {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Meta {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.Visual.Type)
    pub enum Type {
        // @@protoc_insertion_point(enum_value:gz.msgs.Visual.Type.ENTITY)
        ENTITY = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.Visual.Type.MODEL)
        MODEL = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.Visual.Type.LINK)
        LINK = 2,
        // @@protoc_insertion_point(enum_value:gz.msgs.Visual.Type.VISUAL)
        VISUAL = 3,
        // @@protoc_insertion_point(enum_value:gz.msgs.Visual.Type.COLLISION)
        COLLISION = 4,
        // @@protoc_insertion_point(enum_value:gz.msgs.Visual.Type.SENSOR)
        SENSOR = 5,
        // @@protoc_insertion_point(enum_value:gz.msgs.Visual.Type.GUI)
        GUI = 6,
        // @@protoc_insertion_point(enum_value:gz.msgs.Visual.Type.PHYSICS)
        PHYSICS = 7,
    }

    impl ::protobuf::Enum for Type {
        const NAME: &'static str = "Type";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Type> {
            match value {
                0 => ::std::option::Option::Some(Type::ENTITY),
                1 => ::std::option::Option::Some(Type::MODEL),
                2 => ::std::option::Option::Some(Type::LINK),
                3 => ::std::option::Option::Some(Type::VISUAL),
                4 => ::std::option::Option::Some(Type::COLLISION),
                5 => ::std::option::Option::Some(Type::SENSOR),
                6 => ::std::option::Option::Some(Type::GUI),
                7 => ::std::option::Option::Some(Type::PHYSICS),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<Type> {
            match str {
                "ENTITY" => ::std::option::Option::Some(Type::ENTITY),
                "MODEL" => ::std::option::Option::Some(Type::MODEL),
                "LINK" => ::std::option::Option::Some(Type::LINK),
                "VISUAL" => ::std::option::Option::Some(Type::VISUAL),
                "COLLISION" => ::std::option::Option::Some(Type::COLLISION),
                "SENSOR" => ::std::option::Option::Some(Type::SENSOR),
                "GUI" => ::std::option::Option::Some(Type::GUI),
                "PHYSICS" => ::std::option::Option::Some(Type::PHYSICS),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Type] = &[
            Type::ENTITY,
            Type::MODEL,
            Type::LINK,
            Type::VISUAL,
            Type::COLLISION,
            Type::SENSOR,
            Type::GUI,
            Type::PHYSICS,
        ];
    }

    impl ::protobuf::EnumFull for Type {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("Visual.Type").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Type {
        fn default() -> Self {
            Type::ENTITY
        }
    }

    impl Type {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Type>("Visual.Type")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14gz/msgs/visual.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.proto\
    \x1a\x12gz/msgs/pose.proto\x1a\x16gz/msgs/geometry.proto\x1a\x16gz/msgs/\
    material.proto\x1a\x14gz/msgs/plugin.proto\x1a\x16gz/msgs/vector3d.proto\
    \"\xfa\x05\n\x06Visual\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msg\
    s.HeaderR\x06header\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\
    \x0e\n\x02id\x18\x03\x20\x01(\rR\x02id\x12\x1f\n\x0bparent_name\x18\x04\
    \x20\x01(\tR\nparentName\x12\x1b\n\tparent_id\x18\x05\x20\x01(\rR\x08par\
    entId\x12!\n\x0ccast_shadows\x18\x06\x20\x01(\x08R\x0bcastShadows\x12\"\
    \n\x0ctransparency\x18\x07\x20\x01(\x01R\x0ctransparency\x12\x1f\n\x0bla\
    ser_retro\x18\x08\x20\x01(\x01R\nlaserRetro\x12!\n\x04pose\x18\t\x20\x01\
    (\x0b2\r.gz.msgs.PoseR\x04pose\x12-\n\x08geometry\x18\n\x20\x01(\x0b2\
    \x11.gz.msgs.GeometryR\x08geometry\x12-\n\x08material\x18\x0b\x20\x01(\
    \x0b2\x11.gz.msgs.MaterialR\x08material\x12\x18\n\x07visible\x18\x0c\x20\
    \x01(\x08R\x07visible\x12\x1b\n\tdelete_me\x18\r\x20\x01(\x08R\x08delete\
    Me\x12\x1b\n\tis_static\x18\x0e\x20\x01(\x08R\x08isStatic\x12'\n\x06plug\
    in\x18\x0f\x20\x03(\x0b2\x0f.gz.msgs.PluginR\x06plugin\x12'\n\x05scale\
    \x18\x10\x20\x01(\x0b2\x11.gz.msgs.Vector3dR\x05scale\x12(\n\x04meta\x18\
    \x11\x20\x01(\x0b2\x14.gz.msgs.Visual.MetaR\x04meta\x12(\n\x04type\x18\
    \x12\x20\x01(\x0e2\x14.gz.msgs.Visual.TypeR\x04type\x1a\x1c\n\x04Meta\
    \x12\x14\n\x05layer\x18\x01\x20\x01(\x05R\x05layer\"d\n\x04Type\x12\n\n\
    \x06ENTITY\x10\0\x12\t\n\x05MODEL\x10\x01\x12\x08\n\x04LINK\x10\x02\x12\
    \n\n\x06VISUAL\x10\x03\x12\r\n\tCOLLISION\x10\x04\x12\n\n\x06SENSOR\x10\
    \x05\x12\x07\n\x03GUI\x10\x06\x12\x0b\n\x07PHYSICS\x10\x07B\x1b\n\x0bcom\
    .gz.msgsB\x0cVisualProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(6);
            deps.push(super::header::file_descriptor().clone());
            deps.push(super::pose::file_descriptor().clone());
            deps.push(super::geometry::file_descriptor().clone());
            deps.push(super::material::file_descriptor().clone());
            deps.push(super::plugin::file_descriptor().clone());
            deps.push(super::vector3d::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Visual::generated_message_descriptor_data());
            messages.push(visual::Meta::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(visual::Type::generated_enum_descriptor_data());
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
