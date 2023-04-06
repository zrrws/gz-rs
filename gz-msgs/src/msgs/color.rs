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

//! Generated file from `gz/msgs/color.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:gz.msgs.Color)
pub struct Color {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Color.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Color.r)
    pub r: f32,
    // @@protoc_insertion_point(field:gz.msgs.Color.g)
    pub g: f32,
    // @@protoc_insertion_point(field:gz.msgs.Color.b)
    pub b: f32,
    // @@protoc_insertion_point(field:gz.msgs.Color.a)
    pub a: f32,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Color.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Color {
    fn default() -> &'a Color {
        <Color as ::protobuf::Message>::default_instance()
    }
}

impl Color {
    pub fn new() -> Color {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Color| { &m.header },
            |m: &mut Color| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "r",
            |m: &Color| { &m.r },
            |m: &mut Color| { &mut m.r },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "g",
            |m: &Color| { &m.g },
            |m: &mut Color| { &mut m.g },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "b",
            |m: &Color| { &m.b },
            |m: &mut Color| { &mut m.b },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "a",
            |m: &Color| { &m.a },
            |m: &mut Color| { &mut m.a },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Color>(
            "Color",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Color {
    const NAME: &'static str = "Color";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                21 => {
                    self.r = is.read_float()?;
                },
                29 => {
                    self.g = is.read_float()?;
                },
                37 => {
                    self.b = is.read_float()?;
                },
                45 => {
                    self.a = is.read_float()?;
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
        if self.r != 0. {
            my_size += 1 + 4;
        }
        if self.g != 0. {
            my_size += 1 + 4;
        }
        if self.b != 0. {
            my_size += 1 + 4;
        }
        if self.a != 0. {
            my_size += 1 + 4;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.r != 0. {
            os.write_float(2, self.r)?;
        }
        if self.g != 0. {
            os.write_float(3, self.g)?;
        }
        if self.b != 0. {
            os.write_float(4, self.b)?;
        }
        if self.a != 0. {
            os.write_float(5, self.a)?;
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

    fn new() -> Color {
        Color::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.r = 0.;
        self.g = 0.;
        self.b = 0.;
        self.a = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Color {
        static instance: Color = Color {
            header: ::protobuf::MessageField::none(),
            r: 0.,
            g: 0.,
            b: 0.,
            a: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Color {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Color").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Color {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Color {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13gz/msgs/color.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.proto\"h\
    \n\x05Color\x12'\n\x06header\x18\x01\x20\x01(\x0b2\x0f.gz.msgs.HeaderR\
    \x06header\x12\x0c\n\x01r\x18\x02\x20\x01(\x02R\x01r\x12\x0c\n\x01g\x18\
    \x03\x20\x01(\x02R\x01g\x12\x0c\n\x01b\x18\x04\x20\x01(\x02R\x01b\x12\
    \x0c\n\x01a\x18\x05\x20\x01(\x02R\x01aB\x1a\n\x0bcom.gz.msgsB\x0bColorPr\
    otosb\x06proto3\
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
            messages.push(Color::generated_message_descriptor_data());
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
