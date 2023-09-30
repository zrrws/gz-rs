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

//! Generated file from `ignition/msgs/annotated_axis_aligned_2d_box.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.AnnotatedAxisAligned2DBox)
pub struct AnnotatedAxisAligned2DBox {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.AnnotatedAxisAligned2DBox.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.AnnotatedAxisAligned2DBox.box)
    pub box_: ::protobuf::MessageField<super::axis_aligned_2d_box::AxisAligned2DBox>,
    // @@protoc_insertion_point(field:ignition.msgs.AnnotatedAxisAligned2DBox.label)
    pub label: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.AnnotatedAxisAligned2DBox.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AnnotatedAxisAligned2DBox {
    fn default() -> &'a AnnotatedAxisAligned2DBox {
        <AnnotatedAxisAligned2DBox as ::protobuf::Message>::default_instance()
    }
}

impl AnnotatedAxisAligned2DBox {
    pub fn new() -> AnnotatedAxisAligned2DBox {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &AnnotatedAxisAligned2DBox| { &m.header },
            |m: &mut AnnotatedAxisAligned2DBox| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::axis_aligned_2d_box::AxisAligned2DBox>(
            "box",
            |m: &AnnotatedAxisAligned2DBox| { &m.box_ },
            |m: &mut AnnotatedAxisAligned2DBox| { &mut m.box_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "label",
            |m: &AnnotatedAxisAligned2DBox| { &m.label },
            |m: &mut AnnotatedAxisAligned2DBox| { &mut m.label },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AnnotatedAxisAligned2DBox>(
            "AnnotatedAxisAligned2DBox",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AnnotatedAxisAligned2DBox {
    const NAME: &'static str = "AnnotatedAxisAligned2DBox";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.box_)?;
                },
                24 => {
                    self.label = is.read_uint32()?;
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
        if let Some(v) = self.box_.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.label != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.label);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.box_.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if self.label != 0 {
            os.write_uint32(3, self.label)?;
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

    fn new() -> AnnotatedAxisAligned2DBox {
        AnnotatedAxisAligned2DBox::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.box_.clear();
        self.label = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AnnotatedAxisAligned2DBox {
        static instance: AnnotatedAxisAligned2DBox = AnnotatedAxisAligned2DBox {
            header: ::protobuf::MessageField::none(),
            box_: ::protobuf::MessageField::none(),
            label: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AnnotatedAxisAligned2DBox {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AnnotatedAxisAligned2DBox").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AnnotatedAxisAligned2DBox {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AnnotatedAxisAligned2DBox {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n1ignition/msgs/annotated_axis_aligned_2d_box.proto\x12\rignition.msgs\
    \x1a\x1aignition/msgs/header.proto\x1a'ignition/msgs/axis_aligned_2d_box\
    .proto\"\x93\x01\n\x19AnnotatedAxisAligned2DBox\x12-\n\x06header\x18\x01\
    \x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\x121\n\x03box\x18\x02\
    \x20\x01(\x0b2\x1f.ignition.msgs.AxisAligned2DBoxR\x03box\x12\x14\n\x05l\
    abel\x18\x03\x20\x01(\rR\x05labelB4\n\x11com.ignition.msgsB\x1fAnnotated\
    AxisAligned2DBoxProtosb\x06proto3\
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
            deps.push(super::axis_aligned_2d_box::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AnnotatedAxisAligned2DBox::generated_message_descriptor_data());
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
