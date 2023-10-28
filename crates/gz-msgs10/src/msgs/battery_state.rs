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

//! Generated file from `gz/msgs/battery_state.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_3_0;

#[derive(::gz_msgs_common::GzMessage)]
// @@protoc_insertion_point(message:gz.msgs.BatteryState)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BatteryState {
    // message fields
    // @@protoc_insertion_point(field:gz.msgs.BatteryState.header)
    pub header: ::protobuf::MessageField<super::header::Header>,
    // @@protoc_insertion_point(field:gz.msgs.BatteryState.voltage)
    pub voltage: f64,
    // @@protoc_insertion_point(field:gz.msgs.BatteryState.current)
    pub current: f64,
    // @@protoc_insertion_point(field:gz.msgs.BatteryState.charge)
    pub charge: f64,
    // @@protoc_insertion_point(field:gz.msgs.BatteryState.capacity)
    pub capacity: f64,
    // @@protoc_insertion_point(field:gz.msgs.BatteryState.percentage)
    pub percentage: f64,
    // @@protoc_insertion_point(field:gz.msgs.BatteryState.power_supply_status)
    pub power_supply_status: ::protobuf::EnumOrUnknown<battery_state::PowerSupplyStatus>,
    // special fields
    // @@protoc_insertion_point(special_field:gz.msgs.BatteryState.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BatteryState {
    fn default() -> &'a BatteryState {
        <BatteryState as ::protobuf::Message>::default_instance()
    }
}

impl BatteryState {
    pub fn new() -> BatteryState {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::header::Header>(
            "header",
            |m: &BatteryState| { &m.header },
            |m: &mut BatteryState| { &mut m.header },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "voltage",
            |m: &BatteryState| { &m.voltage },
            |m: &mut BatteryState| { &mut m.voltage },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "current",
            |m: &BatteryState| { &m.current },
            |m: &mut BatteryState| { &mut m.current },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "charge",
            |m: &BatteryState| { &m.charge },
            |m: &mut BatteryState| { &mut m.charge },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "capacity",
            |m: &BatteryState| { &m.capacity },
            |m: &mut BatteryState| { &mut m.capacity },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "percentage",
            |m: &BatteryState| { &m.percentage },
            |m: &mut BatteryState| { &mut m.percentage },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "power_supply_status",
            |m: &BatteryState| { &m.power_supply_status },
            |m: &mut BatteryState| { &mut m.power_supply_status },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BatteryState>(
            "BatteryState",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BatteryState {
    const NAME: &'static str = "BatteryState";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.header)?;
                },
                17 => {
                    self.voltage = is.read_double()?;
                },
                25 => {
                    self.current = is.read_double()?;
                },
                33 => {
                    self.charge = is.read_double()?;
                },
                41 => {
                    self.capacity = is.read_double()?;
                },
                49 => {
                    self.percentage = is.read_double()?;
                },
                56 => {
                    self.power_supply_status = is.read_enum_or_unknown()?;
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
        if self.voltage != 0. {
            my_size += 1 + 8;
        }
        if self.current != 0. {
            my_size += 1 + 8;
        }
        if self.charge != 0. {
            my_size += 1 + 8;
        }
        if self.capacity != 0. {
            my_size += 1 + 8;
        }
        if self.percentage != 0. {
            my_size += 1 + 8;
        }
        if self.power_supply_status != ::protobuf::EnumOrUnknown::new(battery_state::PowerSupplyStatus::UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(7, self.power_supply_status.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.header.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.voltage != 0. {
            os.write_double(2, self.voltage)?;
        }
        if self.current != 0. {
            os.write_double(3, self.current)?;
        }
        if self.charge != 0. {
            os.write_double(4, self.charge)?;
        }
        if self.capacity != 0. {
            os.write_double(5, self.capacity)?;
        }
        if self.percentage != 0. {
            os.write_double(6, self.percentage)?;
        }
        if self.power_supply_status != ::protobuf::EnumOrUnknown::new(battery_state::PowerSupplyStatus::UNKNOWN) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.power_supply_status))?;
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

    fn new() -> BatteryState {
        BatteryState::new()
    }

    fn clear(&mut self) {
        self.header.clear();
        self.voltage = 0.;
        self.current = 0.;
        self.charge = 0.;
        self.capacity = 0.;
        self.percentage = 0.;
        self.power_supply_status = ::protobuf::EnumOrUnknown::new(battery_state::PowerSupplyStatus::UNKNOWN);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BatteryState {
        static instance: BatteryState = BatteryState {
            header: ::protobuf::MessageField::none(),
            voltage: 0.,
            current: 0.,
            charge: 0.,
            capacity: 0.,
            percentage: 0.,
            power_supply_status: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BatteryState {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BatteryState").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BatteryState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BatteryState {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `BatteryState`
pub mod battery_state {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:gz.msgs.BatteryState.PowerSupplyStatus)
    pub enum PowerSupplyStatus {
        // @@protoc_insertion_point(enum_value:gz.msgs.BatteryState.PowerSupplyStatus.UNKNOWN)
        UNKNOWN = 0,
        // @@protoc_insertion_point(enum_value:gz.msgs.BatteryState.PowerSupplyStatus.CHARGING)
        CHARGING = 1,
        // @@protoc_insertion_point(enum_value:gz.msgs.BatteryState.PowerSupplyStatus.DISCHARGING)
        DISCHARGING = 2,
        // @@protoc_insertion_point(enum_value:gz.msgs.BatteryState.PowerSupplyStatus.NOT_CHARGING)
        NOT_CHARGING = 3,
        // @@protoc_insertion_point(enum_value:gz.msgs.BatteryState.PowerSupplyStatus.FULL)
        FULL = 4,
    }

    impl ::protobuf::Enum for PowerSupplyStatus {
        const NAME: &'static str = "PowerSupplyStatus";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<PowerSupplyStatus> {
            match value {
                0 => ::std::option::Option::Some(PowerSupplyStatus::UNKNOWN),
                1 => ::std::option::Option::Some(PowerSupplyStatus::CHARGING),
                2 => ::std::option::Option::Some(PowerSupplyStatus::DISCHARGING),
                3 => ::std::option::Option::Some(PowerSupplyStatus::NOT_CHARGING),
                4 => ::std::option::Option::Some(PowerSupplyStatus::FULL),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<PowerSupplyStatus> {
            match str {
                "UNKNOWN" => ::std::option::Option::Some(PowerSupplyStatus::UNKNOWN),
                "CHARGING" => ::std::option::Option::Some(PowerSupplyStatus::CHARGING),
                "DISCHARGING" => ::std::option::Option::Some(PowerSupplyStatus::DISCHARGING),
                "NOT_CHARGING" => ::std::option::Option::Some(PowerSupplyStatus::NOT_CHARGING),
                "FULL" => ::std::option::Option::Some(PowerSupplyStatus::FULL),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [PowerSupplyStatus] = &[
            PowerSupplyStatus::UNKNOWN,
            PowerSupplyStatus::CHARGING,
            PowerSupplyStatus::DISCHARGING,
            PowerSupplyStatus::NOT_CHARGING,
            PowerSupplyStatus::FULL,
        ];
    }

    impl ::protobuf::EnumFull for PowerSupplyStatus {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("BatteryState.PowerSupplyStatus").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for PowerSupplyStatus {
        fn default() -> Self {
            PowerSupplyStatus::UNKNOWN
        }
    }

    impl PowerSupplyStatus {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<PowerSupplyStatus>("BatteryState.PowerSupplyStatus")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bgz/msgs/battery_state.proto\x12\x07gz.msgs\x1a\x14gz/msgs/header.p\
    roto\"\xf5\x02\n\x0cBatteryState\x12'\n\x06header\x18\x01\x20\x01(\x0b2\
    \x0f.gz.msgs.HeaderR\x06header\x12\x18\n\x07voltage\x18\x02\x20\x01(\x01\
    R\x07voltage\x12\x18\n\x07current\x18\x03\x20\x01(\x01R\x07current\x12\
    \x16\n\x06charge\x18\x04\x20\x01(\x01R\x06charge\x12\x1a\n\x08capacity\
    \x18\x05\x20\x01(\x01R\x08capacity\x12\x1e\n\npercentage\x18\x06\x20\x01\
    (\x01R\npercentage\x12W\n\x13power_supply_status\x18\x07\x20\x01(\x0e2'.\
    gz.msgs.BatteryState.PowerSupplyStatusR\x11powerSupplyStatus\"[\n\x11Pow\
    erSupplyStatus\x12\x0b\n\x07UNKNOWN\x10\0\x12\x0c\n\x08CHARGING\x10\x01\
    \x12\x0f\n\x0bDISCHARGING\x10\x02\x12\x10\n\x0cNOT_CHARGING\x10\x03\x12\
    \x08\n\x04FULL\x10\x04B!\n\x0bcom.gz.msgsB\x12BatteryStateProtosb\x06pro\
    to3\
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
            messages.push(BatteryState::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(battery_state::PowerSupplyStatus::generated_enum_descriptor_data());
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
