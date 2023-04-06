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

//! Generated file from `gz/msgs/contact.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:gz.msgs.Contact)
pub struct Contact {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.Contact.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.Contact.collision1)
    pub collision1: ::protobuf::MessageField<super::entity::Entity>,
    // @@protoc_insertion_point(field:gz.msgs.Contact.collision2)
    pub collision2: ::protobuf::MessageField<super::entity::Entity>,
    // @@protoc_insertion_point(field:gz.msgs.Contact.position)
    pub position: ::std::vec::Vec<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:gz.msgs.Contact.normal)
    pub normal: ::std::vec::Vec<super::vector3d::Vector3d>,
    // @@protoc_insertion_point(field:gz.msgs.Contact.depth)
    pub depth: ::std::vec::Vec<f64>,
    // @@protoc_insertion_point(field:gz.msgs.Contact.wrench)
    pub wrench: ::std::vec::Vec<super::joint_wrench::JointWrench>,
    // @@protoc_insertion_point(field:gz.msgs.Contact.world)
    pub world: ::protobuf::MessageField<super::entity::Entity>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.Contact.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Contact {
    fn default() -> &'a Contact {
        <Contact as ::protobuf::Message>::default_instance()
    }
}

impl Contact {
    pub fn new() -> Contact {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Contact| { &m.header },
            |m: &mut Contact| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::entity::Entity>(
            "collision1",
            |m: &Contact| { &m.collision1 },
            |m: &mut Contact| { &mut m.collision1 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::entity::Entity>(
            "collision2",
            |m: &Contact| { &m.collision2 },
            |m: &mut Contact| { &mut m.collision2 },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "position",
            |m: &Contact| { &m.position },
            |m: &mut Contact| { &mut m.position },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "normal",
            |m: &Contact| { &m.normal },
            |m: &mut Contact| { &mut m.normal },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "depth",
            |m: &Contact| { &m.depth },
            |m: &mut Contact| { &mut m.depth },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "wrench",
            |m: &Contact| { &m.wrench },
            |m: &mut Contact| { &mut m.wrench },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::entity::Entity>(
            "world",
            |m: &Contact| { &m.world },
            |m: &mut Contact| { &mut m.world },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Contact>(
            "Contact",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Contact {
    const NAME: &'static str = "Contact";

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
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.collision1)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.collision2)?;
                },
                34 => {
                    self.position.push(is.read_message()?);
                },
                42 => {
                    self.normal.push(is.read_message()?);
                },
                50 => {
                    is.read_repeated_packed_double_into(&mut self.depth)?;
                },
                49 => {
                    self.depth.push(is.read_double()?);
                },
                58 => {
                    self.wrench.push(is.read_message()?);
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.world)?;
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
        if let Some(v) = self.collision1.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.collision2.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.position {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.normal {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += 9 * self.depth.len() as u64;
        for value in &self.wrench {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.world.as_ref() {
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
        if let Some(v) = self.collision1.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let Some(v) = self.collision2.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        for v in &self.position {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        for v in &self.normal {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
        };
        for v in &self.depth {
            os.write_double(6, *v)?;
        };
        for v in &self.wrench {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        if let Some(v) = self.world.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
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

    fn new() -> Contact {
        Contact::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.collision1.clear();
        self.collision2.clear();
        self.position.clear();
        self.normal.clear();
        self.depth.clear();
        self.wrench.clear();
        self.world.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Contact {
        static instance: Contact = Contact {
            header: ::protobuf::MessageField::none(),
            collision1: ::protobuf::MessageField::none(),
            collision2: ::protobuf::MessageField::none(),
            position: ::std::vec::Vec::new(),
            normal: ::std::vec::Vec::new(),
            depth: ::std::vec::Vec::new(),
            wrench: ::std::vec::Vec::new(),
            world: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Contact {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Contact").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Contact {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Contact {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15gz/msgs/contact.proto\x12\x07gz.msgs\x1a\x14gz/msgs/entity.proto\
    \x1a\x16gz/msgs/vector3d.proto\x1a\x1agz/msgs/joint_wrench.proto\x1a\x14\
    gz/msgs/header.proto\"\xd9\x02\n\x07Contact\x12'\n\x06header\x18\x01\x20\
    \x01(\x0b2\x0f.gz.msgs.HeaderR\x06header\x12/\n\ncollision1\x18\x02\x20\
    \x01(\x0b2\x0f.gz.msgs.EntityR\ncollision1\x12/\n\ncollision2\x18\x03\
    \x20\x01(\x0b2\x0f.gz.msgs.EntityR\ncollision2\x12-\n\x08position\x18\
    \x04\x20\x03(\x0b2\x11.gz.msgs.Vector3dR\x08position\x12)\n\x06normal\
    \x18\x05\x20\x03(\x0b2\x11.gz.msgs.Vector3dR\x06normal\x12\x14\n\x05dept\
    h\x18\x06\x20\x03(\x01R\x05depth\x12,\n\x06wrench\x18\x07\x20\x03(\x0b2\
    \x14.gz.msgs.JointWrenchR\x06wrench\x12%\n\x05world\x18\x08\x20\x01(\x0b\
    2\x0f.gz.msgs.EntityR\x05worldB\x1c\n\x0bcom.gz.msgsB\rContactProtosb\
    \x06proto3\
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
            deps.push(super::entity::file_descriptor().clone());
            deps.push(super::vector3d::file_descriptor().clone());
            deps.push(super::joint_wrench::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(Contact::generated_message_descriptor_data());
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
