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

//! Generated file from `gz/msgs/image.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.Image)
pub struct Image {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Image.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Image.width)
    pub width: u32,
    // @@protoc_insertion_point(field:gz.msgs.Image.height)
    pub height: u32,
    // @@protoc_insertion_point(field:gz.msgs.Image.step)
    pub step: u32,
    // @@protoc_insertion_point(field:gz.msgs.Image.data)
    pub data: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:gz.msgs.Image.pixel_format_type)
    pub pixel_format_type: ::protobuf::EnumOrUnknown<PixelFormatType>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Image.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Image {
    fn default() -> &'a Image {
        <Image as ::protobuf::Message>::default_instance()
    }
}

impl Image {
    pub fn new() -> Image {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Image| { &m.header },
            |m: &mut Image| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "width",
            |m: &Image| { &m.width },
            |m: &mut Image| { &mut m.width },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "height",
            |m: &Image| { &m.height },
            |m: &mut Image| { &mut m.height },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "step",
            |m: &Image| { &m.step },
            |m: &mut Image| { &mut m.step },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "data",
            |m: &Image| { &m.data },
            |m: &mut Image| { &mut m.data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "pixel_format_type",
            |m: &Image| { &m.pixel_format_type },
            |m: &mut Image| { &mut m.pixel_format_type },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Image>(
            "Image",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Image {
    const NAME: &'static str = "Image";

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
                    self.width = is.read_uint32()?;
                },
                24 => {
                    self.height = is.read_uint32()?;
                },
                32 => {
                    self.step = is.read_uint32()?;
                },
                42 => {
                    self.data = is.read_bytes()?;
                },
                48 => {
                    self.pixel_format_type = is.read_enum_or_unknown()?;
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
        if self.width != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.width);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.height);
        }
        if self.step != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.step);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(5, &self.data);
        }
        if self.pixel_format_type != ::protobuf::EnumOrUnknown::new(PixelFormatType::UNKNOWN_PIXEL_FORMAT) {
            my_size += ::protobuf::rt::int32_size(6, self.pixel_format_type.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.width != 0 {
            os.write_uint32(2, self.width)?;
        }
        if self.height != 0 {
            os.write_uint32(3, self.height)?;
        }
        if self.step != 0 {
            os.write_uint32(4, self.step)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(5, &self.data)?;
        }
        if self.pixel_format_type != ::protobuf::EnumOrUnknown::new(PixelFormatType::UNKNOWN_PIXEL_FORMAT) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.pixel_format_type))?;
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

    fn new() -> Image {
        Image::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.width = 0;
        self.height = 0;
        self.step = 0;
        self.data.clear();
        self.pixel_format_type = ::protobuf::EnumOrUnknown::new(PixelFormatType::UNKNOWN_PIXEL_FORMAT);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Image {
        static instance: Image = Image {
            header: ::protobuf::MessageField::none(),
            width: 0,
            height: 0,
            step: 0,
            data: ::std::vec::Vec::new(),
            pixel_format_type: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Image {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Image").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Image {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Image {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:gz.msgs.PixelFormatType)
pub enum PixelFormatType {
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.UNKNOWN_PIXEL_FORMAT)
    UNKNOWN_PIXEL_FORMAT = 0,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.L_INT8)
    L_INT8 = 1,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.L_INT16)
    L_INT16 = 2,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.RGB_INT8)
    RGB_INT8 = 3,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.RGBA_INT8)
    RGBA_INT8 = 4,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.BGRA_INT8)
    BGRA_INT8 = 5,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.RGB_INT16)
    RGB_INT16 = 6,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.RGB_INT32)
    RGB_INT32 = 7,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.BGR_INT8)
    BGR_INT8 = 8,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.BGR_INT16)
    BGR_INT16 = 9,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.BGR_INT32)
    BGR_INT32 = 10,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.R_FLOAT16)
    R_FLOAT16 = 11,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.RGB_FLOAT16)
    RGB_FLOAT16 = 12,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.R_FLOAT32)
    R_FLOAT32 = 13,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.RGB_FLOAT32)
    RGB_FLOAT32 = 14,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.BAYER_RGGB8)
    BAYER_RGGB8 = 15,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.BAYER_BGGR8)
    BAYER_BGGR8 = 16,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.BAYER_GBRG8)
    BAYER_GBRG8 = 17,
    // @@protoc_insertion_point(enum_value:gz.msgs.PixelFormatType.BAYER_GRBG8)
    BAYER_GRBG8 = 18,
}

impl ::protobuf::Enum for PixelFormatType {
    const NAME: &'static str = "PixelFormatType";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PixelFormatType> {
        match value {
            0 => ::std::option::Option::Some(PixelFormatType::UNKNOWN_PIXEL_FORMAT),
            1 => ::std::option::Option::Some(PixelFormatType::L_INT8),
            2 => ::std::option::Option::Some(PixelFormatType::L_INT16),
            3 => ::std::option::Option::Some(PixelFormatType::RGB_INT8),
            4 => ::std::option::Option::Some(PixelFormatType::RGBA_INT8),
            5 => ::std::option::Option::Some(PixelFormatType::BGRA_INT8),
            6 => ::std::option::Option::Some(PixelFormatType::RGB_INT16),
            7 => ::std::option::Option::Some(PixelFormatType::RGB_INT32),
            8 => ::std::option::Option::Some(PixelFormatType::BGR_INT8),
            9 => ::std::option::Option::Some(PixelFormatType::BGR_INT16),
            10 => ::std::option::Option::Some(PixelFormatType::BGR_INT32),
            11 => ::std::option::Option::Some(PixelFormatType::R_FLOAT16),
            12 => ::std::option::Option::Some(PixelFormatType::RGB_FLOAT16),
            13 => ::std::option::Option::Some(PixelFormatType::R_FLOAT32),
            14 => ::std::option::Option::Some(PixelFormatType::RGB_FLOAT32),
            15 => ::std::option::Option::Some(PixelFormatType::BAYER_RGGB8),
            16 => ::std::option::Option::Some(PixelFormatType::BAYER_BGGR8),
            17 => ::std::option::Option::Some(PixelFormatType::BAYER_GBRG8),
            18 => ::std::option::Option::Some(PixelFormatType::BAYER_GRBG8),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [PixelFormatType] = &[
        PixelFormatType::UNKNOWN_PIXEL_FORMAT,
        PixelFormatType::L_INT8,
        PixelFormatType::L_INT16,
        PixelFormatType::RGB_INT8,
        PixelFormatType::RGBA_INT8,
        PixelFormatType::BGRA_INT8,
        PixelFormatType::RGB_INT16,
        PixelFormatType::RGB_INT32,
        PixelFormatType::BGR_INT8,
        PixelFormatType::BGR_INT16,
        PixelFormatType::BGR_INT32,
        PixelFormatType::R_FLOAT16,
        PixelFormatType::RGB_FLOAT16,
        PixelFormatType::R_FLOAT32,
        PixelFormatType::RGB_FLOAT32,
        PixelFormatType::BAYER_RGGB8,
        PixelFormatType::BAYER_BGGR8,
        PixelFormatType::BAYER_GBRG8,
        PixelFormatType::BAYER_GRBG8,
    ];
}

impl ::protobuf::EnumFull for PixelFormatType {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("PixelFormatType").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for PixelFormatType {
    fn default() -> Self {
        PixelFormatType::UNKNOWN_PIXEL_FORMAT
    }
}

impl PixelFormatType {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PixelFormatType>("PixelFormatType")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13gz/msgs/image.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.proto\"\
    \xcc\x01\n\x05Image\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.H\
    eaderR\x06header\x12\x14\n\x05width\x18\x02\x20\x01(\rR\x05width\x12\x16\
    \n\x06height\x18\x03\x20\x01(\rR\x06height\x12\x12\n\x04step\x18\x04\x20\
    \x01(\rR\x04step\x12\x12\n\x04data\x18\x05\x20\x01(\x0cR\x04data\x12D\n\
    \x11pixel_format_type\x18\x06\x20\x01(\x0e2\x18.gz.msgs.PixelFormatTypeR\
    \x0fpixelFormatType*\xbe\x02\n\x0fPixelFormatType\x12\x18\n\x14UNKNOWN_P\
    IXEL_FORMAT\x10\0\x12\n\n\x06L_INT8\x10\x01\x12\x0b\n\x07L_INT16\x10\x02\
    \x12\x0c\n\x08RGB_INT8\x10\x03\x12\r\n\tRGBA_INT8\x10\x04\x12\r\n\tBGRA_\
    INT8\x10\x05\x12\r\n\tRGB_INT16\x10\x06\x12\r\n\tRGB_INT32\x10\x07\x12\
    \x0c\n\x08BGR_INT8\x10\x08\x12\r\n\tBGR_INT16\x10\t\x12\r\n\tBGR_INT32\
    \x10\n\x12\r\n\tR_FLOAT16\x10\x0b\x12\x0f\n\x0bRGB_FLOAT16\x10\x0c\x12\r\
    \n\tR_FLOAT32\x10\r\x12\x0f\n\x0bRGB_FLOAT32\x10\x0e\x12\x0f\n\x0bBAYER_\
    RGGB8\x10\x0f\x12\x0f\n\x0bBAYER_BGGR8\x10\x10\x12\x0f\n\x0bBAYER_GBRG8\
    \x10\x11\x12\x0f\n\x0bBAYER_GRBG8\x10\x12B\x1a\n\x0bcom.gz.msgsB\x0bImag\
    eProtosb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Image::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(PixelFormatType::generated_enum_descriptor_data());
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
