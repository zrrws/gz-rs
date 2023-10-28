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

//! Generated file from `ignition/msgs/heightmapgeom.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.HeightmapGeom)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct HeightmapGeom {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.image)
    pub image: ::protobuf::MessageField<super::image::Image>,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.size)
    pub size: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.origin)
    pub origin: ::protobuf::MessageField<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.heights)
    pub heights: ::std::vec::Vec<f32>,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.width)
    pub width: i32,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.height)
    pub height: i32,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.texture)
    pub texture: ::std::vec::Vec<heightmap_geom::Texture>,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.blend)
    pub blend: ::std::vec::Vec<heightmap_geom::Blend>,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.use_terrain_paging)
    pub use_terrain_paging: bool,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.filename)
    pub filename: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.sampling)
    pub sampling: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.HeightmapGeom.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a HeightmapGeom {
    fn default() -> &'a HeightmapGeom {
        <HeightmapGeom as ::protobuf::Message>::default_instance()
    }
}

impl HeightmapGeom {
    pub fn new() -> HeightmapGeom {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(12);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &HeightmapGeom| { &m.header },
            |m: &mut HeightmapGeom| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::image::Image>(
            "image",
            |m: &HeightmapGeom| { &m.image },
            |m: &mut HeightmapGeom| { &mut m.image },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "size",
            |m: &HeightmapGeom| { &m.size },
            |m: &mut HeightmapGeom| { &mut m.size },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::vector3d::Vector3d>(
            "origin",
            |m: &HeightmapGeom| { &m.origin },
            |m: &mut HeightmapGeom| { &mut m.origin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "heights",
            |m: &HeightmapGeom| { &m.heights },
            |m: &mut HeightmapGeom| { &mut m.heights },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "width",
            |m: &HeightmapGeom| { &m.width },
            |m: &mut HeightmapGeom| { &mut m.width },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "height",
            |m: &HeightmapGeom| { &m.height },
            |m: &mut HeightmapGeom| { &mut m.height },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "texture",
            |m: &HeightmapGeom| { &m.texture },
            |m: &mut HeightmapGeom| { &mut m.texture },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "blend",
            |m: &HeightmapGeom| { &m.blend },
            |m: &mut HeightmapGeom| { &mut m.blend },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "use_terrain_paging",
            |m: &HeightmapGeom| { &m.use_terrain_paging },
            |m: &mut HeightmapGeom| { &mut m.use_terrain_paging },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "filename",
            |m: &HeightmapGeom| { &m.filename },
            |m: &mut HeightmapGeom| { &mut m.filename },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sampling",
            |m: &HeightmapGeom| { &m.sampling },
            |m: &mut HeightmapGeom| { &mut m.sampling },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<HeightmapGeom>(
            "HeightmapGeom",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for HeightmapGeom {
    const NAME: &'static str = "HeightmapGeom";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.image)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.size)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.origin)?;
                },
                42 => {
                    is.read_repeated_packed_float_into(&mut self.heights)?;
                },
                45 => {
                    self.heights.push(is.read_float()?);
                },
                48 => {
                    self.width = is.read_int32()?;
                },
                56 => {
                    self.height = is.read_int32()?;
                },
                66 => {
                    self.texture.push(is.read_message()?);
                },
                74 => {
                    self.blend.push(is.read_message()?);
                },
                80 => {
                    self.use_terrain_paging = is.read_bool()?;
                },
                90 => {
                    self.filename = is.read_string()?;
                },
                96 => {
                    self.sampling = is.read_uint32()?;
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
        if let Some(v) = self.image.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.origin.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += 5 * self.heights.len() as u64;
        if self.width != 0 {
            my_size += ::protobuf::rt::int32_size(6, self.width);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::int32_size(7, self.height);
        }
        for value in &self.texture {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.blend {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.use_terrain_paging != false {
            my_size += 1 + 1;
        }
        if !self.filename.is_empty() {
            my_size += ::protobuf::rt::string_size(11, &self.filename);
        }
        if self.sampling != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.sampling);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.image.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.size.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.origin.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        for v in &self.heights {
            os.write_float(5, *v)?;
        };
        if self.width != 0 {
            os.write_int32(6, self.width)?;
        }
        if self.height != 0 {
            os.write_int32(7, self.height)?;
        }
        for v in &self.texture {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for v in &self.blend {
            ::protobuf::rt::write_message_field_with_cached_size(9, v, os)?;
        };
        if self.use_terrain_paging != false {
            os.write_bool(10, self.use_terrain_paging)?;
        }
        if !self.filename.is_empty() {
            os.write_string(11, &self.filename)?;
        }
        if self.sampling != 0 {
            os.write_uint32(12, self.sampling)?;
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

    fn new() -> HeightmapGeom {
        HeightmapGeom::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.image.clear();
        self.size.clear();
        self.origin.clear();
        self.heights.clear();
        self.width = 0;
        self.height = 0;
        self.texture.clear();
        self.blend.clear();
        self.use_terrain_paging = false;
        self.filename.clear();
        self.sampling = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static HeightmapGeom {
        static instance: HeightmapGeom = HeightmapGeom {
            header: ::protobuf::MessageField::none(),
            image: ::protobuf::MessageField::none(),
            size: ::protobuf::MessageField::none(),
            origin: ::protobuf::MessageField::none(),
            heights: ::std::vec::Vec::new(),
            width: 0,
            height: 0,
            texture: ::std::vec::Vec::new(),
            blend: ::std::vec::Vec::new(),
            use_terrain_paging: false,
            filename: ::std::string::String::new(),
            sampling: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for HeightmapGeom {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("HeightmapGeom").unwrap()).clone()
    }
}

impl ::std::fmt::Display for HeightmapGeom {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeightmapGeom {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `HeightmapGeom`
pub mod heightmap_geom {
    #[derive(::gz_msgs_common::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.HeightmapGeom.Texture)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct Texture {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.Texture.diffuse)
        pub diffuse: ::std::string::String,
        // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.Texture.normal)
        pub normal: ::std::string::String,
        // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.Texture.size)
        pub size: f64,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.HeightmapGeom.Texture.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Texture {
        fn default() -> &'a Texture {
            <Texture as ::protobuf::Message>::default_instance()
        }
    }

    impl Texture {
        pub fn new() -> Texture {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(3);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "diffuse",
                |m: &Texture| { &m.diffuse },
                |m: &mut Texture| { &mut m.diffuse },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "normal",
                |m: &Texture| { &m.normal },
                |m: &mut Texture| { &mut m.normal },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "size",
                |m: &Texture| { &m.size },
                |m: &mut Texture| { &mut m.size },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Texture>(
                "HeightmapGeom.Texture",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Texture {
        const NAME: &'static str = "Texture";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.diffuse = is.read_string()?;
                    },
                    18 => {
                        self.normal = is.read_string()?;
                    },
                    25 => {
                        self.size = is.read_double()?;
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
            if !self.diffuse.is_empty() {
                my_size += ::protobuf::rt::string_size(1, &self.diffuse);
            }
            if !self.normal.is_empty() {
                my_size += ::protobuf::rt::string_size(2, &self.normal);
            }
            if self.size != 0. {
                my_size += 1 + 8;
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if !self.diffuse.is_empty() {
                os.write_string(1, &self.diffuse)?;
            }
            if !self.normal.is_empty() {
                os.write_string(2, &self.normal)?;
            }
            if self.size != 0. {
                os.write_double(3, self.size)?;
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

        fn new() -> Texture {
            Texture::new()
        }

        fn clear(&mut self) {
            self.diffuse.clear();
            self.normal.clear();
            self.size = 0.;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Texture {
            static instance: Texture = Texture {
                diffuse: ::std::string::String::new(),
                normal: ::std::string::String::new(),
                size: 0.,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Texture {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("HeightmapGeom.Texture").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Texture {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Texture {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }

    #[derive(::gz_msgs_common::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.HeightmapGeom.Blend)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct Blend {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.Blend.min_height)
        pub min_height: f64,
        // @@protoc_insertion_point(field:ignition.msgs.HeightmapGeom.Blend.fade_dist)
        pub fade_dist: f64,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.HeightmapGeom.Blend.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a Blend {
        fn default() -> &'a Blend {
            <Blend as ::protobuf::Message>::default_instance()
        }
    }

    impl Blend {
        pub fn new() -> Blend {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(2);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "min_height",
                |m: &Blend| { &m.min_height },
                |m: &mut Blend| { &mut m.min_height },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "fade_dist",
                |m: &Blend| { &m.fade_dist },
                |m: &mut Blend| { &mut m.fade_dist },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Blend>(
                "HeightmapGeom.Blend",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for Blend {
        const NAME: &'static str = "Blend";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    9 => {
                        self.min_height = is.read_double()?;
                    },
                    17 => {
                        self.fade_dist = is.read_double()?;
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
            if self.min_height != 0. {
                my_size += 1 + 8;
            }
            if self.fade_dist != 0. {
                my_size += 1 + 8;
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if self.min_height != 0. {
                os.write_double(1, self.min_height)?;
            }
            if self.fade_dist != 0. {
                os.write_double(2, self.fade_dist)?;
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

        fn new() -> Blend {
            Blend::new()
        }

        fn clear(&mut self) {
            self.min_height = 0.;
            self.fade_dist = 0.;
            self.special_fields.clear();
        }

        fn default_instance() -> &'static Blend {
            static instance: Blend = Blend {
                min_height: 0.,
                fade_dist: 0.,
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for Blend {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("HeightmapGeom.Blend").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for Blend {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for Blend {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!ignition/msgs/heightmapgeom.proto\x12\rignition.msgs\x1a\x19ignition/\
    msgs/image.proto\x1a\x1cignition/msgs/vector3d.proto\x1a\x1aignition/msg\
    s/header.proto\"\x86\x05\n\rHeightmapGeom\x12-\n\x06header\x18\x01\x20\
    \x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\x12*\n\x05image\x18\x02\
    \x20\x01(\x0b2\x14.ignition.msgs.ImageR\x05image\x12+\n\x04size\x18\x03\
    \x20\x01(\x0b2\x17.ignition.msgs.Vector3dR\x04size\x12/\n\x06origin\x18\
    \x04\x20\x01(\x0b2\x17.ignition.msgs.Vector3dR\x06origin\x12\x18\n\x07he\
    ights\x18\x05\x20\x03(\x02R\x07heights\x12\x14\n\x05width\x18\x06\x20\
    \x01(\x05R\x05width\x12\x16\n\x06height\x18\x07\x20\x01(\x05R\x06height\
    \x12>\n\x07texture\x18\x08\x20\x03(\x0b2$.ignition.msgs.HeightmapGeom.Te\
    xtureR\x07texture\x128\n\x05blend\x18\t\x20\x03(\x0b2\".ignition.msgs.He\
    ightmapGeom.BlendR\x05blend\x12,\n\x12use_terrain_paging\x18\n\x20\x01(\
    \x08R\x10useTerrainPaging\x12\x1a\n\x08filename\x18\x0b\x20\x01(\tR\x08f\
    ilename\x12\x1a\n\x08sampling\x18\x0c\x20\x01(\rR\x08sampling\x1aO\n\x07\
    Texture\x12\x18\n\x07diffuse\x18\x01\x20\x01(\tR\x07diffuse\x12\x16\n\
    \x06normal\x18\x02\x20\x01(\tR\x06normal\x12\x12\n\x04size\x18\x03\x20\
    \x01(\x01R\x04size\x1aC\n\x05Blend\x12\x1d\n\nmin_height\x18\x01\x20\x01\
    (\x01R\tminHeight\x12\x1b\n\tfade_dist\x18\x02\x20\x01(\x01R\x08fadeDist\
    B(\n\x11com.ignition.msgsB\x13HeightmapGeomProtosb\x06proto3\
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
            deps.push(super::image::file_descriptor().clone());
            deps.push(super::vector3d::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(HeightmapGeom::generated_message_descriptor_data());
            messages.push(heightmap_geom::Texture::generated_message_descriptor_data());
            messages.push(heightmap_geom::Blend::generated_message_descriptor_data());
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
