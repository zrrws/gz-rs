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

//! Generated file from `ignition/msgs/gui.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.GUI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GUI {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.GUI.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.GUI.fullscreen)
    pub fullscreen: bool,
    // @@protoc_insertion_point(field:ignition.msgs.GUI.camera)
    pub camera: ::protobuf::MessageField<super::gui_camera::GUICamera>,
    // @@protoc_insertion_point(field:ignition.msgs.GUI.plugin)
    pub plugin: ::std::vec::Vec<super::plugin::Plugin>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.GUI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GUI {
    fn default() -> &'a GUI {
        <GUI as ::protobuf::Message>::default_instance()
    }
}

impl GUI {
    pub fn new() -> GUI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &GUI| { &m.header },
            |m: &mut GUI| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fullscreen",
            |m: &GUI| { &m.fullscreen },
            |m: &mut GUI| { &mut m.fullscreen },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::gui_camera::GUICamera>(
            "camera",
            |m: &GUI| { &m.camera },
            |m: &mut GUI| { &mut m.camera },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "plugin",
            |m: &GUI| { &m.plugin },
            |m: &mut GUI| { &mut m.plugin },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GUI>(
            "GUI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GUI {
    const NAME: &'static str = "GUI";

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
                    self.fullscreen = is.read_bool()?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.camera)?;
                },
                34 => {
                    self.plugin.push(is.read_message()?);
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
        if self.fullscreen != false {
            my_size += 1 + 1;
        }
        if let Some(v) = self.camera.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.plugin {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.fullscreen != false {
            os.write_bool(2, self.fullscreen)?;
        }
        if let Some(v) = self.camera.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        for v in &self.plugin {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> GUI {
        GUI::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.fullscreen = false;
        self.camera.clear();
        self.plugin.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GUI {
        static instance: GUI = GUI {
            header: ::protobuf::MessageField::none(),
            fullscreen: false,
            camera: ::protobuf::MessageField::none(),
            plugin: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GUI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GUI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GUI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GUI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ignition/msgs/gui.proto\x12\rignition.msgs\x1a\x1eignition/msgs/gu\
    i_camera.proto\x1a\x1aignition/msgs/plugin.proto\x1a\x1aignition/msgs/he\
    ader.proto\"\xb5\x01\n\x03GUI\x12-\n\x06header\x18\x01\x20\x01(\x0b2\x15\
    .ignition.msgs.HeaderR\x06header\x12\x1e\n\nfullscreen\x18\x02\x20\x01(\
    \x08R\nfullscreen\x120\n\x06camera\x18\x03\x20\x01(\x0b2\x18.ignition.ms\
    gs.GUICameraR\x06camera\x12-\n\x06plugin\x18\x04\x20\x03(\x0b2\x15.ignit\
    ion.msgs.PluginR\x06pluginB\x1e\n\x11com.ignition.msgsB\tGUIProtosb\x06p\
    roto3\
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
            deps.push(super::gui_camera::file_descriptor().clone());
            deps.push(super::plugin::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GUI::generated_message_descriptor_data());
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
