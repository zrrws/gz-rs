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

//! Generated file from `ignition/msgs/diagnostics.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.Diagnostics)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct Diagnostics {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.Diagnostics.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.Diagnostics.time)
    pub time: ::std::vec::Vec<diagnostics::DiagTime>,
    // @@protoc_insertion_point(field:ignition.msgs.Diagnostics.real_time)
    pub real_time: ::protobuf::MessageField<super::time::Time>,
    // @@protoc_insertion_point(field:ignition.msgs.Diagnostics.sim_time)
    pub sim_time: ::protobuf::MessageField<super::time::Time>,
    // @@protoc_insertion_point(field:ignition.msgs.Diagnostics.real_time_factor)
    pub real_time_factor: f64,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.Diagnostics.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Diagnostics {
    fn default() -> &'a Diagnostics {
        <Diagnostics as ::protobuf::Message>::default_instance()
    }
}

impl Diagnostics {
    pub fn new() -> Diagnostics {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &Diagnostics| { &m.header },
            |m: &mut Diagnostics| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "time",
            |m: &Diagnostics| { &m.time },
            |m: &mut Diagnostics| { &mut m.time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::time::Time>(
            "real_time",
            |m: &Diagnostics| { &m.real_time },
            |m: &mut Diagnostics| { &mut m.real_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::time::Time>(
            "sim_time",
            |m: &Diagnostics| { &m.sim_time },
            |m: &mut Diagnostics| { &mut m.sim_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "real_time_factor",
            |m: &Diagnostics| { &m.real_time_factor },
            |m: &mut Diagnostics| { &mut m.real_time_factor },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Diagnostics>(
            "Diagnostics",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Diagnostics {
    const NAME: &'static str = "Diagnostics";

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
                    self.time.push(is.read_message()?);
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.real_time)?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.sim_time)?;
                },
                41 => {
                    self.real_time_factor = is.read_double()?;
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
        for value in &self.time {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.real_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.sim_time.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.real_time_factor != 0. {
            my_size += 1 + 8;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        for v in &self.time {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if let Some(v) = self.real_time.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.sim_time.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        }
        if self.real_time_factor != 0. {
            os.write_double(5, self.real_time_factor)?;
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

    fn new() -> Diagnostics {
        Diagnostics::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.time.clear();
        self.real_time.clear();
        self.sim_time.clear();
        self.real_time_factor = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Diagnostics {
        static instance: Diagnostics = Diagnostics {
            header: ::protobuf::MessageField::none(),
            time: ::std::vec::Vec::new(),
            real_time: ::protobuf::MessageField::none(),
            sim_time: ::protobuf::MessageField::none(),
            real_time_factor: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Diagnostics {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Diagnostics").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Diagnostics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Diagnostics {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `Diagnostics`
pub mod diagnostics {
    #[derive(::gz_msgs_common::IgnMessage)]
    // @@protoc_insertion_point(message:ignition.msgs.Diagnostics.DiagTime)
    #[derive(PartialEq,Clone,Default,Debug)]
    pub struct DiagTime {
        // message fields
        // @@protoc_insertion_point(field:ignition.msgs.Diagnostics.DiagTime.name)
        pub name: ::std::string::String,
        // @@protoc_insertion_point(field:ignition.msgs.Diagnostics.DiagTime.elapsed)
        pub elapsed: ::protobuf::MessageField<super::super::time::Time>,
        // @@protoc_insertion_point(field:ignition.msgs.Diagnostics.DiagTime.wall)
        pub wall: ::protobuf::MessageField<super::super::time::Time>,
        // special fields
        // @@protoc_insertion_point(special_field:ignition.msgs.Diagnostics.DiagTime.special_fields)
        pub special_fields: ::protobuf::SpecialFields,
    }

    impl<'a> ::std::default::Default for &'a DiagTime {
        fn default() -> &'a DiagTime {
            <DiagTime as ::protobuf::Message>::default_instance()
        }
    }

    impl DiagTime {
        pub fn new() -> DiagTime {
            ::std::default::Default::default()
        }

        pub(in super) fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
            let mut fields = ::std::vec::Vec::with_capacity(3);
            let mut oneofs = ::std::vec::Vec::with_capacity(0);
            fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
                "name",
                |m: &DiagTime| { &m.name },
                |m: &mut DiagTime| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::time::Time>(
                "elapsed",
                |m: &DiagTime| { &m.elapsed },
                |m: &mut DiagTime| { &mut m.elapsed },
            ));
            fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::super::time::Time>(
                "wall",
                |m: &DiagTime| { &m.wall },
                |m: &mut DiagTime| { &mut m.wall },
            ));
            ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DiagTime>(
                "Diagnostics.DiagTime",
                fields,
                oneofs,
            )
        }
    }

    impl ::protobuf::Message for DiagTime {
        const NAME: &'static str = "DiagTime";

        fn is_initialized(&self) -> bool {
            true
        }

        fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
            while let Some(tag) = is.read_raw_tag_or_eof()? {
                match tag {
                    10 => {
                        self.name = is.read_string()?;
                    },
                    18 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.elapsed)?;
                    },
                    26 => {
                        ::protobuf::rt::read_singular_message_into_field(is, &mut self.wall)?;
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
            if !self.name.is_empty() {
                my_size += ::protobuf::rt::string_size(1, &self.name);
            }
            if let Some(v) = self.elapsed.as_ref() {
                let len = v.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            }
            if let Some(v) = self.wall.as_ref() {
                let len = v.compute_size();
                my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            }
            my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
            self.special_fields.cached_size().set(my_size as u32);
            my_size
        }

        fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
            if !self.name.is_empty() {
                os.write_string(1, &self.name)?;
            }
            if let Some(v) = self.elapsed.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
            }
            if let Some(v) = self.wall.as_ref() {
                ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
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

        fn new() -> DiagTime {
            DiagTime::new()
        }

        fn clear(&mut self) {
            self.name.clear();
            self.elapsed.clear();
            self.wall.clear();
            self.special_fields.clear();
        }

        fn default_instance() -> &'static DiagTime {
            static instance: DiagTime = DiagTime {
                name: ::std::string::String::new(),
                elapsed: ::protobuf::MessageField::none(),
                wall: ::protobuf::MessageField::none(),
                special_fields: ::protobuf::SpecialFields::new(),
            };
            &instance
        }
    }

    impl ::protobuf::MessageFull for DiagTime {
        fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().message_by_package_relative_name("Diagnostics.DiagTime").unwrap()).clone()
        }
    }

    impl ::std::fmt::Display for DiagTime {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            ::protobuf::text_format::fmt(self, f)
        }
    }

    impl ::protobuf::reflect::ProtobufValue for DiagTime {
        type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fignition/msgs/diagnostics.proto\x12\rignition.msgs\x1a\x18ignition\
    /msgs/time.proto\x1a\x1aignition/msgs/header.proto\"\xf9\x02\n\x0bDiagno\
    stics\x12-\n\x06header\x18\x01\x20\x01(\x0b2\x15.ignition.msgs.HeaderR\
    \x06header\x127\n\x04time\x18\x02\x20\x03(\x0b2#.ignition.msgs.Diagnosti\
    cs.DiagTimeR\x04time\x120\n\treal_time\x18\x03\x20\x01(\x0b2\x13.ignitio\
    n.msgs.TimeR\x08realTime\x12.\n\x08sim_time\x18\x04\x20\x01(\x0b2\x13.ig\
    nition.msgs.TimeR\x07simTime\x12(\n\x10real_time_factor\x18\x05\x20\x01(\
    \x01R\x0erealTimeFactor\x1av\n\x08DiagTime\x12\x12\n\x04name\x18\x01\x20\
    \x01(\tR\x04name\x12-\n\x07elapsed\x18\x02\x20\x01(\x0b2\x13.ignition.ms\
    gs.TimeR\x07elapsed\x12'\n\x04wall\x18\x03\x20\x01(\x0b2\x13.ignition.ms\
    gs.TimeR\x04wallB&\n\x11com.ignition.msgsB\x11DiagnosticsProtosb\x06prot\
    o3\
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
            deps.push(super::time::file_descriptor().clone());
            deps.push(super::header::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(Diagnostics::generated_message_descriptor_data());
            messages.push(diagnostics::DiagTime::generated_message_descriptor_data());
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
