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

//! Generated file from `ignition/msgs/serialized.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.SerializedComponent)
pub struct SerializedComponent {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.SerializedComponent.type)
    pub type_: u64,
    // @@protoc_insertion_point(field:ignition.msgs.SerializedComponent.component)
    pub component: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:ignition.msgs.SerializedComponent.remove)
    pub remove: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.SerializedComponent.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SerializedComponent {
    fn default() -> &'a SerializedComponent {
        <SerializedComponent as ::protobuf::Message>::default_instance()
    }
}

impl SerializedComponent {
    pub fn new() -> SerializedComponent {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "type",
            |m: &SerializedComponent| { &m.type_ },
            |m: &mut SerializedComponent| { &mut m.type_ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "component",
            |m: &SerializedComponent| { &m.component },
            |m: &mut SerializedComponent| { &mut m.component },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "remove",
            |m: &SerializedComponent| { &m.remove },
            |m: &mut SerializedComponent| { &mut m.remove },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SerializedComponent>(
            "SerializedComponent",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SerializedComponent {
    const NAME: &'static str = "SerializedComponent";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.type_ = is.read_uint64()?;
                },
                18 => {
                    self.component = is.read_bytes()?;
                },
                24 => {
                    self.remove = is.read_bool()?;
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
        if self.type_ != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.type_);
        }
        if !self.component.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.component);
        }
        if self.remove != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.type_ != 0 {
            os.write_uint64(1, self.type_)?;
        }
        if !self.component.is_empty() {
            os.write_bytes(2, &self.component)?;
        }
        if self.remove != false {
            os.write_bool(3, self.remove)?;
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

    fn new() -> SerializedComponent {
        SerializedComponent::new()
    }

    fn clear(&mut self) {
        self.type_ = 0;
        self.component.clear();
        self.remove = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SerializedComponent {
        static instance: SerializedComponent = SerializedComponent {
            type_: 0,
            component: ::std::vec::Vec::new(),
            remove: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SerializedComponent {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SerializedComponent").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SerializedComponent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SerializedComponent {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.SerializedEntity)
pub struct SerializedEntity {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.SerializedEntity.id)
    pub id: u64,
    // @@protoc_insertion_point(field:ignition.msgs.SerializedEntity.components)
    pub components: ::std::vec::Vec<SerializedComponent>,
    // @@protoc_insertion_point(field:ignition.msgs.SerializedEntity.remove)
    pub remove: bool,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.SerializedEntity.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SerializedEntity {
    fn default() -> &'a SerializedEntity {
        <SerializedEntity as ::protobuf::Message>::default_instance()
    }
}

impl SerializedEntity {
    pub fn new() -> SerializedEntity {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &SerializedEntity| { &m.id },
            |m: &mut SerializedEntity| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "components",
            |m: &SerializedEntity| { &m.components },
            |m: &mut SerializedEntity| { &mut m.components },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "remove",
            |m: &SerializedEntity| { &m.remove },
            |m: &mut SerializedEntity| { &mut m.remove },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SerializedEntity>(
            "SerializedEntity",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SerializedEntity {
    const NAME: &'static str = "SerializedEntity";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.id = is.read_uint64()?;
                },
                18 => {
                    self.components.push(is.read_message()?);
                },
                24 => {
                    self.remove = is.read_bool()?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint64_size(1, self.id);
        }
        for value in &self.components {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.remove != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint64(1, self.id)?;
        }
        for v in &self.components {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if self.remove != false {
            os.write_bool(3, self.remove)?;
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

    fn new() -> SerializedEntity {
        SerializedEntity::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.components.clear();
        self.remove = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SerializedEntity {
        static instance: SerializedEntity = SerializedEntity {
            id: 0,
            components: ::std::vec::Vec::new(),
            remove: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SerializedEntity {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SerializedEntity").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SerializedEntity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SerializedEntity {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.SerializedState)
pub struct SerializedState {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.SerializedState.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:ignition.msgs.SerializedState.entities)
    pub entities: ::std::vec::Vec<SerializedEntity>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.SerializedState.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SerializedState {
    fn default() -> &'a SerializedState {
        <SerializedState as ::protobuf::Message>::default_instance()
    }
}

impl SerializedState {
    pub fn new() -> SerializedState {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &SerializedState| { &m.header },
            |m: &mut SerializedState| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "entities",
            |m: &SerializedState| { &m.entities },
            |m: &mut SerializedState| { &mut m.entities },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SerializedState>(
            "SerializedState",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SerializedState {
    const NAME: &'static str = "SerializedState";

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
                    self.entities.push(is.read_message()?);
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
        for value in &self.entities {
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
        for v in &self.entities {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> SerializedState {
        SerializedState::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.entities.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SerializedState {
        static instance: SerializedState = SerializedState {
            header: ::protobuf::MessageField::none(),
            entities: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SerializedState {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SerializedState").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SerializedState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SerializedState {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
#[derive(::gz_msgs_derive::IgnMessage)]
// @@protoc_insertion_point(message:ignition.msgs.SerializedStep)
pub struct SerializedStep {
    // message fields
    // @@protoc_insertion_point(field:ignition.msgs.SerializedStep.stats)
    pub stats: ::protobuf::MessageField<super::world_stats::WorldStatistics>,
    // @@protoc_insertion_point(field:ignition.msgs.SerializedStep.state)
    pub state: ::protobuf::MessageField<SerializedState>,
    // special fields
    // @@protoc_insertion_point(special_field:ignition.msgs.SerializedStep.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SerializedStep {
    fn default() -> &'a SerializedStep {
        <SerializedStep as ::protobuf::Message>::default_instance()
    }
}

impl SerializedStep {
    pub fn new() -> SerializedStep {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::world_stats::WorldStatistics>(
            "stats",
            |m: &SerializedStep| { &m.stats },
            |m: &mut SerializedStep| { &mut m.stats },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, SerializedState>(
            "state",
            |m: &SerializedStep| { &m.state },
            |m: &mut SerializedStep| { &mut m.state },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SerializedStep>(
            "SerializedStep",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SerializedStep {
    const NAME: &'static str = "SerializedStep";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.stats)?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.state)?;
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
        if let Some(v) = self.stats.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.stats.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if let Some(v) = self.state.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> SerializedStep {
        SerializedStep::new()
    }

    fn clear(&mut self) {
        self.stats.clear();
        self.state.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SerializedStep {
        static instance: SerializedStep = SerializedStep {
            stats: ::protobuf::MessageField::none(),
            state: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SerializedStep {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SerializedStep").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SerializedStep {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SerializedStep {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eignition/msgs/serialized.proto\x12\rignition.msgs\x1a\x1aignition/\
    msgs/header.proto\x1a\x1fignition/msgs/world_stats.proto\"_\n\x13Seriali\
    zedComponent\x12\x12\n\x04type\x18\x01\x20\x01(\x04R\x04type\x12\x1c\n\t\
    component\x18\x02\x20\x01(\x0cR\tcomponent\x12\x16\n\x06remove\x18\x03\
    \x20\x01(\x08R\x06remove\"~\n\x10SerializedEntity\x12\x0e\n\x02id\x18\
    \x01\x20\x01(\x04R\x02id\x12B\n\ncomponents\x18\x02\x20\x03(\x0b2\".igni\
    tion.msgs.SerializedComponentR\ncomponents\x12\x16\n\x06remove\x18\x03\
    \x20\x01(\x08R\x06remove\"}\n\x0fSerializedState\x12-\n\x06header\x18\
    \x01\x20\x01(\x0b2\x15.ignition.msgs.HeaderR\x06header\x12;\n\x08entitie\
    s\x18\x02\x20\x03(\x0b2\x1f.ignition.msgs.SerializedEntityR\x08entities\
    \"|\n\x0eSerializedStep\x124\n\x05stats\x18\x01\x20\x01(\x0b2\x1e.igniti\
    on.msgs.WorldStatisticsR\x05stats\x124\n\x05state\x18\x02\x20\x01(\x0b2\
    \x1e.ignition.msgs.SerializedStateR\x05stateB\x13\n\x11com.ignition.msgs\
    b\x06proto3\
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
            deps.push(super::world_stats::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(4);
            messages.push(SerializedComponent::generated_message_descriptor_data());
            messages.push(SerializedEntity::generated_message_descriptor_data());
            messages.push(SerializedState::generated_message_descriptor_data());
            messages.push(SerializedStep::generated_message_descriptor_data());
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
