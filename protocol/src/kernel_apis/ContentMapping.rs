// This file is generated by rust-protobuf 3.1.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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

//! Generated file from `kernel/apis/common/ContentMapping.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_1_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:ContentMapping)
pub struct ContentMapping {
    // message fields
    // @@protoc_insertion_point(field:ContentMapping.file)
    pub file: ::std::string::String,
    // @@protoc_insertion_point(field:ContentMapping.hash)
    pub hash: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:ContentMapping.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ContentMapping {
    fn default() -> &'a ContentMapping {
        <ContentMapping as ::protobuf::Message>::default_instance()
    }
}

impl ContentMapping {
    pub fn new() -> ContentMapping {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "file",
            |m: &ContentMapping| { &m.file },
            |m: &mut ContentMapping| { &mut m.file },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hash",
            |m: &ContentMapping| { &m.hash },
            |m: &mut ContentMapping| { &mut m.hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ContentMapping>(
            "ContentMapping",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ContentMapping {
    const NAME: &'static str = "ContentMapping";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.file = is.read_string()?;
                },
                18 => {
                    self.hash = is.read_string()?;
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
        if !self.file.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.file);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.file.is_empty() {
            os.write_string(1, &self.file)?;
        }
        if !self.hash.is_empty() {
            os.write_string(2, &self.hash)?;
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

    fn new() -> ContentMapping {
        ContentMapping::new()
    }

    fn clear(&mut self) {
        self.file.clear();
        self.hash.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ContentMapping {
        static instance: ContentMapping = ContentMapping {
            file: ::std::string::String::new(),
            hash: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ContentMapping {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ContentMapping").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ContentMapping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContentMapping {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'kernel/apis/common/ContentMapping.proto\"8\n\x0eContentMapping\x12\
    \x12\n\x04file\x18\x01\x20\x01(\tR\x04file\x12\x12\n\x04hash\x18\x02\x20\
    \x01(\tR\x04hashJ\x98\x01\n\x06\x12\x04\0\0\x05\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x05\x01\n\n\n\x03\x04\0\x01\
    \x12\x03\x02\x08\x16\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x14\n\x0c\n\
    \x05\x04\0\x02\0\x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\
    \x03\x03\x0b\x0f\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x12\x13\n\x0b\n\
    \x04\x04\0\x02\x01\x12\x03\x04\x04\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x0b\x0f\n\x0c\n\
    \x05\x04\0\x02\x01\x03\x12\x03\x04\x12\x13b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ContentMapping::generated_message_descriptor_data());
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
