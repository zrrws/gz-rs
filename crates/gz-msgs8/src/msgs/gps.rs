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

//! Generated file from `ignition/msgs/gps.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_common::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.GPS)
pub struct GPS {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.GPS.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.GPS.link_name)
    pub link_name: ::std::string::String,
    // @@protoc_insertion_point(field:ignition.msgs.GPS.latitude_deg)
    pub latitude_deg: f64,
    // @@protoc_insertion_point(field:ignition.msgs.GPS.longitude_deg)
    pub longitude_deg: f64,
    // @@protoc_insertion_point(field:ignition.msgs.GPS.altitude)
    pub altitude: f64,
    // @@protoc_insertion_point(field:ignition.msgs.GPS.velocity_east)
    pub velocity_east: f64,
    // @@protoc_insertion_point(field:ignition.msgs.GPS.velocity_north)
    pub velocity_north: f64,
    // @@protoc_insertion_point(field:ignition.msgs.GPS.velocity_up)
    pub velocity_up: f64,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.GPS.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GPS {
    fn default() -> &'a GPS {
        <GPS as ::protobuf::Message>::default_instance()
    }
}

impl GPS {
    pub fn new() -> GPS {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &GPS| { &m.header },
            |m: &mut GPS| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "link_name",
            |m: &GPS| { &m.link_name },
            |m: &mut GPS| { &mut m.link_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "latitude_deg",
            |m: &GPS| { &m.latitude_deg },
            |m: &mut GPS| { &mut m.latitude_deg },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "longitude_deg",
            |m: &GPS| { &m.longitude_deg },
            |m: &mut GPS| { &mut m.longitude_deg },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "altitude",
            |m: &GPS| { &m.altitude },
            |m: &mut GPS| { &mut m.altitude },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "velocity_east",
            |m: &GPS| { &m.velocity_east },
            |m: &mut GPS| { &mut m.velocity_east },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "velocity_north",
            |m: &GPS| { &m.velocity_north },
            |m: &mut GPS| { &mut m.velocity_north },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "velocity_up",
            |m: &GPS| { &m.velocity_up },
            |m: &mut GPS| { &mut m.velocity_up },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GPS>(
            "GPS",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GPS {
    const NAME: &'static str = "GPS";

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
                    self.link_name = is.read_string()?;
                },
                25 => {
                    self.latitude_deg = is.read_double()?;
                },
                33 => {
                    self.longitude_deg = is.read_double()?;
                },
                41 => {
                    self.altitude = is.read_double()?;
                },
                49 => {
                    self.velocity_east = is.read_double()?;
                },
                57 => {
                    self.velocity_north = is.read_double()?;
                },
                65 => {
                    self.velocity_up = is.read_double()?;
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
        if !self.link_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.link_name);
        }
        if self.latitude_deg != 0. {
            my_size += 1 + 8;
        }
        if self.longitude_deg != 0. {
            my_size += 1 + 8;
        }
        if self.altitude != 0. {
            my_size += 1 + 8;
        }
        if self.velocity_east != 0. {
            my_size += 1 + 8;
        }
        if self.velocity_north != 0. {
            my_size += 1 + 8;
        }
        if self.velocity_up != 0. {
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
        if !self.link_name.is_empty() {
            os.write_string(2, &self.link_name)?;
        }
        if self.latitude_deg != 0. {
            os.write_double(3, self.latitude_deg)?;
        }
        if self.longitude_deg != 0. {
            os.write_double(4, self.longitude_deg)?;
        }
        if self.altitude != 0. {
            os.write_double(5, self.altitude)?;
        }
        if self.velocity_east != 0. {
            os.write_double(6, self.velocity_east)?;
        }
        if self.velocity_north != 0. {
            os.write_double(7, self.velocity_north)?;
        }
        if self.velocity_up != 0. {
            os.write_double(8, self.velocity_up)?;
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

    fn new() -> GPS {
        GPS::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.link_name.clear();
        self.latitude_deg = 0.;
        self.longitude_deg = 0.;
        self.altitude = 0.;
        self.velocity_east = 0.;
        self.velocity_north = 0.;
        self.velocity_up = 0.;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GPS {
        static instance: GPS = GPS {
            header: ::protobuf::MessageField::none(),
            link_name: ::std::string::String::new(),
            latitude_deg: 0.,
            longitude_deg: 0.,
            altitude: 0.,
            velocity_east: 0.,
            velocity_north: 0.,
            velocity_up: 0.,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GPS {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GPS").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GPS {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GPS {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17ignition/msgs/gps.proto\x12\rignition.msgs\x1a\x1aignition/msgs/he\
    ader.proto\"\xa2\x02\n\x03GPS\x12-\n\x06header\x18\x01\x20\x01(\x0b2\x15\
    .ignition.msgs.HeaderR\x06header\x12\x1b\n\tlink_name\x18\x02\x20\x01(\t\
    R\x08linkName\x12!\n\x0clatitude_deg\x18\x03\x20\x01(\x01R\x0blatitudeDe\
    g\x12#\n\rlongitude_deg\x18\x04\x20\x01(\x01R\x0clongitudeDeg\x12\x1a\n\
    \x08altitude\x18\x05\x20\x01(\x01R\x08altitude\x12#\n\rvelocity_east\x18\
    \x06\x20\x01(\x01R\x0cvelocityEast\x12%\n\x0evelocity_north\x18\x07\x20\
    \x01(\x01R\rvelocityNorth\x12\x1f\n\x0bvelocity_up\x18\x08\x20\x01(\x01R\
    \nvelocityUpB\x1e\n\x11com.ignition.msgsB\tGPSProtosb\x06proto3\
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
            messages.push(GPS::generated_message_descriptor_data());
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
