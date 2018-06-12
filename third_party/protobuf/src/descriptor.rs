// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use std::string::ToString;
use std::boxed::Box;

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct FileDescriptorSet {
    // message fields
    file: ::protobuf::RepeatedField<FileDescriptorProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileDescriptorSet {}

impl FileDescriptorSet {
    pub fn new() -> FileDescriptorSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileDescriptorSet {
        static mut instance: ::protobuf::lazy::Lazy<FileDescriptorSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileDescriptorSet,
        };
        unsafe {
            instance.get(FileDescriptorSet::new)
        }
    }

    // repeated .google.protobuf.FileDescriptorProto file = 1;

    pub fn clear_file(&mut self) {
        self.file.clear();
    }

    // Param is passed by value, moved
    pub fn set_file(&mut self, v: ::protobuf::RepeatedField<FileDescriptorProto>) {
        self.file = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file(&mut self) -> &mut ::protobuf::RepeatedField<FileDescriptorProto> {
        &mut self.file
    }

    // Take field
    pub fn take_file(&mut self) -> ::protobuf::RepeatedField<FileDescriptorProto> {
        ::std::mem::replace(&mut self.file, ::protobuf::RepeatedField::new())
    }

    pub fn get_file(&self) -> &[FileDescriptorProto] {
        &self.file
    }

    fn get_file_for_reflect(&self) -> &::protobuf::RepeatedField<FileDescriptorProto> {
        &self.file
    }

    fn mut_file_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FileDescriptorProto> {
        &mut self.file
    }
}

impl ::protobuf::Message for FileDescriptorSet {
    fn is_initialized(&self) -> bool {
        for v in &self.file {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.file)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.file {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.file {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileDescriptorSet {
    fn new() -> FileDescriptorSet {
        FileDescriptorSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileDescriptorSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileDescriptorProto>>(
                    "file",
                    FileDescriptorSet::get_file_for_reflect,
                    FileDescriptorSet::mut_file_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileDescriptorSet>(
                    "FileDescriptorSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileDescriptorSet {
    fn clear(&mut self) {
        self.clear_file();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileDescriptorSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileDescriptorSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileDescriptorProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    package: ::protobuf::SingularField<::std::string::String>,
    dependency: ::protobuf::RepeatedField<::std::string::String>,
    public_dependency: ::std::vec::Vec<i32>,
    weak_dependency: ::std::vec::Vec<i32>,
    message_type: ::protobuf::RepeatedField<DescriptorProto>,
    enum_type: ::protobuf::RepeatedField<EnumDescriptorProto>,
    service: ::protobuf::RepeatedField<ServiceDescriptorProto>,
    extension: ::protobuf::RepeatedField<FieldDescriptorProto>,
    options: ::protobuf::SingularPtrField<FileOptions>,
    source_code_info: ::protobuf::SingularPtrField<SourceCodeInfo>,
    syntax: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileDescriptorProto {}

impl FileDescriptorProto {
    pub fn new() -> FileDescriptorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileDescriptorProto {
        static mut instance: ::protobuf::lazy::Lazy<FileDescriptorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileDescriptorProto,
        };
        unsafe {
            instance.get(FileDescriptorProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string package = 2;

    pub fn clear_package(&mut self) {
        self.package.clear();
    }

    pub fn has_package(&self) -> bool {
        self.package.is_some()
    }

    // Param is passed by value, moved
    pub fn set_package(&mut self, v: ::std::string::String) {
        self.package = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_package(&mut self) -> &mut ::std::string::String {
        if self.package.is_none() {
            self.package.set_default();
        }
        self.package.as_mut().unwrap()
    }

    // Take field
    pub fn take_package(&mut self) -> ::std::string::String {
        self.package.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_package(&self) -> &str {
        match self.package.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_package_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.package
    }

    fn mut_package_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.package
    }

    // repeated string dependency = 3;

    pub fn clear_dependency(&mut self) {
        self.dependency.clear();
    }

    // Param is passed by value, moved
    pub fn set_dependency(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.dependency = v;
    }

    // Mutable pointer to the field.
    pub fn mut_dependency(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.dependency
    }

    // Take field
    pub fn take_dependency(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.dependency, ::protobuf::RepeatedField::new())
    }

    pub fn get_dependency(&self) -> &[::std::string::String] {
        &self.dependency
    }

    fn get_dependency_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.dependency
    }

    fn mut_dependency_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.dependency
    }

    // repeated int32 public_dependency = 10;

    pub fn clear_public_dependency(&mut self) {
        self.public_dependency.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_dependency(&mut self, v: ::std::vec::Vec<i32>) {
        self.public_dependency = v;
    }

    // Mutable pointer to the field.
    pub fn mut_public_dependency(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.public_dependency
    }

    // Take field
    pub fn take_public_dependency(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.public_dependency, ::std::vec::Vec::new())
    }

    pub fn get_public_dependency(&self) -> &[i32] {
        &self.public_dependency
    }

    fn get_public_dependency_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.public_dependency
    }

    fn mut_public_dependency_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.public_dependency
    }

    // repeated int32 weak_dependency = 11;

    pub fn clear_weak_dependency(&mut self) {
        self.weak_dependency.clear();
    }

    // Param is passed by value, moved
    pub fn set_weak_dependency(&mut self, v: ::std::vec::Vec<i32>) {
        self.weak_dependency = v;
    }

    // Mutable pointer to the field.
    pub fn mut_weak_dependency(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.weak_dependency
    }

    // Take field
    pub fn take_weak_dependency(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.weak_dependency, ::std::vec::Vec::new())
    }

    pub fn get_weak_dependency(&self) -> &[i32] {
        &self.weak_dependency
    }

    fn get_weak_dependency_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.weak_dependency
    }

    fn mut_weak_dependency_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.weak_dependency
    }

    // repeated .google.protobuf.DescriptorProto message_type = 4;

    pub fn clear_message_type(&mut self) {
        self.message_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_message_type(&mut self, v: ::protobuf::RepeatedField<DescriptorProto>) {
        self.message_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_message_type(&mut self) -> &mut ::protobuf::RepeatedField<DescriptorProto> {
        &mut self.message_type
    }

    // Take field
    pub fn take_message_type(&mut self) -> ::protobuf::RepeatedField<DescriptorProto> {
        ::std::mem::replace(&mut self.message_type, ::protobuf::RepeatedField::new())
    }

    pub fn get_message_type(&self) -> &[DescriptorProto] {
        &self.message_type
    }

    fn get_message_type_for_reflect(&self) -> &::protobuf::RepeatedField<DescriptorProto> {
        &self.message_type
    }

    fn mut_message_type_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DescriptorProto> {
        &mut self.message_type
    }

    // repeated .google.protobuf.EnumDescriptorProto enum_type = 5;

    pub fn clear_enum_type(&mut self) {
        self.enum_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_enum_type(&mut self, v: ::protobuf::RepeatedField<EnumDescriptorProto>) {
        self.enum_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enum_type(&mut self) -> &mut ::protobuf::RepeatedField<EnumDescriptorProto> {
        &mut self.enum_type
    }

    // Take field
    pub fn take_enum_type(&mut self) -> ::protobuf::RepeatedField<EnumDescriptorProto> {
        ::std::mem::replace(&mut self.enum_type, ::protobuf::RepeatedField::new())
    }

    pub fn get_enum_type(&self) -> &[EnumDescriptorProto] {
        &self.enum_type
    }

    fn get_enum_type_for_reflect(&self) -> &::protobuf::RepeatedField<EnumDescriptorProto> {
        &self.enum_type
    }

    fn mut_enum_type_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EnumDescriptorProto> {
        &mut self.enum_type
    }

    // repeated .google.protobuf.ServiceDescriptorProto service = 6;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ::protobuf::RepeatedField<ServiceDescriptorProto>) {
        self.service = v;
    }

    // Mutable pointer to the field.
    pub fn mut_service(&mut self) -> &mut ::protobuf::RepeatedField<ServiceDescriptorProto> {
        &mut self.service
    }

    // Take field
    pub fn take_service(&mut self) -> ::protobuf::RepeatedField<ServiceDescriptorProto> {
        ::std::mem::replace(&mut self.service, ::protobuf::RepeatedField::new())
    }

    pub fn get_service(&self) -> &[ServiceDescriptorProto] {
        &self.service
    }

    fn get_service_for_reflect(&self) -> &::protobuf::RepeatedField<ServiceDescriptorProto> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ServiceDescriptorProto> {
        &mut self.service
    }

    // repeated .google.protobuf.FieldDescriptorProto extension = 7;

    pub fn clear_extension(&mut self) {
        self.extension.clear();
    }

    // Param is passed by value, moved
    pub fn set_extension(&mut self, v: ::protobuf::RepeatedField<FieldDescriptorProto>) {
        self.extension = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extension(&mut self) -> &mut ::protobuf::RepeatedField<FieldDescriptorProto> {
        &mut self.extension
    }

    // Take field
    pub fn take_extension(&mut self) -> ::protobuf::RepeatedField<FieldDescriptorProto> {
        ::std::mem::replace(&mut self.extension, ::protobuf::RepeatedField::new())
    }

    pub fn get_extension(&self) -> &[FieldDescriptorProto] {
        &self.extension
    }

    fn get_extension_for_reflect(&self) -> &::protobuf::RepeatedField<FieldDescriptorProto> {
        &self.extension
    }

    fn mut_extension_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FieldDescriptorProto> {
        &mut self.extension
    }

    // optional .google.protobuf.FileOptions options = 8;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: FileOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut FileOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> FileOptions {
        self.options.take().unwrap_or_else(|| FileOptions::new())
    }

    pub fn get_options(&self) -> &FileOptions {
        self.options.as_ref().unwrap_or_else(|| FileOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<FileOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FileOptions> {
        &mut self.options
    }

    // optional .google.protobuf.SourceCodeInfo source_code_info = 9;

    pub fn clear_source_code_info(&mut self) {
        self.source_code_info.clear();
    }

    pub fn has_source_code_info(&self) -> bool {
        self.source_code_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_code_info(&mut self, v: SourceCodeInfo) {
        self.source_code_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_code_info(&mut self) -> &mut SourceCodeInfo {
        if self.source_code_info.is_none() {
            self.source_code_info.set_default();
        }
        self.source_code_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_code_info(&mut self) -> SourceCodeInfo {
        self.source_code_info.take().unwrap_or_else(|| SourceCodeInfo::new())
    }

    pub fn get_source_code_info(&self) -> &SourceCodeInfo {
        self.source_code_info.as_ref().unwrap_or_else(|| SourceCodeInfo::default_instance())
    }

    fn get_source_code_info_for_reflect(&self) -> &::protobuf::SingularPtrField<SourceCodeInfo> {
        &self.source_code_info
    }

    fn mut_source_code_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SourceCodeInfo> {
        &mut self.source_code_info
    }

    // optional string syntax = 12;

    pub fn clear_syntax(&mut self) {
        self.syntax.clear();
    }

    pub fn has_syntax(&self) -> bool {
        self.syntax.is_some()
    }

    // Param is passed by value, moved
    pub fn set_syntax(&mut self, v: ::std::string::String) {
        self.syntax = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_syntax(&mut self) -> &mut ::std::string::String {
        if self.syntax.is_none() {
            self.syntax.set_default();
        }
        self.syntax.as_mut().unwrap()
    }

    // Take field
    pub fn take_syntax(&mut self) -> ::std::string::String {
        self.syntax.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_syntax(&self) -> &str {
        match self.syntax.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_syntax_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.syntax
    }

    fn mut_syntax_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.syntax
    }
}

impl ::protobuf::Message for FileDescriptorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.message_type {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.enum_type {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.service {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.extension {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.source_code_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.package)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.dependency)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.public_dependency)?;
                },
                11 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.weak_dependency)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.message_type)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.enum_type)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.service)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extension)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                9 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.source_code_info)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.syntax)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.package.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.dependency {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.public_dependency {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.weak_dependency {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.message_type {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.enum_type {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.service {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.extension {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.source_code_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.syntax.as_ref() {
            my_size += ::protobuf::rt::string_size(12, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.package.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.dependency {
            os.write_string(3, &v)?;
        };
        for v in &self.public_dependency {
            os.write_int32(10, *v)?;
        };
        for v in &self.weak_dependency {
            os.write_int32(11, *v)?;
        };
        for v in &self.message_type {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.enum_type {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.service {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.extension {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.source_code_info.as_ref() {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.syntax.as_ref() {
            os.write_string(12, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileDescriptorProto {
    fn new() -> FileDescriptorProto {
        FileDescriptorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileDescriptorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    FileDescriptorProto::get_name_for_reflect,
                    FileDescriptorProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "package",
                    FileDescriptorProto::get_package_for_reflect,
                    FileDescriptorProto::mut_package_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "dependency",
                    FileDescriptorProto::get_dependency_for_reflect,
                    FileDescriptorProto::mut_dependency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "public_dependency",
                    FileDescriptorProto::get_public_dependency_for_reflect,
                    FileDescriptorProto::mut_public_dependency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "weak_dependency",
                    FileDescriptorProto::get_weak_dependency_for_reflect,
                    FileDescriptorProto::mut_weak_dependency_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DescriptorProto>>(
                    "message_type",
                    FileDescriptorProto::get_message_type_for_reflect,
                    FileDescriptorProto::mut_message_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EnumDescriptorProto>>(
                    "enum_type",
                    FileDescriptorProto::get_enum_type_for_reflect,
                    FileDescriptorProto::mut_enum_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ServiceDescriptorProto>>(
                    "service",
                    FileDescriptorProto::get_service_for_reflect,
                    FileDescriptorProto::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FieldDescriptorProto>>(
                    "extension",
                    FileDescriptorProto::get_extension_for_reflect,
                    FileDescriptorProto::mut_extension_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FileOptions>>(
                    "options",
                    FileDescriptorProto::get_options_for_reflect,
                    FileDescriptorProto::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SourceCodeInfo>>(
                    "source_code_info",
                    FileDescriptorProto::get_source_code_info_for_reflect,
                    FileDescriptorProto::mut_source_code_info_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "syntax",
                    FileDescriptorProto::get_syntax_for_reflect,
                    FileDescriptorProto::mut_syntax_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileDescriptorProto>(
                    "FileDescriptorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileDescriptorProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_package();
        self.clear_dependency();
        self.clear_public_dependency();
        self.clear_weak_dependency();
        self.clear_message_type();
        self.clear_enum_type();
        self.clear_service();
        self.clear_extension();
        self.clear_options();
        self.clear_source_code_info();
        self.clear_syntax();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileDescriptorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileDescriptorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DescriptorProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    field: ::protobuf::RepeatedField<FieldDescriptorProto>,
    extension: ::protobuf::RepeatedField<FieldDescriptorProto>,
    nested_type: ::protobuf::RepeatedField<DescriptorProto>,
    enum_type: ::protobuf::RepeatedField<EnumDescriptorProto>,
    extension_range: ::protobuf::RepeatedField<DescriptorProto_ExtensionRange>,
    oneof_decl: ::protobuf::RepeatedField<OneofDescriptorProto>,
    options: ::protobuf::SingularPtrField<MessageOptions>,
    reserved_range: ::protobuf::RepeatedField<DescriptorProto_ReservedRange>,
    reserved_name: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DescriptorProto {}

impl DescriptorProto {
    pub fn new() -> DescriptorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DescriptorProto {
        static mut instance: ::protobuf::lazy::Lazy<DescriptorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DescriptorProto,
        };
        unsafe {
            instance.get(DescriptorProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated .google.protobuf.FieldDescriptorProto field = 2;

    pub fn clear_field(&mut self) {
        self.field.clear();
    }

    // Param is passed by value, moved
    pub fn set_field(&mut self, v: ::protobuf::RepeatedField<FieldDescriptorProto>) {
        self.field = v;
    }

    // Mutable pointer to the field.
    pub fn mut_field(&mut self) -> &mut ::protobuf::RepeatedField<FieldDescriptorProto> {
        &mut self.field
    }

    // Take field
    pub fn take_field(&mut self) -> ::protobuf::RepeatedField<FieldDescriptorProto> {
        ::std::mem::replace(&mut self.field, ::protobuf::RepeatedField::new())
    }

    pub fn get_field(&self) -> &[FieldDescriptorProto] {
        &self.field
    }

    fn get_field_for_reflect(&self) -> &::protobuf::RepeatedField<FieldDescriptorProto> {
        &self.field
    }

    fn mut_field_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FieldDescriptorProto> {
        &mut self.field
    }

    // repeated .google.protobuf.FieldDescriptorProto extension = 6;

    pub fn clear_extension(&mut self) {
        self.extension.clear();
    }

    // Param is passed by value, moved
    pub fn set_extension(&mut self, v: ::protobuf::RepeatedField<FieldDescriptorProto>) {
        self.extension = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extension(&mut self) -> &mut ::protobuf::RepeatedField<FieldDescriptorProto> {
        &mut self.extension
    }

    // Take field
    pub fn take_extension(&mut self) -> ::protobuf::RepeatedField<FieldDescriptorProto> {
        ::std::mem::replace(&mut self.extension, ::protobuf::RepeatedField::new())
    }

    pub fn get_extension(&self) -> &[FieldDescriptorProto] {
        &self.extension
    }

    fn get_extension_for_reflect(&self) -> &::protobuf::RepeatedField<FieldDescriptorProto> {
        &self.extension
    }

    fn mut_extension_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<FieldDescriptorProto> {
        &mut self.extension
    }

    // repeated .google.protobuf.DescriptorProto nested_type = 3;

    pub fn clear_nested_type(&mut self) {
        self.nested_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_nested_type(&mut self, v: ::protobuf::RepeatedField<DescriptorProto>) {
        self.nested_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nested_type(&mut self) -> &mut ::protobuf::RepeatedField<DescriptorProto> {
        &mut self.nested_type
    }

    // Take field
    pub fn take_nested_type(&mut self) -> ::protobuf::RepeatedField<DescriptorProto> {
        ::std::mem::replace(&mut self.nested_type, ::protobuf::RepeatedField::new())
    }

    pub fn get_nested_type(&self) -> &[DescriptorProto] {
        &self.nested_type
    }

    fn get_nested_type_for_reflect(&self) -> &::protobuf::RepeatedField<DescriptorProto> {
        &self.nested_type
    }

    fn mut_nested_type_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DescriptorProto> {
        &mut self.nested_type
    }

    // repeated .google.protobuf.EnumDescriptorProto enum_type = 4;

    pub fn clear_enum_type(&mut self) {
        self.enum_type.clear();
    }

    // Param is passed by value, moved
    pub fn set_enum_type(&mut self, v: ::protobuf::RepeatedField<EnumDescriptorProto>) {
        self.enum_type = v;
    }

    // Mutable pointer to the field.
    pub fn mut_enum_type(&mut self) -> &mut ::protobuf::RepeatedField<EnumDescriptorProto> {
        &mut self.enum_type
    }

    // Take field
    pub fn take_enum_type(&mut self) -> ::protobuf::RepeatedField<EnumDescriptorProto> {
        ::std::mem::replace(&mut self.enum_type, ::protobuf::RepeatedField::new())
    }

    pub fn get_enum_type(&self) -> &[EnumDescriptorProto] {
        &self.enum_type
    }

    fn get_enum_type_for_reflect(&self) -> &::protobuf::RepeatedField<EnumDescriptorProto> {
        &self.enum_type
    }

    fn mut_enum_type_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EnumDescriptorProto> {
        &mut self.enum_type
    }

    // repeated .google.protobuf.DescriptorProto.ExtensionRange extension_range = 5;

    pub fn clear_extension_range(&mut self) {
        self.extension_range.clear();
    }

    // Param is passed by value, moved
    pub fn set_extension_range(&mut self, v: ::protobuf::RepeatedField<DescriptorProto_ExtensionRange>) {
        self.extension_range = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extension_range(&mut self) -> &mut ::protobuf::RepeatedField<DescriptorProto_ExtensionRange> {
        &mut self.extension_range
    }

    // Take field
    pub fn take_extension_range(&mut self) -> ::protobuf::RepeatedField<DescriptorProto_ExtensionRange> {
        ::std::mem::replace(&mut self.extension_range, ::protobuf::RepeatedField::new())
    }

    pub fn get_extension_range(&self) -> &[DescriptorProto_ExtensionRange] {
        &self.extension_range
    }

    fn get_extension_range_for_reflect(&self) -> &::protobuf::RepeatedField<DescriptorProto_ExtensionRange> {
        &self.extension_range
    }

    fn mut_extension_range_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DescriptorProto_ExtensionRange> {
        &mut self.extension_range
    }

    // repeated .google.protobuf.OneofDescriptorProto oneof_decl = 8;

    pub fn clear_oneof_decl(&mut self) {
        self.oneof_decl.clear();
    }

    // Param is passed by value, moved
    pub fn set_oneof_decl(&mut self, v: ::protobuf::RepeatedField<OneofDescriptorProto>) {
        self.oneof_decl = v;
    }

    // Mutable pointer to the field.
    pub fn mut_oneof_decl(&mut self) -> &mut ::protobuf::RepeatedField<OneofDescriptorProto> {
        &mut self.oneof_decl
    }

    // Take field
    pub fn take_oneof_decl(&mut self) -> ::protobuf::RepeatedField<OneofDescriptorProto> {
        ::std::mem::replace(&mut self.oneof_decl, ::protobuf::RepeatedField::new())
    }

    pub fn get_oneof_decl(&self) -> &[OneofDescriptorProto] {
        &self.oneof_decl
    }

    fn get_oneof_decl_for_reflect(&self) -> &::protobuf::RepeatedField<OneofDescriptorProto> {
        &self.oneof_decl
    }

    fn mut_oneof_decl_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<OneofDescriptorProto> {
        &mut self.oneof_decl
    }

    // optional .google.protobuf.MessageOptions options = 7;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: MessageOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut MessageOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> MessageOptions {
        self.options.take().unwrap_or_else(|| MessageOptions::new())
    }

    pub fn get_options(&self) -> &MessageOptions {
        self.options.as_ref().unwrap_or_else(|| MessageOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<MessageOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MessageOptions> {
        &mut self.options
    }

    // repeated .google.protobuf.DescriptorProto.ReservedRange reserved_range = 9;

    pub fn clear_reserved_range(&mut self) {
        self.reserved_range.clear();
    }

    // Param is passed by value, moved
    pub fn set_reserved_range(&mut self, v: ::protobuf::RepeatedField<DescriptorProto_ReservedRange>) {
        self.reserved_range = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reserved_range(&mut self) -> &mut ::protobuf::RepeatedField<DescriptorProto_ReservedRange> {
        &mut self.reserved_range
    }

    // Take field
    pub fn take_reserved_range(&mut self) -> ::protobuf::RepeatedField<DescriptorProto_ReservedRange> {
        ::std::mem::replace(&mut self.reserved_range, ::protobuf::RepeatedField::new())
    }

    pub fn get_reserved_range(&self) -> &[DescriptorProto_ReservedRange] {
        &self.reserved_range
    }

    fn get_reserved_range_for_reflect(&self) -> &::protobuf::RepeatedField<DescriptorProto_ReservedRange> {
        &self.reserved_range
    }

    fn mut_reserved_range_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DescriptorProto_ReservedRange> {
        &mut self.reserved_range
    }

    // repeated string reserved_name = 10;

    pub fn clear_reserved_name(&mut self) {
        self.reserved_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_reserved_name(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.reserved_name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reserved_name(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.reserved_name
    }

    // Take field
    pub fn take_reserved_name(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.reserved_name, ::protobuf::RepeatedField::new())
    }

    pub fn get_reserved_name(&self) -> &[::std::string::String] {
        &self.reserved_name
    }

    fn get_reserved_name_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.reserved_name
    }

    fn mut_reserved_name_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.reserved_name
    }
}

impl ::protobuf::Message for DescriptorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.field {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.extension {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.nested_type {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.enum_type {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.extension_range {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.oneof_decl {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.reserved_range {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.field)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extension)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.nested_type)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.enum_type)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.extension_range)?;
                },
                8 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.oneof_decl)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.reserved_range)?;
                },
                10 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.reserved_name)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.field {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.extension {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.nested_type {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.enum_type {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.extension_range {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.oneof_decl {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.reserved_range {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.reserved_name {
            my_size += ::protobuf::rt::string_size(10, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.field {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.extension {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.nested_type {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.enum_type {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.extension_range {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.oneof_decl {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.reserved_range {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.reserved_name {
            os.write_string(10, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DescriptorProto {
    fn new() -> DescriptorProto {
        DescriptorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DescriptorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DescriptorProto::get_name_for_reflect,
                    DescriptorProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FieldDescriptorProto>>(
                    "field",
                    DescriptorProto::get_field_for_reflect,
                    DescriptorProto::mut_field_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FieldDescriptorProto>>(
                    "extension",
                    DescriptorProto::get_extension_for_reflect,
                    DescriptorProto::mut_extension_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DescriptorProto>>(
                    "nested_type",
                    DescriptorProto::get_nested_type_for_reflect,
                    DescriptorProto::mut_nested_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EnumDescriptorProto>>(
                    "enum_type",
                    DescriptorProto::get_enum_type_for_reflect,
                    DescriptorProto::mut_enum_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DescriptorProto_ExtensionRange>>(
                    "extension_range",
                    DescriptorProto::get_extension_range_for_reflect,
                    DescriptorProto::mut_extension_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OneofDescriptorProto>>(
                    "oneof_decl",
                    DescriptorProto::get_oneof_decl_for_reflect,
                    DescriptorProto::mut_oneof_decl_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MessageOptions>>(
                    "options",
                    DescriptorProto::get_options_for_reflect,
                    DescriptorProto::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DescriptorProto_ReservedRange>>(
                    "reserved_range",
                    DescriptorProto::get_reserved_range_for_reflect,
                    DescriptorProto::mut_reserved_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reserved_name",
                    DescriptorProto::get_reserved_name_for_reflect,
                    DescriptorProto::mut_reserved_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DescriptorProto>(
                    "DescriptorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DescriptorProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field();
        self.clear_extension();
        self.clear_nested_type();
        self.clear_enum_type();
        self.clear_extension_range();
        self.clear_oneof_decl();
        self.clear_options();
        self.clear_reserved_range();
        self.clear_reserved_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DescriptorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DescriptorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DescriptorProto_ExtensionRange {
    // message fields
    start: ::std::option::Option<i32>,
    end: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DescriptorProto_ExtensionRange {}

impl DescriptorProto_ExtensionRange {
    pub fn new() -> DescriptorProto_ExtensionRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DescriptorProto_ExtensionRange {
        static mut instance: ::protobuf::lazy::Lazy<DescriptorProto_ExtensionRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DescriptorProto_ExtensionRange,
        };
        unsafe {
            instance.get(DescriptorProto_ExtensionRange::new)
        }
    }

    // optional int32 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: i32) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> i32 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.start
    }

    // optional int32 end = 2;

    pub fn clear_end(&mut self) {
        self.end = ::std::option::Option::None;
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: i32) {
        self.end = ::std::option::Option::Some(v);
    }

    pub fn get_end(&self) -> i32 {
        self.end.unwrap_or(0)
    }

    fn get_end_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.end
    }
}

impl ::protobuf::Message for DescriptorProto_ExtensionRange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.end = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.end {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.end {
            os.write_int32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DescriptorProto_ExtensionRange {
    fn new() -> DescriptorProto_ExtensionRange {
        DescriptorProto_ExtensionRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<DescriptorProto_ExtensionRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "start",
                    DescriptorProto_ExtensionRange::get_start_for_reflect,
                    DescriptorProto_ExtensionRange::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "end",
                    DescriptorProto_ExtensionRange::get_end_for_reflect,
                    DescriptorProto_ExtensionRange::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DescriptorProto_ExtensionRange>(
                    "DescriptorProto_ExtensionRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DescriptorProto_ExtensionRange {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DescriptorProto_ExtensionRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DescriptorProto_ExtensionRange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DescriptorProto_ReservedRange {
    // message fields
    start: ::std::option::Option<i32>,
    end: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DescriptorProto_ReservedRange {}

impl DescriptorProto_ReservedRange {
    pub fn new() -> DescriptorProto_ReservedRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DescriptorProto_ReservedRange {
        static mut instance: ::protobuf::lazy::Lazy<DescriptorProto_ReservedRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DescriptorProto_ReservedRange,
        };
        unsafe {
            instance.get(DescriptorProto_ReservedRange::new)
        }
    }

    // optional int32 start = 1;

    pub fn clear_start(&mut self) {
        self.start = ::std::option::Option::None;
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: i32) {
        self.start = ::std::option::Option::Some(v);
    }

    pub fn get_start(&self) -> i32 {
        self.start.unwrap_or(0)
    }

    fn get_start_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.start
    }

    // optional int32 end = 2;

    pub fn clear_end(&mut self) {
        self.end = ::std::option::Option::None;
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: i32) {
        self.end = ::std::option::Option::Some(v);
    }

    pub fn get_end(&self) -> i32 {
        self.end.unwrap_or(0)
    }

    fn get_end_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.end
    }
}

impl ::protobuf::Message for DescriptorProto_ReservedRange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.start = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.end = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.end {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.end {
            os.write_int32(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DescriptorProto_ReservedRange {
    fn new() -> DescriptorProto_ReservedRange {
        DescriptorProto_ReservedRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<DescriptorProto_ReservedRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "start",
                    DescriptorProto_ReservedRange::get_start_for_reflect,
                    DescriptorProto_ReservedRange::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "end",
                    DescriptorProto_ReservedRange::get_end_for_reflect,
                    DescriptorProto_ReservedRange::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DescriptorProto_ReservedRange>(
                    "DescriptorProto_ReservedRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DescriptorProto_ReservedRange {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DescriptorProto_ReservedRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DescriptorProto_ReservedRange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FieldDescriptorProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    number: ::std::option::Option<i32>,
    label: ::std::option::Option<FieldDescriptorProto_Label>,
    field_type: ::std::option::Option<FieldDescriptorProto_Type>,
    type_name: ::protobuf::SingularField<::std::string::String>,
    extendee: ::protobuf::SingularField<::std::string::String>,
    default_value: ::protobuf::SingularField<::std::string::String>,
    oneof_index: ::std::option::Option<i32>,
    json_name: ::protobuf::SingularField<::std::string::String>,
    options: ::protobuf::SingularPtrField<FieldOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FieldDescriptorProto {}

impl FieldDescriptorProto {
    pub fn new() -> FieldDescriptorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FieldDescriptorProto {
        static mut instance: ::protobuf::lazy::Lazy<FieldDescriptorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FieldDescriptorProto,
        };
        unsafe {
            instance.get(FieldDescriptorProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional int32 number = 3;

    pub fn clear_number(&mut self) {
        self.number = ::std::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = ::std::option::Option::Some(v);
    }

    pub fn get_number(&self) -> i32 {
        self.number.unwrap_or(0)
    }

    fn get_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.number
    }

    // optional .google.protobuf.FieldDescriptorProto.Label label = 4;

    pub fn clear_label(&mut self) {
        self.label = ::std::option::Option::None;
    }

    pub fn has_label(&self) -> bool {
        self.label.is_some()
    }

    // Param is passed by value, moved
    pub fn set_label(&mut self, v: FieldDescriptorProto_Label) {
        self.label = ::std::option::Option::Some(v);
    }

    pub fn get_label(&self) -> FieldDescriptorProto_Label {
        self.label.unwrap_or(FieldDescriptorProto_Label::LABEL_OPTIONAL)
    }

    fn get_label_for_reflect(&self) -> &::std::option::Option<FieldDescriptorProto_Label> {
        &self.label
    }

    fn mut_label_for_reflect(&mut self) -> &mut ::std::option::Option<FieldDescriptorProto_Label> {
        &mut self.label
    }

    // optional .google.protobuf.FieldDescriptorProto.Type type = 5;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: FieldDescriptorProto_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> FieldDescriptorProto_Type {
        self.field_type.unwrap_or(FieldDescriptorProto_Type::TYPE_DOUBLE)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<FieldDescriptorProto_Type> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<FieldDescriptorProto_Type> {
        &mut self.field_type
    }

    // optional string type_name = 6;

    pub fn clear_type_name(&mut self) {
        self.type_name.clear();
    }

    pub fn has_type_name(&self) -> bool {
        self.type_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_type_name(&mut self, v: ::std::string::String) {
        self.type_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_type_name(&mut self) -> &mut ::std::string::String {
        if self.type_name.is_none() {
            self.type_name.set_default();
        }
        self.type_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_type_name(&mut self) -> ::std::string::String {
        self.type_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_type_name(&self) -> &str {
        match self.type_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_type_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.type_name
    }

    fn mut_type_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.type_name
    }

    // optional string extendee = 2;

    pub fn clear_extendee(&mut self) {
        self.extendee.clear();
    }

    pub fn has_extendee(&self) -> bool {
        self.extendee.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extendee(&mut self, v: ::std::string::String) {
        self.extendee = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_extendee(&mut self) -> &mut ::std::string::String {
        if self.extendee.is_none() {
            self.extendee.set_default();
        }
        self.extendee.as_mut().unwrap()
    }

    // Take field
    pub fn take_extendee(&mut self) -> ::std::string::String {
        self.extendee.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_extendee(&self) -> &str {
        match self.extendee.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_extendee_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.extendee
    }

    fn mut_extendee_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.extendee
    }

    // optional string default_value = 7;

    pub fn clear_default_value(&mut self) {
        self.default_value.clear();
    }

    pub fn has_default_value(&self) -> bool {
        self.default_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_default_value(&mut self, v: ::std::string::String) {
        self.default_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_default_value(&mut self) -> &mut ::std::string::String {
        if self.default_value.is_none() {
            self.default_value.set_default();
        }
        self.default_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_default_value(&mut self) -> ::std::string::String {
        self.default_value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_default_value(&self) -> &str {
        match self.default_value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_default_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.default_value
    }

    fn mut_default_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.default_value
    }

    // optional int32 oneof_index = 9;

    pub fn clear_oneof_index(&mut self) {
        self.oneof_index = ::std::option::Option::None;
    }

    pub fn has_oneof_index(&self) -> bool {
        self.oneof_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oneof_index(&mut self, v: i32) {
        self.oneof_index = ::std::option::Option::Some(v);
    }

    pub fn get_oneof_index(&self) -> i32 {
        self.oneof_index.unwrap_or(0)
    }

    fn get_oneof_index_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.oneof_index
    }

    fn mut_oneof_index_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.oneof_index
    }

    // optional string json_name = 10;

    pub fn clear_json_name(&mut self) {
        self.json_name.clear();
    }

    pub fn has_json_name(&self) -> bool {
        self.json_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_json_name(&mut self, v: ::std::string::String) {
        self.json_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_json_name(&mut self) -> &mut ::std::string::String {
        if self.json_name.is_none() {
            self.json_name.set_default();
        }
        self.json_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_json_name(&mut self) -> ::std::string::String {
        self.json_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_json_name(&self) -> &str {
        match self.json_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_json_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.json_name
    }

    fn mut_json_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.json_name
    }

    // optional .google.protobuf.FieldOptions options = 8;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: FieldOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut FieldOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> FieldOptions {
        self.options.take().unwrap_or_else(|| FieldOptions::new())
    }

    pub fn get_options(&self) -> &FieldOptions {
        self.options.as_ref().unwrap_or_else(|| FieldOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<FieldOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FieldOptions> {
        &mut self.options
    }
}

impl ::protobuf::Message for FieldDescriptorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.number = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.label = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.type_name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.extendee)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.default_value)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.oneof_index = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.json_name)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.number {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.label {
            my_size += ::protobuf::rt::enum_size(4, v);
        }
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(5, v);
        }
        if let Some(ref v) = self.type_name.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.extendee.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.default_value.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        if let Some(v) = self.oneof_index {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.json_name.as_ref() {
            my_size += ::protobuf::rt::string_size(10, &v);
        }
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.number {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.label {
            os.write_enum(4, v.value())?;
        }
        if let Some(v) = self.field_type {
            os.write_enum(5, v.value())?;
        }
        if let Some(ref v) = self.type_name.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.extendee.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.default_value.as_ref() {
            os.write_string(7, &v)?;
        }
        if let Some(v) = self.oneof_index {
            os.write_int32(9, v)?;
        }
        if let Some(ref v) = self.json_name.as_ref() {
            os.write_string(10, &v)?;
        }
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FieldDescriptorProto {
    fn new() -> FieldDescriptorProto {
        FieldDescriptorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FieldDescriptorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    FieldDescriptorProto::get_name_for_reflect,
                    FieldDescriptorProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "number",
                    FieldDescriptorProto::get_number_for_reflect,
                    FieldDescriptorProto::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FieldDescriptorProto_Label>>(
                    "label",
                    FieldDescriptorProto::get_label_for_reflect,
                    FieldDescriptorProto::mut_label_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FieldDescriptorProto_Type>>(
                    "type",
                    FieldDescriptorProto::get_field_type_for_reflect,
                    FieldDescriptorProto::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type_name",
                    FieldDescriptorProto::get_type_name_for_reflect,
                    FieldDescriptorProto::mut_type_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "extendee",
                    FieldDescriptorProto::get_extendee_for_reflect,
                    FieldDescriptorProto::mut_extendee_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "default_value",
                    FieldDescriptorProto::get_default_value_for_reflect,
                    FieldDescriptorProto::mut_default_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "oneof_index",
                    FieldDescriptorProto::get_oneof_index_for_reflect,
                    FieldDescriptorProto::mut_oneof_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "json_name",
                    FieldDescriptorProto::get_json_name_for_reflect,
                    FieldDescriptorProto::mut_json_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FieldOptions>>(
                    "options",
                    FieldDescriptorProto::get_options_for_reflect,
                    FieldDescriptorProto::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FieldDescriptorProto>(
                    "FieldDescriptorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FieldDescriptorProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_number();
        self.clear_label();
        self.clear_field_type();
        self.clear_type_name();
        self.clear_extendee();
        self.clear_default_value();
        self.clear_oneof_index();
        self.clear_json_name();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FieldDescriptorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FieldDescriptorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FieldDescriptorProto_Type {
    TYPE_DOUBLE = 1,
    TYPE_FLOAT = 2,
    TYPE_INT64 = 3,
    TYPE_UINT64 = 4,
    TYPE_INT32 = 5,
    TYPE_FIXED64 = 6,
    TYPE_FIXED32 = 7,
    TYPE_BOOL = 8,
    TYPE_STRING = 9,
    TYPE_GROUP = 10,
    TYPE_MESSAGE = 11,
    TYPE_BYTES = 12,
    TYPE_UINT32 = 13,
    TYPE_ENUM = 14,
    TYPE_SFIXED32 = 15,
    TYPE_SFIXED64 = 16,
    TYPE_SINT32 = 17,
    TYPE_SINT64 = 18,
}

impl ::protobuf::ProtobufEnum for FieldDescriptorProto_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FieldDescriptorProto_Type> {
        match value {
            1 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_DOUBLE),
            2 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_FLOAT),
            3 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_INT64),
            4 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_UINT64),
            5 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_INT32),
            6 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_FIXED64),
            7 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_FIXED32),
            8 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_BOOL),
            9 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_STRING),
            10 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_GROUP),
            11 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_MESSAGE),
            12 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_BYTES),
            13 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_UINT32),
            14 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_ENUM),
            15 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_SFIXED32),
            16 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_SFIXED64),
            17 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_SINT32),
            18 => ::std::option::Option::Some(FieldDescriptorProto_Type::TYPE_SINT64),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FieldDescriptorProto_Type] = &[
            FieldDescriptorProto_Type::TYPE_DOUBLE,
            FieldDescriptorProto_Type::TYPE_FLOAT,
            FieldDescriptorProto_Type::TYPE_INT64,
            FieldDescriptorProto_Type::TYPE_UINT64,
            FieldDescriptorProto_Type::TYPE_INT32,
            FieldDescriptorProto_Type::TYPE_FIXED64,
            FieldDescriptorProto_Type::TYPE_FIXED32,
            FieldDescriptorProto_Type::TYPE_BOOL,
            FieldDescriptorProto_Type::TYPE_STRING,
            FieldDescriptorProto_Type::TYPE_GROUP,
            FieldDescriptorProto_Type::TYPE_MESSAGE,
            FieldDescriptorProto_Type::TYPE_BYTES,
            FieldDescriptorProto_Type::TYPE_UINT32,
            FieldDescriptorProto_Type::TYPE_ENUM,
            FieldDescriptorProto_Type::TYPE_SFIXED32,
            FieldDescriptorProto_Type::TYPE_SFIXED64,
            FieldDescriptorProto_Type::TYPE_SINT32,
            FieldDescriptorProto_Type::TYPE_SINT64,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<FieldDescriptorProto_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FieldDescriptorProto_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FieldDescriptorProto_Type {
}

impl ::protobuf::reflect::ProtobufValue for FieldDescriptorProto_Type {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FieldDescriptorProto_Label {
    LABEL_OPTIONAL = 1,
    LABEL_REQUIRED = 2,
    LABEL_REPEATED = 3,
}

impl ::protobuf::ProtobufEnum for FieldDescriptorProto_Label {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FieldDescriptorProto_Label> {
        match value {
            1 => ::std::option::Option::Some(FieldDescriptorProto_Label::LABEL_OPTIONAL),
            2 => ::std::option::Option::Some(FieldDescriptorProto_Label::LABEL_REQUIRED),
            3 => ::std::option::Option::Some(FieldDescriptorProto_Label::LABEL_REPEATED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FieldDescriptorProto_Label] = &[
            FieldDescriptorProto_Label::LABEL_OPTIONAL,
            FieldDescriptorProto_Label::LABEL_REQUIRED,
            FieldDescriptorProto_Label::LABEL_REPEATED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<FieldDescriptorProto_Label>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FieldDescriptorProto_Label", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FieldDescriptorProto_Label {
}

impl ::protobuf::reflect::ProtobufValue for FieldDescriptorProto_Label {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OneofDescriptorProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    options: ::protobuf::SingularPtrField<OneofOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OneofDescriptorProto {}

impl OneofDescriptorProto {
    pub fn new() -> OneofDescriptorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OneofDescriptorProto {
        static mut instance: ::protobuf::lazy::Lazy<OneofDescriptorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OneofDescriptorProto,
        };
        unsafe {
            instance.get(OneofDescriptorProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional .google.protobuf.OneofOptions options = 2;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: OneofOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut OneofOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> OneofOptions {
        self.options.take().unwrap_or_else(|| OneofOptions::new())
    }

    pub fn get_options(&self) -> &OneofOptions {
        self.options.as_ref().unwrap_or_else(|| OneofOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<OneofOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OneofOptions> {
        &mut self.options
    }
}

impl ::protobuf::Message for OneofDescriptorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OneofDescriptorProto {
    fn new() -> OneofDescriptorProto {
        OneofDescriptorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OneofDescriptorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    OneofDescriptorProto::get_name_for_reflect,
                    OneofDescriptorProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OneofOptions>>(
                    "options",
                    OneofDescriptorProto::get_options_for_reflect,
                    OneofDescriptorProto::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OneofDescriptorProto>(
                    "OneofDescriptorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OneofDescriptorProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OneofDescriptorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OneofDescriptorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnumDescriptorProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::RepeatedField<EnumValueDescriptorProto>,
    options: ::protobuf::SingularPtrField<EnumOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnumDescriptorProto {}

impl EnumDescriptorProto {
    pub fn new() -> EnumDescriptorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnumDescriptorProto {
        static mut instance: ::protobuf::lazy::Lazy<EnumDescriptorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnumDescriptorProto,
        };
        unsafe {
            instance.get(EnumDescriptorProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated .google.protobuf.EnumValueDescriptorProto value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::protobuf::RepeatedField<EnumValueDescriptorProto>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value(&mut self) -> &mut ::protobuf::RepeatedField<EnumValueDescriptorProto> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::protobuf::RepeatedField<EnumValueDescriptorProto> {
        ::std::mem::replace(&mut self.value, ::protobuf::RepeatedField::new())
    }

    pub fn get_value(&self) -> &[EnumValueDescriptorProto] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::protobuf::RepeatedField<EnumValueDescriptorProto> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<EnumValueDescriptorProto> {
        &mut self.value
    }

    // optional .google.protobuf.EnumOptions options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: EnumOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut EnumOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> EnumOptions {
        self.options.take().unwrap_or_else(|| EnumOptions::new())
    }

    pub fn get_options(&self) -> &EnumOptions {
        self.options.as_ref().unwrap_or_else(|| EnumOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<EnumOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EnumOptions> {
        &mut self.options
    }
}

impl ::protobuf::Message for EnumDescriptorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.value {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.value {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.value {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EnumDescriptorProto {
    fn new() -> EnumDescriptorProto {
        EnumDescriptorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnumDescriptorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    EnumDescriptorProto::get_name_for_reflect,
                    EnumDescriptorProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EnumValueDescriptorProto>>(
                    "value",
                    EnumDescriptorProto::get_value_for_reflect,
                    EnumDescriptorProto::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EnumOptions>>(
                    "options",
                    EnumDescriptorProto::get_options_for_reflect,
                    EnumDescriptorProto::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnumDescriptorProto>(
                    "EnumDescriptorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnumDescriptorProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnumDescriptorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnumDescriptorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnumValueDescriptorProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    number: ::std::option::Option<i32>,
    options: ::protobuf::SingularPtrField<EnumValueOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnumValueDescriptorProto {}

impl EnumValueDescriptorProto {
    pub fn new() -> EnumValueDescriptorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnumValueDescriptorProto {
        static mut instance: ::protobuf::lazy::Lazy<EnumValueDescriptorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnumValueDescriptorProto,
        };
        unsafe {
            instance.get(EnumValueDescriptorProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional int32 number = 2;

    pub fn clear_number(&mut self) {
        self.number = ::std::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: i32) {
        self.number = ::std::option::Option::Some(v);
    }

    pub fn get_number(&self) -> i32 {
        self.number.unwrap_or(0)
    }

    fn get_number_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.number
    }

    // optional .google.protobuf.EnumValueOptions options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: EnumValueOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut EnumValueOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> EnumValueOptions {
        self.options.take().unwrap_or_else(|| EnumValueOptions::new())
    }

    pub fn get_options(&self) -> &EnumValueOptions {
        self.options.as_ref().unwrap_or_else(|| EnumValueOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<EnumValueOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EnumValueOptions> {
        &mut self.options
    }
}

impl ::protobuf::Message for EnumValueDescriptorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.number = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.number {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.number {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EnumValueDescriptorProto {
    fn new() -> EnumValueDescriptorProto {
        EnumValueDescriptorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnumValueDescriptorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    EnumValueDescriptorProto::get_name_for_reflect,
                    EnumValueDescriptorProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "number",
                    EnumValueDescriptorProto::get_number_for_reflect,
                    EnumValueDescriptorProto::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EnumValueOptions>>(
                    "options",
                    EnumValueDescriptorProto::get_options_for_reflect,
                    EnumValueDescriptorProto::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnumValueDescriptorProto>(
                    "EnumValueDescriptorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnumValueDescriptorProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_number();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnumValueDescriptorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnumValueDescriptorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceDescriptorProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    method: ::protobuf::RepeatedField<MethodDescriptorProto>,
    options: ::protobuf::SingularPtrField<ServiceOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceDescriptorProto {}

impl ServiceDescriptorProto {
    pub fn new() -> ServiceDescriptorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceDescriptorProto {
        static mut instance: ::protobuf::lazy::Lazy<ServiceDescriptorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceDescriptorProto,
        };
        unsafe {
            instance.get(ServiceDescriptorProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // repeated .google.protobuf.MethodDescriptorProto method = 2;

    pub fn clear_method(&mut self) {
        self.method.clear();
    }

    // Param is passed by value, moved
    pub fn set_method(&mut self, v: ::protobuf::RepeatedField<MethodDescriptorProto>) {
        self.method = v;
    }

    // Mutable pointer to the field.
    pub fn mut_method(&mut self) -> &mut ::protobuf::RepeatedField<MethodDescriptorProto> {
        &mut self.method
    }

    // Take field
    pub fn take_method(&mut self) -> ::protobuf::RepeatedField<MethodDescriptorProto> {
        ::std::mem::replace(&mut self.method, ::protobuf::RepeatedField::new())
    }

    pub fn get_method(&self) -> &[MethodDescriptorProto] {
        &self.method
    }

    fn get_method_for_reflect(&self) -> &::protobuf::RepeatedField<MethodDescriptorProto> {
        &self.method
    }

    fn mut_method_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MethodDescriptorProto> {
        &mut self.method
    }

    // optional .google.protobuf.ServiceOptions options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ServiceOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut ServiceOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> ServiceOptions {
        self.options.take().unwrap_or_else(|| ServiceOptions::new())
    }

    pub fn get_options(&self) -> &ServiceOptions {
        self.options.as_ref().unwrap_or_else(|| ServiceOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<ServiceOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ServiceOptions> {
        &mut self.options
    }
}

impl ::protobuf::Message for ServiceDescriptorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.method {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.method)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.method {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.method {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServiceDescriptorProto {
    fn new() -> ServiceDescriptorProto {
        ServiceDescriptorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceDescriptorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ServiceDescriptorProto::get_name_for_reflect,
                    ServiceDescriptorProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MethodDescriptorProto>>(
                    "method",
                    ServiceDescriptorProto::get_method_for_reflect,
                    ServiceDescriptorProto::mut_method_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ServiceOptions>>(
                    "options",
                    ServiceDescriptorProto::get_options_for_reflect,
                    ServiceDescriptorProto::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceDescriptorProto>(
                    "ServiceDescriptorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceDescriptorProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_method();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceDescriptorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceDescriptorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MethodDescriptorProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    input_type: ::protobuf::SingularField<::std::string::String>,
    output_type: ::protobuf::SingularField<::std::string::String>,
    options: ::protobuf::SingularPtrField<MethodOptions>,
    client_streaming: ::std::option::Option<bool>,
    server_streaming: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MethodDescriptorProto {}

impl MethodDescriptorProto {
    pub fn new() -> MethodDescriptorProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MethodDescriptorProto {
        static mut instance: ::protobuf::lazy::Lazy<MethodDescriptorProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MethodDescriptorProto,
        };
        unsafe {
            instance.get(MethodDescriptorProto::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string input_type = 2;

    pub fn clear_input_type(&mut self) {
        self.input_type.clear();
    }

    pub fn has_input_type(&self) -> bool {
        self.input_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_type(&mut self, v: ::std::string::String) {
        self.input_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input_type(&mut self) -> &mut ::std::string::String {
        if self.input_type.is_none() {
            self.input_type.set_default();
        }
        self.input_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_input_type(&mut self) -> ::std::string::String {
        self.input_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_input_type(&self) -> &str {
        match self.input_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_input_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.input_type
    }

    fn mut_input_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.input_type
    }

    // optional string output_type = 3;

    pub fn clear_output_type(&mut self) {
        self.output_type.clear();
    }

    pub fn has_output_type(&self) -> bool {
        self.output_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_output_type(&mut self, v: ::std::string::String) {
        self.output_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_output_type(&mut self) -> &mut ::std::string::String {
        if self.output_type.is_none() {
            self.output_type.set_default();
        }
        self.output_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_output_type(&mut self) -> ::std::string::String {
        self.output_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_output_type(&self) -> &str {
        match self.output_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_output_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.output_type
    }

    fn mut_output_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.output_type
    }

    // optional .google.protobuf.MethodOptions options = 4;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: MethodOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut MethodOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> MethodOptions {
        self.options.take().unwrap_or_else(|| MethodOptions::new())
    }

    pub fn get_options(&self) -> &MethodOptions {
        self.options.as_ref().unwrap_or_else(|| MethodOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<MethodOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MethodOptions> {
        &mut self.options
    }

    // optional bool client_streaming = 5;

    pub fn clear_client_streaming(&mut self) {
        self.client_streaming = ::std::option::Option::None;
    }

    pub fn has_client_streaming(&self) -> bool {
        self.client_streaming.is_some()
    }

    // Param is passed by value, moved
    pub fn set_client_streaming(&mut self, v: bool) {
        self.client_streaming = ::std::option::Option::Some(v);
    }

    pub fn get_client_streaming(&self) -> bool {
        self.client_streaming.unwrap_or(false)
    }

    fn get_client_streaming_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.client_streaming
    }

    fn mut_client_streaming_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.client_streaming
    }

    // optional bool server_streaming = 6;

    pub fn clear_server_streaming(&mut self) {
        self.server_streaming = ::std::option::Option::None;
    }

    pub fn has_server_streaming(&self) -> bool {
        self.server_streaming.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_streaming(&mut self, v: bool) {
        self.server_streaming = ::std::option::Option::Some(v);
    }

    pub fn get_server_streaming(&self) -> bool {
        self.server_streaming.unwrap_or(false)
    }

    fn get_server_streaming_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.server_streaming
    }

    fn mut_server_streaming_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.server_streaming
    }
}

impl ::protobuf::Message for MethodDescriptorProto {
    fn is_initialized(&self) -> bool {
        for v in &self.options {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.input_type)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.output_type)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.client_streaming = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.server_streaming = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.input_type.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.output_type.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.client_streaming {
            my_size += 2;
        }
        if let Some(v) = self.server_streaming {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.input_type.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.output_type.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.client_streaming {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.server_streaming {
            os.write_bool(6, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MethodDescriptorProto {
    fn new() -> MethodDescriptorProto {
        MethodDescriptorProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<MethodDescriptorProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    MethodDescriptorProto::get_name_for_reflect,
                    MethodDescriptorProto::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "input_type",
                    MethodDescriptorProto::get_input_type_for_reflect,
                    MethodDescriptorProto::mut_input_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "output_type",
                    MethodDescriptorProto::get_output_type_for_reflect,
                    MethodDescriptorProto::mut_output_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MethodOptions>>(
                    "options",
                    MethodDescriptorProto::get_options_for_reflect,
                    MethodDescriptorProto::mut_options_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "client_streaming",
                    MethodDescriptorProto::get_client_streaming_for_reflect,
                    MethodDescriptorProto::mut_client_streaming_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "server_streaming",
                    MethodDescriptorProto::get_server_streaming_for_reflect,
                    MethodDescriptorProto::mut_server_streaming_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MethodDescriptorProto>(
                    "MethodDescriptorProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MethodDescriptorProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_input_type();
        self.clear_output_type();
        self.clear_options();
        self.clear_client_streaming();
        self.clear_server_streaming();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MethodDescriptorProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MethodDescriptorProto {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FileOptions {
    // message fields
    java_package: ::protobuf::SingularField<::std::string::String>,
    java_outer_classname: ::protobuf::SingularField<::std::string::String>,
    java_multiple_files: ::std::option::Option<bool>,
    java_generate_equals_and_hash: ::std::option::Option<bool>,
    java_string_check_utf8: ::std::option::Option<bool>,
    optimize_for: ::std::option::Option<FileOptions_OptimizeMode>,
    go_package: ::protobuf::SingularField<::std::string::String>,
    cc_generic_services: ::std::option::Option<bool>,
    java_generic_services: ::std::option::Option<bool>,
    py_generic_services: ::std::option::Option<bool>,
    deprecated: ::std::option::Option<bool>,
    cc_enable_arenas: ::std::option::Option<bool>,
    objc_class_prefix: ::protobuf::SingularField<::std::string::String>,
    csharp_namespace: ::protobuf::SingularField<::std::string::String>,
    uninterpreted_option: ::protobuf::RepeatedField<UninterpretedOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FileOptions {}

impl FileOptions {
    pub fn new() -> FileOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileOptions {
        static mut instance: ::protobuf::lazy::Lazy<FileOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileOptions,
        };
        unsafe {
            instance.get(FileOptions::new)
        }
    }

    // optional string java_package = 1;

    pub fn clear_java_package(&mut self) {
        self.java_package.clear();
    }

    pub fn has_java_package(&self) -> bool {
        self.java_package.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_package(&mut self, v: ::std::string::String) {
        self.java_package = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_java_package(&mut self) -> &mut ::std::string::String {
        if self.java_package.is_none() {
            self.java_package.set_default();
        }
        self.java_package.as_mut().unwrap()
    }

    // Take field
    pub fn take_java_package(&mut self) -> ::std::string::String {
        self.java_package.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_java_package(&self) -> &str {
        match self.java_package.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_java_package_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.java_package
    }

    fn mut_java_package_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.java_package
    }

    // optional string java_outer_classname = 8;

    pub fn clear_java_outer_classname(&mut self) {
        self.java_outer_classname.clear();
    }

    pub fn has_java_outer_classname(&self) -> bool {
        self.java_outer_classname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_outer_classname(&mut self, v: ::std::string::String) {
        self.java_outer_classname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_java_outer_classname(&mut self) -> &mut ::std::string::String {
        if self.java_outer_classname.is_none() {
            self.java_outer_classname.set_default();
        }
        self.java_outer_classname.as_mut().unwrap()
    }

    // Take field
    pub fn take_java_outer_classname(&mut self) -> ::std::string::String {
        self.java_outer_classname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_java_outer_classname(&self) -> &str {
        match self.java_outer_classname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_java_outer_classname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.java_outer_classname
    }

    fn mut_java_outer_classname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.java_outer_classname
    }

    // optional bool java_multiple_files = 10;

    pub fn clear_java_multiple_files(&mut self) {
        self.java_multiple_files = ::std::option::Option::None;
    }

    pub fn has_java_multiple_files(&self) -> bool {
        self.java_multiple_files.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_multiple_files(&mut self, v: bool) {
        self.java_multiple_files = ::std::option::Option::Some(v);
    }

    pub fn get_java_multiple_files(&self) -> bool {
        self.java_multiple_files.unwrap_or(false)
    }

    fn get_java_multiple_files_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.java_multiple_files
    }

    fn mut_java_multiple_files_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.java_multiple_files
    }

    // optional bool java_generate_equals_and_hash = 20;

    pub fn clear_java_generate_equals_and_hash(&mut self) {
        self.java_generate_equals_and_hash = ::std::option::Option::None;
    }

    pub fn has_java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_generate_equals_and_hash(&mut self, v: bool) {
        self.java_generate_equals_and_hash = ::std::option::Option::Some(v);
    }

    pub fn get_java_generate_equals_and_hash(&self) -> bool {
        self.java_generate_equals_and_hash.unwrap_or(false)
    }

    fn get_java_generate_equals_and_hash_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.java_generate_equals_and_hash
    }

    fn mut_java_generate_equals_and_hash_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.java_generate_equals_and_hash
    }

    // optional bool java_string_check_utf8 = 27;

    pub fn clear_java_string_check_utf8(&mut self) {
        self.java_string_check_utf8 = ::std::option::Option::None;
    }

    pub fn has_java_string_check_utf8(&self) -> bool {
        self.java_string_check_utf8.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_string_check_utf8(&mut self, v: bool) {
        self.java_string_check_utf8 = ::std::option::Option::Some(v);
    }

    pub fn get_java_string_check_utf8(&self) -> bool {
        self.java_string_check_utf8.unwrap_or(false)
    }

    fn get_java_string_check_utf8_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.java_string_check_utf8
    }

    fn mut_java_string_check_utf8_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.java_string_check_utf8
    }

    // optional .google.protobuf.FileOptions.OptimizeMode optimize_for = 9;

    pub fn clear_optimize_for(&mut self) {
        self.optimize_for = ::std::option::Option::None;
    }

    pub fn has_optimize_for(&self) -> bool {
        self.optimize_for.is_some()
    }

    // Param is passed by value, moved
    pub fn set_optimize_for(&mut self, v: FileOptions_OptimizeMode) {
        self.optimize_for = ::std::option::Option::Some(v);
    }

    pub fn get_optimize_for(&self) -> FileOptions_OptimizeMode {
        self.optimize_for.unwrap_or(FileOptions_OptimizeMode::SPEED)
    }

    fn get_optimize_for_for_reflect(&self) -> &::std::option::Option<FileOptions_OptimizeMode> {
        &self.optimize_for
    }

    fn mut_optimize_for_for_reflect(&mut self) -> &mut ::std::option::Option<FileOptions_OptimizeMode> {
        &mut self.optimize_for
    }

    // optional string go_package = 11;

    pub fn clear_go_package(&mut self) {
        self.go_package.clear();
    }

    pub fn has_go_package(&self) -> bool {
        self.go_package.is_some()
    }

    // Param is passed by value, moved
    pub fn set_go_package(&mut self, v: ::std::string::String) {
        self.go_package = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_go_package(&mut self) -> &mut ::std::string::String {
        if self.go_package.is_none() {
            self.go_package.set_default();
        }
        self.go_package.as_mut().unwrap()
    }

    // Take field
    pub fn take_go_package(&mut self) -> ::std::string::String {
        self.go_package.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_go_package(&self) -> &str {
        match self.go_package.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_go_package_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.go_package
    }

    fn mut_go_package_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.go_package
    }

    // optional bool cc_generic_services = 16;

    pub fn clear_cc_generic_services(&mut self) {
        self.cc_generic_services = ::std::option::Option::None;
    }

    pub fn has_cc_generic_services(&self) -> bool {
        self.cc_generic_services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cc_generic_services(&mut self, v: bool) {
        self.cc_generic_services = ::std::option::Option::Some(v);
    }

    pub fn get_cc_generic_services(&self) -> bool {
        self.cc_generic_services.unwrap_or(false)
    }

    fn get_cc_generic_services_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.cc_generic_services
    }

    fn mut_cc_generic_services_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.cc_generic_services
    }

    // optional bool java_generic_services = 17;

    pub fn clear_java_generic_services(&mut self) {
        self.java_generic_services = ::std::option::Option::None;
    }

    pub fn has_java_generic_services(&self) -> bool {
        self.java_generic_services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_java_generic_services(&mut self, v: bool) {
        self.java_generic_services = ::std::option::Option::Some(v);
    }

    pub fn get_java_generic_services(&self) -> bool {
        self.java_generic_services.unwrap_or(false)
    }

    fn get_java_generic_services_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.java_generic_services
    }

    fn mut_java_generic_services_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.java_generic_services
    }

    // optional bool py_generic_services = 18;

    pub fn clear_py_generic_services(&mut self) {
        self.py_generic_services = ::std::option::Option::None;
    }

    pub fn has_py_generic_services(&self) -> bool {
        self.py_generic_services.is_some()
    }

    // Param is passed by value, moved
    pub fn set_py_generic_services(&mut self, v: bool) {
        self.py_generic_services = ::std::option::Option::Some(v);
    }

    pub fn get_py_generic_services(&self) -> bool {
        self.py_generic_services.unwrap_or(false)
    }

    fn get_py_generic_services_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.py_generic_services
    }

    fn mut_py_generic_services_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.py_generic_services
    }

    // optional bool deprecated = 23;

    pub fn clear_deprecated(&mut self) {
        self.deprecated = ::std::option::Option::None;
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: bool) {
        self.deprecated = ::std::option::Option::Some(v);
    }

    pub fn get_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    fn get_deprecated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deprecated
    }

    // optional bool cc_enable_arenas = 31;

    pub fn clear_cc_enable_arenas(&mut self) {
        self.cc_enable_arenas = ::std::option::Option::None;
    }

    pub fn has_cc_enable_arenas(&self) -> bool {
        self.cc_enable_arenas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cc_enable_arenas(&mut self, v: bool) {
        self.cc_enable_arenas = ::std::option::Option::Some(v);
    }

    pub fn get_cc_enable_arenas(&self) -> bool {
        self.cc_enable_arenas.unwrap_or(false)
    }

    fn get_cc_enable_arenas_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.cc_enable_arenas
    }

    fn mut_cc_enable_arenas_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.cc_enable_arenas
    }

    // optional string objc_class_prefix = 36;

    pub fn clear_objc_class_prefix(&mut self) {
        self.objc_class_prefix.clear();
    }

    pub fn has_objc_class_prefix(&self) -> bool {
        self.objc_class_prefix.is_some()
    }

    // Param is passed by value, moved
    pub fn set_objc_class_prefix(&mut self, v: ::std::string::String) {
        self.objc_class_prefix = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_objc_class_prefix(&mut self) -> &mut ::std::string::String {
        if self.objc_class_prefix.is_none() {
            self.objc_class_prefix.set_default();
        }
        self.objc_class_prefix.as_mut().unwrap()
    }

    // Take field
    pub fn take_objc_class_prefix(&mut self) -> ::std::string::String {
        self.objc_class_prefix.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_objc_class_prefix(&self) -> &str {
        match self.objc_class_prefix.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_objc_class_prefix_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.objc_class_prefix
    }

    fn mut_objc_class_prefix_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.objc_class_prefix
    }

    // optional string csharp_namespace = 37;

    pub fn clear_csharp_namespace(&mut self) {
        self.csharp_namespace.clear();
    }

    pub fn has_csharp_namespace(&self) -> bool {
        self.csharp_namespace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_csharp_namespace(&mut self, v: ::std::string::String) {
        self.csharp_namespace = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_csharp_namespace(&mut self) -> &mut ::std::string::String {
        if self.csharp_namespace.is_none() {
            self.csharp_namespace.set_default();
        }
        self.csharp_namespace.as_mut().unwrap()
    }

    // Take field
    pub fn take_csharp_namespace(&mut self) -> ::std::string::String {
        self.csharp_namespace.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_csharp_namespace(&self) -> &str {
        match self.csharp_namespace.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_csharp_namespace_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.csharp_namespace
    }

    fn mut_csharp_namespace_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.csharp_namespace
    }

    // repeated .google.protobuf.UninterpretedOption uninterpreted_option = 999;

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption>) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }

    // Take field
    pub fn take_uninterpreted_option(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption> {
        ::std::mem::replace(&mut self.uninterpreted_option, ::protobuf::RepeatedField::new())
    }

    pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_option
    }

    fn get_uninterpreted_option_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption> {
        &self.uninterpreted_option
    }

    fn mut_uninterpreted_option_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }
}

impl ::protobuf::Message for FileOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.uninterpreted_option {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.java_package)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.java_outer_classname)?;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.java_multiple_files = ::std::option::Option::Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.java_generate_equals_and_hash = ::std::option::Option::Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.java_string_check_utf8 = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.optimize_for = ::std::option::Option::Some(tmp);
                },
                11 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.go_package)?;
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.cc_generic_services = ::std::option::Option::Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.java_generic_services = ::std::option::Option::Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.py_generic_services = ::std::option::Option::Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deprecated = ::std::option::Option::Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.cc_enable_arenas = ::std::option::Option::Some(tmp);
                },
                36 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.objc_class_prefix)?;
                },
                37 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.csharp_namespace)?;
                },
                999 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uninterpreted_option)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.java_package.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.java_outer_classname.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        if let Some(v) = self.java_multiple_files {
            my_size += 2;
        }
        if let Some(v) = self.java_generate_equals_and_hash {
            my_size += 3;
        }
        if let Some(v) = self.java_string_check_utf8 {
            my_size += 3;
        }
        if let Some(v) = self.optimize_for {
            my_size += ::protobuf::rt::enum_size(9, v);
        }
        if let Some(ref v) = self.go_package.as_ref() {
            my_size += ::protobuf::rt::string_size(11, &v);
        }
        if let Some(v) = self.cc_generic_services {
            my_size += 3;
        }
        if let Some(v) = self.java_generic_services {
            my_size += 3;
        }
        if let Some(v) = self.py_generic_services {
            my_size += 3;
        }
        if let Some(v) = self.deprecated {
            my_size += 3;
        }
        if let Some(v) = self.cc_enable_arenas {
            my_size += 3;
        }
        if let Some(ref v) = self.objc_class_prefix.as_ref() {
            my_size += ::protobuf::rt::string_size(36, &v);
        }
        if let Some(ref v) = self.csharp_namespace.as_ref() {
            my_size += ::protobuf::rt::string_size(37, &v);
        }
        for value in &self.uninterpreted_option {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.java_package.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.java_outer_classname.as_ref() {
            os.write_string(8, &v)?;
        }
        if let Some(v) = self.java_multiple_files {
            os.write_bool(10, v)?;
        }
        if let Some(v) = self.java_generate_equals_and_hash {
            os.write_bool(20, v)?;
        }
        if let Some(v) = self.java_string_check_utf8 {
            os.write_bool(27, v)?;
        }
        if let Some(v) = self.optimize_for {
            os.write_enum(9, v.value())?;
        }
        if let Some(ref v) = self.go_package.as_ref() {
            os.write_string(11, &v)?;
        }
        if let Some(v) = self.cc_generic_services {
            os.write_bool(16, v)?;
        }
        if let Some(v) = self.java_generic_services {
            os.write_bool(17, v)?;
        }
        if let Some(v) = self.py_generic_services {
            os.write_bool(18, v)?;
        }
        if let Some(v) = self.deprecated {
            os.write_bool(23, v)?;
        }
        if let Some(v) = self.cc_enable_arenas {
            os.write_bool(31, v)?;
        }
        if let Some(ref v) = self.objc_class_prefix.as_ref() {
            os.write_string(36, &v)?;
        }
        if let Some(ref v) = self.csharp_namespace.as_ref() {
            os.write_string(37, &v)?;
        }
        for v in &self.uninterpreted_option {
            os.write_tag(999, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileOptions {
    fn new() -> FileOptions {
        FileOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "java_package",
                    FileOptions::get_java_package_for_reflect,
                    FileOptions::mut_java_package_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "java_outer_classname",
                    FileOptions::get_java_outer_classname_for_reflect,
                    FileOptions::mut_java_outer_classname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "java_multiple_files",
                    FileOptions::get_java_multiple_files_for_reflect,
                    FileOptions::mut_java_multiple_files_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "java_generate_equals_and_hash",
                    FileOptions::get_java_generate_equals_and_hash_for_reflect,
                    FileOptions::mut_java_generate_equals_and_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "java_string_check_utf8",
                    FileOptions::get_java_string_check_utf8_for_reflect,
                    FileOptions::mut_java_string_check_utf8_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FileOptions_OptimizeMode>>(
                    "optimize_for",
                    FileOptions::get_optimize_for_for_reflect,
                    FileOptions::mut_optimize_for_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "go_package",
                    FileOptions::get_go_package_for_reflect,
                    FileOptions::mut_go_package_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "cc_generic_services",
                    FileOptions::get_cc_generic_services_for_reflect,
                    FileOptions::mut_cc_generic_services_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "java_generic_services",
                    FileOptions::get_java_generic_services_for_reflect,
                    FileOptions::mut_java_generic_services_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "py_generic_services",
                    FileOptions::get_py_generic_services_for_reflect,
                    FileOptions::mut_py_generic_services_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deprecated",
                    FileOptions::get_deprecated_for_reflect,
                    FileOptions::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "cc_enable_arenas",
                    FileOptions::get_cc_enable_arenas_for_reflect,
                    FileOptions::mut_cc_enable_arenas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "objc_class_prefix",
                    FileOptions::get_objc_class_prefix_for_reflect,
                    FileOptions::mut_objc_class_prefix_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "csharp_namespace",
                    FileOptions::get_csharp_namespace_for_reflect,
                    FileOptions::mut_csharp_namespace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption>>(
                    "uninterpreted_option",
                    FileOptions::get_uninterpreted_option_for_reflect,
                    FileOptions::mut_uninterpreted_option_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileOptions>(
                    "FileOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileOptions {
    fn clear(&mut self) {
        self.clear_java_package();
        self.clear_java_outer_classname();
        self.clear_java_multiple_files();
        self.clear_java_generate_equals_and_hash();
        self.clear_java_string_check_utf8();
        self.clear_optimize_for();
        self.clear_go_package();
        self.clear_cc_generic_services();
        self.clear_java_generic_services();
        self.clear_py_generic_services();
        self.clear_deprecated();
        self.clear_cc_enable_arenas();
        self.clear_objc_class_prefix();
        self.clear_csharp_namespace();
        self.clear_uninterpreted_option();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FileOptions_OptimizeMode {
    SPEED = 1,
    CODE_SIZE = 2,
    LITE_RUNTIME = 3,
}

impl ::protobuf::ProtobufEnum for FileOptions_OptimizeMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FileOptions_OptimizeMode> {
        match value {
            1 => ::std::option::Option::Some(FileOptions_OptimizeMode::SPEED),
            2 => ::std::option::Option::Some(FileOptions_OptimizeMode::CODE_SIZE),
            3 => ::std::option::Option::Some(FileOptions_OptimizeMode::LITE_RUNTIME),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FileOptions_OptimizeMode] = &[
            FileOptions_OptimizeMode::SPEED,
            FileOptions_OptimizeMode::CODE_SIZE,
            FileOptions_OptimizeMode::LITE_RUNTIME,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<FileOptions_OptimizeMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FileOptions_OptimizeMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FileOptions_OptimizeMode {
}

impl ::protobuf::reflect::ProtobufValue for FileOptions_OptimizeMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MessageOptions {
    // message fields
    message_set_wire_format: ::std::option::Option<bool>,
    no_standard_descriptor_accessor: ::std::option::Option<bool>,
    deprecated: ::std::option::Option<bool>,
    map_entry: ::std::option::Option<bool>,
    uninterpreted_option: ::protobuf::RepeatedField<UninterpretedOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MessageOptions {}

impl MessageOptions {
    pub fn new() -> MessageOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MessageOptions {
        static mut instance: ::protobuf::lazy::Lazy<MessageOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MessageOptions,
        };
        unsafe {
            instance.get(MessageOptions::new)
        }
    }

    // optional bool message_set_wire_format = 1;

    pub fn clear_message_set_wire_format(&mut self) {
        self.message_set_wire_format = ::std::option::Option::None;
    }

    pub fn has_message_set_wire_format(&self) -> bool {
        self.message_set_wire_format.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_set_wire_format(&mut self, v: bool) {
        self.message_set_wire_format = ::std::option::Option::Some(v);
    }

    pub fn get_message_set_wire_format(&self) -> bool {
        self.message_set_wire_format.unwrap_or(false)
    }

    fn get_message_set_wire_format_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.message_set_wire_format
    }

    fn mut_message_set_wire_format_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.message_set_wire_format
    }

    // optional bool no_standard_descriptor_accessor = 2;

    pub fn clear_no_standard_descriptor_accessor(&mut self) {
        self.no_standard_descriptor_accessor = ::std::option::Option::None;
    }

    pub fn has_no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_no_standard_descriptor_accessor(&mut self, v: bool) {
        self.no_standard_descriptor_accessor = ::std::option::Option::Some(v);
    }

    pub fn get_no_standard_descriptor_accessor(&self) -> bool {
        self.no_standard_descriptor_accessor.unwrap_or(false)
    }

    fn get_no_standard_descriptor_accessor_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.no_standard_descriptor_accessor
    }

    fn mut_no_standard_descriptor_accessor_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.no_standard_descriptor_accessor
    }

    // optional bool deprecated = 3;

    pub fn clear_deprecated(&mut self) {
        self.deprecated = ::std::option::Option::None;
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: bool) {
        self.deprecated = ::std::option::Option::Some(v);
    }

    pub fn get_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    fn get_deprecated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deprecated
    }

    // optional bool map_entry = 7;

    pub fn clear_map_entry(&mut self) {
        self.map_entry = ::std::option::Option::None;
    }

    pub fn has_map_entry(&self) -> bool {
        self.map_entry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_map_entry(&mut self, v: bool) {
        self.map_entry = ::std::option::Option::Some(v);
    }

    pub fn get_map_entry(&self) -> bool {
        self.map_entry.unwrap_or(false)
    }

    fn get_map_entry_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.map_entry
    }

    fn mut_map_entry_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.map_entry
    }

    // repeated .google.protobuf.UninterpretedOption uninterpreted_option = 999;

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption>) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }

    // Take field
    pub fn take_uninterpreted_option(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption> {
        ::std::mem::replace(&mut self.uninterpreted_option, ::protobuf::RepeatedField::new())
    }

    pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_option
    }

    fn get_uninterpreted_option_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption> {
        &self.uninterpreted_option
    }

    fn mut_uninterpreted_option_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }
}

impl ::protobuf::Message for MessageOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.uninterpreted_option {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.message_set_wire_format = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.no_standard_descriptor_accessor = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deprecated = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.map_entry = ::std::option::Option::Some(tmp);
                },
                999 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uninterpreted_option)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.message_set_wire_format {
            my_size += 2;
        }
        if let Some(v) = self.no_standard_descriptor_accessor {
            my_size += 2;
        }
        if let Some(v) = self.deprecated {
            my_size += 2;
        }
        if let Some(v) = self.map_entry {
            my_size += 2;
        }
        for value in &self.uninterpreted_option {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message_set_wire_format {
            os.write_bool(1, v)?;
        }
        if let Some(v) = self.no_standard_descriptor_accessor {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.deprecated {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.map_entry {
            os.write_bool(7, v)?;
        }
        for v in &self.uninterpreted_option {
            os.write_tag(999, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MessageOptions {
    fn new() -> MessageOptions {
        MessageOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<MessageOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "message_set_wire_format",
                    MessageOptions::get_message_set_wire_format_for_reflect,
                    MessageOptions::mut_message_set_wire_format_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "no_standard_descriptor_accessor",
                    MessageOptions::get_no_standard_descriptor_accessor_for_reflect,
                    MessageOptions::mut_no_standard_descriptor_accessor_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deprecated",
                    MessageOptions::get_deprecated_for_reflect,
                    MessageOptions::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "map_entry",
                    MessageOptions::get_map_entry_for_reflect,
                    MessageOptions::mut_map_entry_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption>>(
                    "uninterpreted_option",
                    MessageOptions::get_uninterpreted_option_for_reflect,
                    MessageOptions::mut_uninterpreted_option_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MessageOptions>(
                    "MessageOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MessageOptions {
    fn clear(&mut self) {
        self.clear_message_set_wire_format();
        self.clear_no_standard_descriptor_accessor();
        self.clear_deprecated();
        self.clear_map_entry();
        self.clear_uninterpreted_option();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MessageOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MessageOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct FieldOptions {
    // message fields
    ctype: ::std::option::Option<FieldOptions_CType>,
    packed: ::std::option::Option<bool>,
    jstype: ::std::option::Option<FieldOptions_JSType>,
    lazy: ::std::option::Option<bool>,
    deprecated: ::std::option::Option<bool>,
    weak: ::std::option::Option<bool>,
    uninterpreted_option: ::protobuf::RepeatedField<UninterpretedOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FieldOptions {}

impl FieldOptions {
    pub fn new() -> FieldOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FieldOptions {
        static mut instance: ::protobuf::lazy::Lazy<FieldOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FieldOptions,
        };
        unsafe {
            instance.get(FieldOptions::new)
        }
    }

    // optional .google.protobuf.FieldOptions.CType ctype = 1;

    pub fn clear_ctype(&mut self) {
        self.ctype = ::std::option::Option::None;
    }

    pub fn has_ctype(&self) -> bool {
        self.ctype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ctype(&mut self, v: FieldOptions_CType) {
        self.ctype = ::std::option::Option::Some(v);
    }

    pub fn get_ctype(&self) -> FieldOptions_CType {
        self.ctype.unwrap_or(FieldOptions_CType::STRING)
    }

    fn get_ctype_for_reflect(&self) -> &::std::option::Option<FieldOptions_CType> {
        &self.ctype
    }

    fn mut_ctype_for_reflect(&mut self) -> &mut ::std::option::Option<FieldOptions_CType> {
        &mut self.ctype
    }

    // optional bool packed = 2;

    pub fn clear_packed(&mut self) {
        self.packed = ::std::option::Option::None;
    }

    pub fn has_packed(&self) -> bool {
        self.packed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_packed(&mut self, v: bool) {
        self.packed = ::std::option::Option::Some(v);
    }

    pub fn get_packed(&self) -> bool {
        self.packed.unwrap_or(false)
    }

    fn get_packed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.packed
    }

    fn mut_packed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.packed
    }

    // optional .google.protobuf.FieldOptions.JSType jstype = 6;

    pub fn clear_jstype(&mut self) {
        self.jstype = ::std::option::Option::None;
    }

    pub fn has_jstype(&self) -> bool {
        self.jstype.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jstype(&mut self, v: FieldOptions_JSType) {
        self.jstype = ::std::option::Option::Some(v);
    }

    pub fn get_jstype(&self) -> FieldOptions_JSType {
        self.jstype.unwrap_or(FieldOptions_JSType::JS_NORMAL)
    }

    fn get_jstype_for_reflect(&self) -> &::std::option::Option<FieldOptions_JSType> {
        &self.jstype
    }

    fn mut_jstype_for_reflect(&mut self) -> &mut ::std::option::Option<FieldOptions_JSType> {
        &mut self.jstype
    }

    // optional bool lazy = 5;

    pub fn clear_lazy(&mut self) {
        self.lazy = ::std::option::Option::None;
    }

    pub fn has_lazy(&self) -> bool {
        self.lazy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lazy(&mut self, v: bool) {
        self.lazy = ::std::option::Option::Some(v);
    }

    pub fn get_lazy(&self) -> bool {
        self.lazy.unwrap_or(false)
    }

    fn get_lazy_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.lazy
    }

    fn mut_lazy_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.lazy
    }

    // optional bool deprecated = 3;

    pub fn clear_deprecated(&mut self) {
        self.deprecated = ::std::option::Option::None;
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: bool) {
        self.deprecated = ::std::option::Option::Some(v);
    }

    pub fn get_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    fn get_deprecated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deprecated
    }

    // optional bool weak = 10;

    pub fn clear_weak(&mut self) {
        self.weak = ::std::option::Option::None;
    }

    pub fn has_weak(&self) -> bool {
        self.weak.is_some()
    }

    // Param is passed by value, moved
    pub fn set_weak(&mut self, v: bool) {
        self.weak = ::std::option::Option::Some(v);
    }

    pub fn get_weak(&self) -> bool {
        self.weak.unwrap_or(false)
    }

    fn get_weak_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.weak
    }

    fn mut_weak_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.weak
    }

    // repeated .google.protobuf.UninterpretedOption uninterpreted_option = 999;

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption>) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }

    // Take field
    pub fn take_uninterpreted_option(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption> {
        ::std::mem::replace(&mut self.uninterpreted_option, ::protobuf::RepeatedField::new())
    }

    pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_option
    }

    fn get_uninterpreted_option_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption> {
        &self.uninterpreted_option
    }

    fn mut_uninterpreted_option_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }
}

impl ::protobuf::Message for FieldOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.uninterpreted_option {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.ctype = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.packed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.jstype = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.lazy = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deprecated = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.weak = ::std::option::Option::Some(tmp);
                },
                999 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uninterpreted_option)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.ctype {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(v) = self.packed {
            my_size += 2;
        }
        if let Some(v) = self.jstype {
            my_size += ::protobuf::rt::enum_size(6, v);
        }
        if let Some(v) = self.lazy {
            my_size += 2;
        }
        if let Some(v) = self.deprecated {
            my_size += 2;
        }
        if let Some(v) = self.weak {
            my_size += 2;
        }
        for value in &self.uninterpreted_option {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ctype {
            os.write_enum(1, v.value())?;
        }
        if let Some(v) = self.packed {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.jstype {
            os.write_enum(6, v.value())?;
        }
        if let Some(v) = self.lazy {
            os.write_bool(5, v)?;
        }
        if let Some(v) = self.deprecated {
            os.write_bool(3, v)?;
        }
        if let Some(v) = self.weak {
            os.write_bool(10, v)?;
        }
        for v in &self.uninterpreted_option {
            os.write_tag(999, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FieldOptions {
    fn new() -> FieldOptions {
        FieldOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<FieldOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FieldOptions_CType>>(
                    "ctype",
                    FieldOptions::get_ctype_for_reflect,
                    FieldOptions::mut_ctype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "packed",
                    FieldOptions::get_packed_for_reflect,
                    FieldOptions::mut_packed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FieldOptions_JSType>>(
                    "jstype",
                    FieldOptions::get_jstype_for_reflect,
                    FieldOptions::mut_jstype_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "lazy",
                    FieldOptions::get_lazy_for_reflect,
                    FieldOptions::mut_lazy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deprecated",
                    FieldOptions::get_deprecated_for_reflect,
                    FieldOptions::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "weak",
                    FieldOptions::get_weak_for_reflect,
                    FieldOptions::mut_weak_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption>>(
                    "uninterpreted_option",
                    FieldOptions::get_uninterpreted_option_for_reflect,
                    FieldOptions::mut_uninterpreted_option_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FieldOptions>(
                    "FieldOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FieldOptions {
    fn clear(&mut self) {
        self.clear_ctype();
        self.clear_packed();
        self.clear_jstype();
        self.clear_lazy();
        self.clear_deprecated();
        self.clear_weak();
        self.clear_uninterpreted_option();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FieldOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FieldOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FieldOptions_CType {
    STRING = 0,
    CORD = 1,
    STRING_PIECE = 2,
}

impl ::protobuf::ProtobufEnum for FieldOptions_CType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FieldOptions_CType> {
        match value {
            0 => ::std::option::Option::Some(FieldOptions_CType::STRING),
            1 => ::std::option::Option::Some(FieldOptions_CType::CORD),
            2 => ::std::option::Option::Some(FieldOptions_CType::STRING_PIECE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FieldOptions_CType] = &[
            FieldOptions_CType::STRING,
            FieldOptions_CType::CORD,
            FieldOptions_CType::STRING_PIECE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<FieldOptions_CType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FieldOptions_CType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FieldOptions_CType {
}

impl ::protobuf::reflect::ProtobufValue for FieldOptions_CType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FieldOptions_JSType {
    JS_NORMAL = 0,
    JS_STRING = 1,
    JS_NUMBER = 2,
}

impl ::protobuf::ProtobufEnum for FieldOptions_JSType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FieldOptions_JSType> {
        match value {
            0 => ::std::option::Option::Some(FieldOptions_JSType::JS_NORMAL),
            1 => ::std::option::Option::Some(FieldOptions_JSType::JS_STRING),
            2 => ::std::option::Option::Some(FieldOptions_JSType::JS_NUMBER),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FieldOptions_JSType] = &[
            FieldOptions_JSType::JS_NORMAL,
            FieldOptions_JSType::JS_STRING,
            FieldOptions_JSType::JS_NUMBER,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<FieldOptions_JSType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("FieldOptions_JSType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for FieldOptions_JSType {
}

impl ::protobuf::reflect::ProtobufValue for FieldOptions_JSType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OneofOptions {
    // message fields
    uninterpreted_option: ::protobuf::RepeatedField<UninterpretedOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OneofOptions {}

impl OneofOptions {
    pub fn new() -> OneofOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OneofOptions {
        static mut instance: ::protobuf::lazy::Lazy<OneofOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OneofOptions,
        };
        unsafe {
            instance.get(OneofOptions::new)
        }
    }

    // repeated .google.protobuf.UninterpretedOption uninterpreted_option = 999;

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption>) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }

    // Take field
    pub fn take_uninterpreted_option(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption> {
        ::std::mem::replace(&mut self.uninterpreted_option, ::protobuf::RepeatedField::new())
    }

    pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_option
    }

    fn get_uninterpreted_option_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption> {
        &self.uninterpreted_option
    }

    fn mut_uninterpreted_option_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }
}

impl ::protobuf::Message for OneofOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.uninterpreted_option {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                999 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uninterpreted_option)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.uninterpreted_option {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.uninterpreted_option {
            os.write_tag(999, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OneofOptions {
    fn new() -> OneofOptions {
        OneofOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<OneofOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption>>(
                    "uninterpreted_option",
                    OneofOptions::get_uninterpreted_option_for_reflect,
                    OneofOptions::mut_uninterpreted_option_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OneofOptions>(
                    "OneofOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OneofOptions {
    fn clear(&mut self) {
        self.clear_uninterpreted_option();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OneofOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OneofOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnumOptions {
    // message fields
    allow_alias: ::std::option::Option<bool>,
    deprecated: ::std::option::Option<bool>,
    uninterpreted_option: ::protobuf::RepeatedField<UninterpretedOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnumOptions {}

impl EnumOptions {
    pub fn new() -> EnumOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnumOptions {
        static mut instance: ::protobuf::lazy::Lazy<EnumOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnumOptions,
        };
        unsafe {
            instance.get(EnumOptions::new)
        }
    }

    // optional bool allow_alias = 2;

    pub fn clear_allow_alias(&mut self) {
        self.allow_alias = ::std::option::Option::None;
    }

    pub fn has_allow_alias(&self) -> bool {
        self.allow_alias.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allow_alias(&mut self, v: bool) {
        self.allow_alias = ::std::option::Option::Some(v);
    }

    pub fn get_allow_alias(&self) -> bool {
        self.allow_alias.unwrap_or(false)
    }

    fn get_allow_alias_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allow_alias
    }

    fn mut_allow_alias_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allow_alias
    }

    // optional bool deprecated = 3;

    pub fn clear_deprecated(&mut self) {
        self.deprecated = ::std::option::Option::None;
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: bool) {
        self.deprecated = ::std::option::Option::Some(v);
    }

    pub fn get_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    fn get_deprecated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deprecated
    }

    // repeated .google.protobuf.UninterpretedOption uninterpreted_option = 999;

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption>) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }

    // Take field
    pub fn take_uninterpreted_option(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption> {
        ::std::mem::replace(&mut self.uninterpreted_option, ::protobuf::RepeatedField::new())
    }

    pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_option
    }

    fn get_uninterpreted_option_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption> {
        &self.uninterpreted_option
    }

    fn mut_uninterpreted_option_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }
}

impl ::protobuf::Message for EnumOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.uninterpreted_option {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.allow_alias = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deprecated = ::std::option::Option::Some(tmp);
                },
                999 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uninterpreted_option)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.allow_alias {
            my_size += 2;
        }
        if let Some(v) = self.deprecated {
            my_size += 2;
        }
        for value in &self.uninterpreted_option {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.allow_alias {
            os.write_bool(2, v)?;
        }
        if let Some(v) = self.deprecated {
            os.write_bool(3, v)?;
        }
        for v in &self.uninterpreted_option {
            os.write_tag(999, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EnumOptions {
    fn new() -> EnumOptions {
        EnumOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnumOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allow_alias",
                    EnumOptions::get_allow_alias_for_reflect,
                    EnumOptions::mut_allow_alias_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deprecated",
                    EnumOptions::get_deprecated_for_reflect,
                    EnumOptions::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption>>(
                    "uninterpreted_option",
                    EnumOptions::get_uninterpreted_option_for_reflect,
                    EnumOptions::mut_uninterpreted_option_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnumOptions>(
                    "EnumOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnumOptions {
    fn clear(&mut self) {
        self.clear_allow_alias();
        self.clear_deprecated();
        self.clear_uninterpreted_option();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnumOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnumOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnumValueOptions {
    // message fields
    deprecated: ::std::option::Option<bool>,
    uninterpreted_option: ::protobuf::RepeatedField<UninterpretedOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnumValueOptions {}

impl EnumValueOptions {
    pub fn new() -> EnumValueOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnumValueOptions {
        static mut instance: ::protobuf::lazy::Lazy<EnumValueOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnumValueOptions,
        };
        unsafe {
            instance.get(EnumValueOptions::new)
        }
    }

    // optional bool deprecated = 1;

    pub fn clear_deprecated(&mut self) {
        self.deprecated = ::std::option::Option::None;
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: bool) {
        self.deprecated = ::std::option::Option::Some(v);
    }

    pub fn get_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    fn get_deprecated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deprecated
    }

    // repeated .google.protobuf.UninterpretedOption uninterpreted_option = 999;

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption>) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }

    // Take field
    pub fn take_uninterpreted_option(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption> {
        ::std::mem::replace(&mut self.uninterpreted_option, ::protobuf::RepeatedField::new())
    }

    pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_option
    }

    fn get_uninterpreted_option_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption> {
        &self.uninterpreted_option
    }

    fn mut_uninterpreted_option_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }
}

impl ::protobuf::Message for EnumValueOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.uninterpreted_option {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deprecated = ::std::option::Option::Some(tmp);
                },
                999 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uninterpreted_option)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.deprecated {
            my_size += 2;
        }
        for value in &self.uninterpreted_option {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.deprecated {
            os.write_bool(1, v)?;
        }
        for v in &self.uninterpreted_option {
            os.write_tag(999, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EnumValueOptions {
    fn new() -> EnumValueOptions {
        EnumValueOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnumValueOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deprecated",
                    EnumValueOptions::get_deprecated_for_reflect,
                    EnumValueOptions::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption>>(
                    "uninterpreted_option",
                    EnumValueOptions::get_uninterpreted_option_for_reflect,
                    EnumValueOptions::mut_uninterpreted_option_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnumValueOptions>(
                    "EnumValueOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnumValueOptions {
    fn clear(&mut self) {
        self.clear_deprecated();
        self.clear_uninterpreted_option();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnumValueOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnumValueOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceOptions {
    // message fields
    deprecated: ::std::option::Option<bool>,
    uninterpreted_option: ::protobuf::RepeatedField<UninterpretedOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceOptions {}

impl ServiceOptions {
    pub fn new() -> ServiceOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceOptions {
        static mut instance: ::protobuf::lazy::Lazy<ServiceOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceOptions,
        };
        unsafe {
            instance.get(ServiceOptions::new)
        }
    }

    // optional bool deprecated = 33;

    pub fn clear_deprecated(&mut self) {
        self.deprecated = ::std::option::Option::None;
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: bool) {
        self.deprecated = ::std::option::Option::Some(v);
    }

    pub fn get_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    fn get_deprecated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deprecated
    }

    // repeated .google.protobuf.UninterpretedOption uninterpreted_option = 999;

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption>) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }

    // Take field
    pub fn take_uninterpreted_option(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption> {
        ::std::mem::replace(&mut self.uninterpreted_option, ::protobuf::RepeatedField::new())
    }

    pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_option
    }

    fn get_uninterpreted_option_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption> {
        &self.uninterpreted_option
    }

    fn mut_uninterpreted_option_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }
}

impl ::protobuf::Message for ServiceOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.uninterpreted_option {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deprecated = ::std::option::Option::Some(tmp);
                },
                999 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uninterpreted_option)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.deprecated {
            my_size += 3;
        }
        for value in &self.uninterpreted_option {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.deprecated {
            os.write_bool(33, v)?;
        }
        for v in &self.uninterpreted_option {
            os.write_tag(999, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServiceOptions {
    fn new() -> ServiceOptions {
        ServiceOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deprecated",
                    ServiceOptions::get_deprecated_for_reflect,
                    ServiceOptions::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption>>(
                    "uninterpreted_option",
                    ServiceOptions::get_uninterpreted_option_for_reflect,
                    ServiceOptions::mut_uninterpreted_option_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceOptions>(
                    "ServiceOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceOptions {
    fn clear(&mut self) {
        self.clear_deprecated();
        self.clear_uninterpreted_option();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MethodOptions {
    // message fields
    deprecated: ::std::option::Option<bool>,
    uninterpreted_option: ::protobuf::RepeatedField<UninterpretedOption>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MethodOptions {}

impl MethodOptions {
    pub fn new() -> MethodOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MethodOptions {
        static mut instance: ::protobuf::lazy::Lazy<MethodOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MethodOptions,
        };
        unsafe {
            instance.get(MethodOptions::new)
        }
    }

    // optional bool deprecated = 33;

    pub fn clear_deprecated(&mut self) {
        self.deprecated = ::std::option::Option::None;
    }

    pub fn has_deprecated(&self) -> bool {
        self.deprecated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deprecated(&mut self, v: bool) {
        self.deprecated = ::std::option::Option::Some(v);
    }

    pub fn get_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    fn get_deprecated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.deprecated
    }

    fn mut_deprecated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.deprecated
    }

    // repeated .google.protobuf.UninterpretedOption uninterpreted_option = 999;

    pub fn clear_uninterpreted_option(&mut self) {
        self.uninterpreted_option.clear();
    }

    // Param is passed by value, moved
    pub fn set_uninterpreted_option(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption>) {
        self.uninterpreted_option = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uninterpreted_option(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }

    // Take field
    pub fn take_uninterpreted_option(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption> {
        ::std::mem::replace(&mut self.uninterpreted_option, ::protobuf::RepeatedField::new())
    }

    pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
        &self.uninterpreted_option
    }

    fn get_uninterpreted_option_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption> {
        &self.uninterpreted_option
    }

    fn mut_uninterpreted_option_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption> {
        &mut self.uninterpreted_option
    }
}

impl ::protobuf::Message for MethodOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.uninterpreted_option {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.deprecated = ::std::option::Option::Some(tmp);
                },
                999 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.uninterpreted_option)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.deprecated {
            my_size += 3;
        }
        for value in &self.uninterpreted_option {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.deprecated {
            os.write_bool(33, v)?;
        }
        for v in &self.uninterpreted_option {
            os.write_tag(999, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MethodOptions {
    fn new() -> MethodOptions {
        MethodOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<MethodOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "deprecated",
                    MethodOptions::get_deprecated_for_reflect,
                    MethodOptions::mut_deprecated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption>>(
                    "uninterpreted_option",
                    MethodOptions::get_uninterpreted_option_for_reflect,
                    MethodOptions::mut_uninterpreted_option_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MethodOptions>(
                    "MethodOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MethodOptions {
    fn clear(&mut self) {
        self.clear_deprecated();
        self.clear_uninterpreted_option();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MethodOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MethodOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UninterpretedOption {
    // message fields
    name: ::protobuf::RepeatedField<UninterpretedOption_NamePart>,
    identifier_value: ::protobuf::SingularField<::std::string::String>,
    positive_int_value: ::std::option::Option<u64>,
    negative_int_value: ::std::option::Option<i64>,
    double_value: ::std::option::Option<f64>,
    string_value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    aggregate_value: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UninterpretedOption {}

impl UninterpretedOption {
    pub fn new() -> UninterpretedOption {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UninterpretedOption {
        static mut instance: ::protobuf::lazy::Lazy<UninterpretedOption> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UninterpretedOption,
        };
        unsafe {
            instance.get(UninterpretedOption::new)
        }
    }

    // repeated .google.protobuf.UninterpretedOption.NamePart name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::protobuf::RepeatedField<UninterpretedOption_NamePart>) {
        self.name = v;
    }

    // Mutable pointer to the field.
    pub fn mut_name(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption_NamePart> {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::protobuf::RepeatedField<UninterpretedOption_NamePart> {
        ::std::mem::replace(&mut self.name, ::protobuf::RepeatedField::new())
    }

    pub fn get_name(&self) -> &[UninterpretedOption_NamePart] {
        &self.name
    }

    fn get_name_for_reflect(&self) -> &::protobuf::RepeatedField<UninterpretedOption_NamePart> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<UninterpretedOption_NamePart> {
        &mut self.name
    }

    // optional string identifier_value = 3;

    pub fn clear_identifier_value(&mut self) {
        self.identifier_value.clear();
    }

    pub fn has_identifier_value(&self) -> bool {
        self.identifier_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_identifier_value(&mut self, v: ::std::string::String) {
        self.identifier_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_identifier_value(&mut self) -> &mut ::std::string::String {
        if self.identifier_value.is_none() {
            self.identifier_value.set_default();
        }
        self.identifier_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_identifier_value(&mut self) -> ::std::string::String {
        self.identifier_value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_identifier_value(&self) -> &str {
        match self.identifier_value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_identifier_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.identifier_value
    }

    fn mut_identifier_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.identifier_value
    }

    // optional uint64 positive_int_value = 4;

    pub fn clear_positive_int_value(&mut self) {
        self.positive_int_value = ::std::option::Option::None;
    }

    pub fn has_positive_int_value(&self) -> bool {
        self.positive_int_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_positive_int_value(&mut self, v: u64) {
        self.positive_int_value = ::std::option::Option::Some(v);
    }

    pub fn get_positive_int_value(&self) -> u64 {
        self.positive_int_value.unwrap_or(0)
    }

    fn get_positive_int_value_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.positive_int_value
    }

    fn mut_positive_int_value_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.positive_int_value
    }

    // optional int64 negative_int_value = 5;

    pub fn clear_negative_int_value(&mut self) {
        self.negative_int_value = ::std::option::Option::None;
    }

    pub fn has_negative_int_value(&self) -> bool {
        self.negative_int_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_negative_int_value(&mut self, v: i64) {
        self.negative_int_value = ::std::option::Option::Some(v);
    }

    pub fn get_negative_int_value(&self) -> i64 {
        self.negative_int_value.unwrap_or(0)
    }

    fn get_negative_int_value_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.negative_int_value
    }

    fn mut_negative_int_value_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.negative_int_value
    }

    // optional double double_value = 6;

    pub fn clear_double_value(&mut self) {
        self.double_value = ::std::option::Option::None;
    }

    pub fn has_double_value(&self) -> bool {
        self.double_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_double_value(&mut self, v: f64) {
        self.double_value = ::std::option::Option::Some(v);
    }

    pub fn get_double_value(&self) -> f64 {
        self.double_value.unwrap_or(0.)
    }

    fn get_double_value_for_reflect(&self) -> &::std::option::Option<f64> {
        &self.double_value
    }

    fn mut_double_value_for_reflect(&mut self) -> &mut ::std::option::Option<f64> {
        &mut self.double_value
    }

    // optional bytes string_value = 7;

    pub fn clear_string_value(&mut self) {
        self.string_value.clear();
    }

    pub fn has_string_value(&self) -> bool {
        self.string_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_string_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.string_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_string_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.string_value.is_none() {
            self.string_value.set_default();
        }
        self.string_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_string_value(&mut self) -> ::std::vec::Vec<u8> {
        self.string_value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_string_value(&self) -> &[u8] {
        match self.string_value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_string_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.string_value
    }

    fn mut_string_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.string_value
    }

    // optional string aggregate_value = 8;

    pub fn clear_aggregate_value(&mut self) {
        self.aggregate_value.clear();
    }

    pub fn has_aggregate_value(&self) -> bool {
        self.aggregate_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aggregate_value(&mut self, v: ::std::string::String) {
        self.aggregate_value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_aggregate_value(&mut self) -> &mut ::std::string::String {
        if self.aggregate_value.is_none() {
            self.aggregate_value.set_default();
        }
        self.aggregate_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_aggregate_value(&mut self) -> ::std::string::String {
        self.aggregate_value.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_aggregate_value(&self) -> &str {
        match self.aggregate_value.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_aggregate_value_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.aggregate_value
    }

    fn mut_aggregate_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.aggregate_value
    }
}

impl ::protobuf::Message for UninterpretedOption {
    fn is_initialized(&self) -> bool {
        for v in &self.name {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.identifier_value)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.positive_int_value = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.negative_int_value = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_double()?;
                    self.double_value = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.string_value)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.aggregate_value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.name {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.identifier_value.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.positive_int_value {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.negative_int_value {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.double_value {
            my_size += 9;
        }
        if let Some(ref v) = self.string_value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(7, &v);
        }
        if let Some(ref v) = self.aggregate_value.as_ref() {
            my_size += ::protobuf::rt::string_size(8, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.name {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.identifier_value.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.positive_int_value {
            os.write_uint64(4, v)?;
        }
        if let Some(v) = self.negative_int_value {
            os.write_int64(5, v)?;
        }
        if let Some(v) = self.double_value {
            os.write_double(6, v)?;
        }
        if let Some(ref v) = self.string_value.as_ref() {
            os.write_bytes(7, &v)?;
        }
        if let Some(ref v) = self.aggregate_value.as_ref() {
            os.write_string(8, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UninterpretedOption {
    fn new() -> UninterpretedOption {
        UninterpretedOption::new()
    }

    fn descriptor_static(_: ::std::option::Option<UninterpretedOption>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UninterpretedOption_NamePart>>(
                    "name",
                    UninterpretedOption::get_name_for_reflect,
                    UninterpretedOption::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "identifier_value",
                    UninterpretedOption::get_identifier_value_for_reflect,
                    UninterpretedOption::mut_identifier_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "positive_int_value",
                    UninterpretedOption::get_positive_int_value_for_reflect,
                    UninterpretedOption::mut_positive_int_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "negative_int_value",
                    UninterpretedOption::get_negative_int_value_for_reflect,
                    UninterpretedOption::mut_negative_int_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeDouble>(
                    "double_value",
                    UninterpretedOption::get_double_value_for_reflect,
                    UninterpretedOption::mut_double_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "string_value",
                    UninterpretedOption::get_string_value_for_reflect,
                    UninterpretedOption::mut_string_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "aggregate_value",
                    UninterpretedOption::get_aggregate_value_for_reflect,
                    UninterpretedOption::mut_aggregate_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UninterpretedOption>(
                    "UninterpretedOption",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UninterpretedOption {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_identifier_value();
        self.clear_positive_int_value();
        self.clear_negative_int_value();
        self.clear_double_value();
        self.clear_string_value();
        self.clear_aggregate_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UninterpretedOption {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UninterpretedOption {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UninterpretedOption_NamePart {
    // message fields
    name_part: ::protobuf::SingularField<::std::string::String>,
    is_extension: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UninterpretedOption_NamePart {}

impl UninterpretedOption_NamePart {
    pub fn new() -> UninterpretedOption_NamePart {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UninterpretedOption_NamePart {
        static mut instance: ::protobuf::lazy::Lazy<UninterpretedOption_NamePart> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UninterpretedOption_NamePart,
        };
        unsafe {
            instance.get(UninterpretedOption_NamePart::new)
        }
    }

    // required string name_part = 1;

    pub fn clear_name_part(&mut self) {
        self.name_part.clear();
    }

    pub fn has_name_part(&self) -> bool {
        self.name_part.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name_part(&mut self, v: ::std::string::String) {
        self.name_part = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name_part(&mut self) -> &mut ::std::string::String {
        if self.name_part.is_none() {
            self.name_part.set_default();
        }
        self.name_part.as_mut().unwrap()
    }

    // Take field
    pub fn take_name_part(&mut self) -> ::std::string::String {
        self.name_part.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name_part(&self) -> &str {
        match self.name_part.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_part_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name_part
    }

    fn mut_name_part_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name_part
    }

    // required bool is_extension = 2;

    pub fn clear_is_extension(&mut self) {
        self.is_extension = ::std::option::Option::None;
    }

    pub fn has_is_extension(&self) -> bool {
        self.is_extension.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_extension(&mut self, v: bool) {
        self.is_extension = ::std::option::Option::Some(v);
    }

    pub fn get_is_extension(&self) -> bool {
        self.is_extension.unwrap_or(false)
    }

    fn get_is_extension_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_extension
    }

    fn mut_is_extension_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_extension
    }
}

impl ::protobuf::Message for UninterpretedOption_NamePart {
    fn is_initialized(&self) -> bool {
        if self.name_part.is_none() {
            return false;
        }
        if self.is_extension.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name_part)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_extension = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.name_part.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.is_extension {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name_part.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.is_extension {
            os.write_bool(2, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UninterpretedOption_NamePart {
    fn new() -> UninterpretedOption_NamePart {
        UninterpretedOption_NamePart::new()
    }

    fn descriptor_static(_: ::std::option::Option<UninterpretedOption_NamePart>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name_part",
                    UninterpretedOption_NamePart::get_name_part_for_reflect,
                    UninterpretedOption_NamePart::mut_name_part_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_extension",
                    UninterpretedOption_NamePart::get_is_extension_for_reflect,
                    UninterpretedOption_NamePart::mut_is_extension_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UninterpretedOption_NamePart>(
                    "UninterpretedOption_NamePart",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UninterpretedOption_NamePart {
    fn clear(&mut self) {
        self.clear_name_part();
        self.clear_is_extension();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UninterpretedOption_NamePart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UninterpretedOption_NamePart {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SourceCodeInfo {
    // message fields
    location: ::protobuf::RepeatedField<SourceCodeInfo_Location>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SourceCodeInfo {}

impl SourceCodeInfo {
    pub fn new() -> SourceCodeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SourceCodeInfo {
        static mut instance: ::protobuf::lazy::Lazy<SourceCodeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SourceCodeInfo,
        };
        unsafe {
            instance.get(SourceCodeInfo::new)
        }
    }

    // repeated .google.protobuf.SourceCodeInfo.Location location = 1;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::protobuf::RepeatedField<SourceCodeInfo_Location>) {
        self.location = v;
    }

    // Mutable pointer to the field.
    pub fn mut_location(&mut self) -> &mut ::protobuf::RepeatedField<SourceCodeInfo_Location> {
        &mut self.location
    }

    // Take field
    pub fn take_location(&mut self) -> ::protobuf::RepeatedField<SourceCodeInfo_Location> {
        ::std::mem::replace(&mut self.location, ::protobuf::RepeatedField::new())
    }

    pub fn get_location(&self) -> &[SourceCodeInfo_Location] {
        &self.location
    }

    fn get_location_for_reflect(&self) -> &::protobuf::RepeatedField<SourceCodeInfo_Location> {
        &self.location
    }

    fn mut_location_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SourceCodeInfo_Location> {
        &mut self.location
    }
}

impl ::protobuf::Message for SourceCodeInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.location {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.location)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.location {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.location {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SourceCodeInfo {
    fn new() -> SourceCodeInfo {
        SourceCodeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<SourceCodeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SourceCodeInfo_Location>>(
                    "location",
                    SourceCodeInfo::get_location_for_reflect,
                    SourceCodeInfo::mut_location_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SourceCodeInfo>(
                    "SourceCodeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SourceCodeInfo {
    fn clear(&mut self) {
        self.clear_location();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SourceCodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SourceCodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SourceCodeInfo_Location {
    // message fields
    path: ::std::vec::Vec<i32>,
    span: ::std::vec::Vec<i32>,
    leading_comments: ::protobuf::SingularField<::std::string::String>,
    trailing_comments: ::protobuf::SingularField<::std::string::String>,
    leading_detached_comments: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SourceCodeInfo_Location {}

impl SourceCodeInfo_Location {
    pub fn new() -> SourceCodeInfo_Location {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SourceCodeInfo_Location {
        static mut instance: ::protobuf::lazy::Lazy<SourceCodeInfo_Location> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SourceCodeInfo_Location,
        };
        unsafe {
            instance.get(SourceCodeInfo_Location::new)
        }
    }

    // repeated int32 path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::vec::Vec<i32>) {
        self.path = v;
    }

    // Mutable pointer to the field.
    pub fn mut_path(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.path, ::std::vec::Vec::new())
    }

    pub fn get_path(&self) -> &[i32] {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.path
    }

    // repeated int32 span = 2;

    pub fn clear_span(&mut self) {
        self.span.clear();
    }

    // Param is passed by value, moved
    pub fn set_span(&mut self, v: ::std::vec::Vec<i32>) {
        self.span = v;
    }

    // Mutable pointer to the field.
    pub fn mut_span(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.span
    }

    // Take field
    pub fn take_span(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.span, ::std::vec::Vec::new())
    }

    pub fn get_span(&self) -> &[i32] {
        &self.span
    }

    fn get_span_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.span
    }

    fn mut_span_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.span
    }

    // optional string leading_comments = 3;

    pub fn clear_leading_comments(&mut self) {
        self.leading_comments.clear();
    }

    pub fn has_leading_comments(&self) -> bool {
        self.leading_comments.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leading_comments(&mut self, v: ::std::string::String) {
        self.leading_comments = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leading_comments(&mut self) -> &mut ::std::string::String {
        if self.leading_comments.is_none() {
            self.leading_comments.set_default();
        }
        self.leading_comments.as_mut().unwrap()
    }

    // Take field
    pub fn take_leading_comments(&mut self) -> ::std::string::String {
        self.leading_comments.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_leading_comments(&self) -> &str {
        match self.leading_comments.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_leading_comments_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.leading_comments
    }

    fn mut_leading_comments_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.leading_comments
    }

    // optional string trailing_comments = 4;

    pub fn clear_trailing_comments(&mut self) {
        self.trailing_comments.clear();
    }

    pub fn has_trailing_comments(&self) -> bool {
        self.trailing_comments.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trailing_comments(&mut self, v: ::std::string::String) {
        self.trailing_comments = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_trailing_comments(&mut self) -> &mut ::std::string::String {
        if self.trailing_comments.is_none() {
            self.trailing_comments.set_default();
        }
        self.trailing_comments.as_mut().unwrap()
    }

    // Take field
    pub fn take_trailing_comments(&mut self) -> ::std::string::String {
        self.trailing_comments.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_trailing_comments(&self) -> &str {
        match self.trailing_comments.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_trailing_comments_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.trailing_comments
    }

    fn mut_trailing_comments_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.trailing_comments
    }

    // repeated string leading_detached_comments = 6;

    pub fn clear_leading_detached_comments(&mut self) {
        self.leading_detached_comments.clear();
    }

    // Param is passed by value, moved
    pub fn set_leading_detached_comments(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.leading_detached_comments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_leading_detached_comments(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.leading_detached_comments
    }

    // Take field
    pub fn take_leading_detached_comments(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.leading_detached_comments, ::protobuf::RepeatedField::new())
    }

    pub fn get_leading_detached_comments(&self) -> &[::std::string::String] {
        &self.leading_detached_comments
    }

    fn get_leading_detached_comments_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.leading_detached_comments
    }

    fn mut_leading_detached_comments_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.leading_detached_comments
    }
}

impl ::protobuf::Message for SourceCodeInfo_Location {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.span)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.leading_comments)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.trailing_comments)?;
                },
                6 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.leading_detached_comments)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.path);
        }
        if !self.span.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.span);
        }
        if let Some(ref v) = self.leading_comments.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.trailing_comments.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        for value in &self.leading_detached_comments {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.path))?;
            for v in &self.path {
                os.write_int32_no_tag(*v)?;
            };
        }
        if !self.span.is_empty() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.span))?;
            for v in &self.span {
                os.write_int32_no_tag(*v)?;
            };
        }
        if let Some(ref v) = self.leading_comments.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.trailing_comments.as_ref() {
            os.write_string(4, &v)?;
        }
        for v in &self.leading_detached_comments {
            os.write_string(6, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SourceCodeInfo_Location {
    fn new() -> SourceCodeInfo_Location {
        SourceCodeInfo_Location::new()
    }

    fn descriptor_static(_: ::std::option::Option<SourceCodeInfo_Location>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "path",
                    SourceCodeInfo_Location::get_path_for_reflect,
                    SourceCodeInfo_Location::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "span",
                    SourceCodeInfo_Location::get_span_for_reflect,
                    SourceCodeInfo_Location::mut_span_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "leading_comments",
                    SourceCodeInfo_Location::get_leading_comments_for_reflect,
                    SourceCodeInfo_Location::mut_leading_comments_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "trailing_comments",
                    SourceCodeInfo_Location::get_trailing_comments_for_reflect,
                    SourceCodeInfo_Location::mut_trailing_comments_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "leading_detached_comments",
                    SourceCodeInfo_Location::get_leading_detached_comments_for_reflect,
                    SourceCodeInfo_Location::mut_leading_detached_comments_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SourceCodeInfo_Location>(
                    "SourceCodeInfo_Location",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SourceCodeInfo_Location {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_span();
        self.clear_leading_comments();
        self.clear_trailing_comments();
        self.clear_leading_detached_comments();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SourceCodeInfo_Location {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SourceCodeInfo_Location {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GeneratedCodeInfo {
    // message fields
    annotation: ::protobuf::RepeatedField<GeneratedCodeInfo_Annotation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GeneratedCodeInfo {}

impl GeneratedCodeInfo {
    pub fn new() -> GeneratedCodeInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GeneratedCodeInfo {
        static mut instance: ::protobuf::lazy::Lazy<GeneratedCodeInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GeneratedCodeInfo,
        };
        unsafe {
            instance.get(GeneratedCodeInfo::new)
        }
    }

    // repeated .google.protobuf.GeneratedCodeInfo.Annotation annotation = 1;

    pub fn clear_annotation(&mut self) {
        self.annotation.clear();
    }

    // Param is passed by value, moved
    pub fn set_annotation(&mut self, v: ::protobuf::RepeatedField<GeneratedCodeInfo_Annotation>) {
        self.annotation = v;
    }

    // Mutable pointer to the field.
    pub fn mut_annotation(&mut self) -> &mut ::protobuf::RepeatedField<GeneratedCodeInfo_Annotation> {
        &mut self.annotation
    }

    // Take field
    pub fn take_annotation(&mut self) -> ::protobuf::RepeatedField<GeneratedCodeInfo_Annotation> {
        ::std::mem::replace(&mut self.annotation, ::protobuf::RepeatedField::new())
    }

    pub fn get_annotation(&self) -> &[GeneratedCodeInfo_Annotation] {
        &self.annotation
    }

    fn get_annotation_for_reflect(&self) -> &::protobuf::RepeatedField<GeneratedCodeInfo_Annotation> {
        &self.annotation
    }

    fn mut_annotation_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<GeneratedCodeInfo_Annotation> {
        &mut self.annotation
    }
}

impl ::protobuf::Message for GeneratedCodeInfo {
    fn is_initialized(&self) -> bool {
        for v in &self.annotation {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.annotation)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.annotation {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.annotation {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GeneratedCodeInfo {
    fn new() -> GeneratedCodeInfo {
        GeneratedCodeInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<GeneratedCodeInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GeneratedCodeInfo_Annotation>>(
                    "annotation",
                    GeneratedCodeInfo::get_annotation_for_reflect,
                    GeneratedCodeInfo::mut_annotation_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GeneratedCodeInfo>(
                    "GeneratedCodeInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GeneratedCodeInfo {
    fn clear(&mut self) {
        self.clear_annotation();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GeneratedCodeInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GeneratedCodeInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GeneratedCodeInfo_Annotation {
    // message fields
    path: ::std::vec::Vec<i32>,
    source_file: ::protobuf::SingularField<::std::string::String>,
    begin: ::std::option::Option<i32>,
    end: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GeneratedCodeInfo_Annotation {}

impl GeneratedCodeInfo_Annotation {
    pub fn new() -> GeneratedCodeInfo_Annotation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GeneratedCodeInfo_Annotation {
        static mut instance: ::protobuf::lazy::Lazy<GeneratedCodeInfo_Annotation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GeneratedCodeInfo_Annotation,
        };
        unsafe {
            instance.get(GeneratedCodeInfo_Annotation::new)
        }
    }

    // repeated int32 path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::vec::Vec<i32>) {
        self.path = v;
    }

    // Mutable pointer to the field.
    pub fn mut_path(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.path
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::vec::Vec<i32> {
        ::std::mem::replace(&mut self.path, ::std::vec::Vec::new())
    }

    pub fn get_path(&self) -> &[i32] {
        &self.path
    }

    fn get_path_for_reflect(&self) -> &::std::vec::Vec<i32> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::std::vec::Vec<i32> {
        &mut self.path
    }

    // optional string source_file = 2;

    pub fn clear_source_file(&mut self) {
        self.source_file.clear();
    }

    pub fn has_source_file(&self) -> bool {
        self.source_file.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source_file(&mut self, v: ::std::string::String) {
        self.source_file = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source_file(&mut self) -> &mut ::std::string::String {
        if self.source_file.is_none() {
            self.source_file.set_default();
        }
        self.source_file.as_mut().unwrap()
    }

    // Take field
    pub fn take_source_file(&mut self) -> ::std::string::String {
        self.source_file.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_source_file(&self) -> &str {
        match self.source_file.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_source_file_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.source_file
    }

    fn mut_source_file_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.source_file
    }

    // optional int32 begin = 3;

    pub fn clear_begin(&mut self) {
        self.begin = ::std::option::Option::None;
    }

    pub fn has_begin(&self) -> bool {
        self.begin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_begin(&mut self, v: i32) {
        self.begin = ::std::option::Option::Some(v);
    }

    pub fn get_begin(&self) -> i32 {
        self.begin.unwrap_or(0)
    }

    fn get_begin_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.begin
    }

    fn mut_begin_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.begin
    }

    // optional int32 end = 4;

    pub fn clear_end(&mut self) {
        self.end = ::std::option::Option::None;
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: i32) {
        self.end = ::std::option::Option::Some(v);
    }

    pub fn get_end(&self) -> i32 {
        self.end.unwrap_or(0)
    }

    fn get_end_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.end
    }
}

impl ::protobuf::Message for GeneratedCodeInfo_Annotation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_int32_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.source_file)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.begin = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.end = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.path.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(1, &self.path);
        }
        if let Some(ref v) = self.source_file.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(v) = self.begin {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.end {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.path.is_empty() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            // TODO: Data size is computed again, it should be cached
            os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.path))?;
            for v in &self.path {
                os.write_int32_no_tag(*v)?;
            };
        }
        if let Some(ref v) = self.source_file.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(v) = self.begin {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.end {
            os.write_int32(4, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GeneratedCodeInfo_Annotation {
    fn new() -> GeneratedCodeInfo_Annotation {
        GeneratedCodeInfo_Annotation::new()
    }

    fn descriptor_static(_: ::std::option::Option<GeneratedCodeInfo_Annotation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "path",
                    GeneratedCodeInfo_Annotation::get_path_for_reflect,
                    GeneratedCodeInfo_Annotation::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "source_file",
                    GeneratedCodeInfo_Annotation::get_source_file_for_reflect,
                    GeneratedCodeInfo_Annotation::mut_source_file_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "begin",
                    GeneratedCodeInfo_Annotation::get_begin_for_reflect,
                    GeneratedCodeInfo_Annotation::mut_begin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "end",
                    GeneratedCodeInfo_Annotation::get_end_for_reflect,
                    GeneratedCodeInfo_Annotation::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GeneratedCodeInfo_Annotation>(
                    "GeneratedCodeInfo_Annotation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GeneratedCodeInfo_Annotation {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_source_file();
        self.clear_begin();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GeneratedCodeInfo_Annotation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GeneratedCodeInfo_Annotation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20google/protobuf/descriptor.proto\x12\x0fgoogle.protobuf\"M\n\x11Fi\
    leDescriptorSet\x128\n\x04file\x18\x01\x20\x03(\x0b2$.google.protobuf.Fi\
    leDescriptorProtoR\x04file\"\xe4\x04\n\x13FileDescriptorProto\x12\x12\n\
    \x04name\x18\x01\x20\x01(\tR\x04name\x12\x18\n\x07package\x18\x02\x20\
    \x01(\tR\x07package\x12\x1e\n\ndependency\x18\x03\x20\x03(\tR\ndependenc\
    y\x12+\n\x11public_dependency\x18\n\x20\x03(\x05R\x10publicDependency\
    \x12'\n\x0fweak_dependency\x18\x0b\x20\x03(\x05R\x0eweakDependency\x12C\
    \n\x0cmessage_type\x18\x04\x20\x03(\x0b2\x20.google.protobuf.DescriptorP\
    rotoR\x0bmessageType\x12A\n\tenum_type\x18\x05\x20\x03(\x0b2$.google.pro\
    tobuf.EnumDescriptorProtoR\x08enumType\x12A\n\x07service\x18\x06\x20\x03\
    (\x0b2'.google.protobuf.ServiceDescriptorProtoR\x07service\x12C\n\texten\
    sion\x18\x07\x20\x03(\x0b2%.google.protobuf.FieldDescriptorProtoR\texten\
    sion\x126\n\x07options\x18\x08\x20\x01(\x0b2\x1c.google.protobuf.FileOpt\
    ionsR\x07options\x12I\n\x10source_code_info\x18\t\x20\x01(\x0b2\x1f.goog\
    le.protobuf.SourceCodeInfoR\x0esourceCodeInfo\x12\x16\n\x06syntax\x18\
    \x0c\x20\x01(\tR\x06syntax\"\xf7\x05\n\x0fDescriptorProto\x12\x12\n\x04n\
    ame\x18\x01\x20\x01(\tR\x04name\x12;\n\x05field\x18\x02\x20\x03(\x0b2%.g\
    oogle.protobuf.FieldDescriptorProtoR\x05field\x12C\n\textension\x18\x06\
    \x20\x03(\x0b2%.google.protobuf.FieldDescriptorProtoR\textension\x12A\n\
    \x0bnested_type\x18\x03\x20\x03(\x0b2\x20.google.protobuf.DescriptorProt\
    oR\nnestedType\x12A\n\tenum_type\x18\x04\x20\x03(\x0b2$.google.protobuf.\
    EnumDescriptorProtoR\x08enumType\x12X\n\x0fextension_range\x18\x05\x20\
    \x03(\x0b2/.google.protobuf.DescriptorProto.ExtensionRangeR\x0eextension\
    Range\x12D\n\noneof_decl\x18\x08\x20\x03(\x0b2%.google.protobuf.OneofDes\
    criptorProtoR\toneofDecl\x129\n\x07options\x18\x07\x20\x01(\x0b2\x1f.goo\
    gle.protobuf.MessageOptionsR\x07options\x12U\n\x0ereserved_range\x18\t\
    \x20\x03(\x0b2..google.protobuf.DescriptorProto.ReservedRangeR\rreserved\
    Range\x12#\n\rreserved_name\x18\n\x20\x03(\tR\x0creservedName\x1a8\n\x0e\
    ExtensionRange\x12\x14\n\x05start\x18\x01\x20\x01(\x05R\x05start\x12\x10\
    \n\x03end\x18\x02\x20\x01(\x05R\x03end\x1a7\n\rReservedRange\x12\x14\n\
    \x05start\x18\x01\x20\x01(\x05R\x05start\x12\x10\n\x03end\x18\x02\x20\
    \x01(\x05R\x03end\"\x98\x06\n\x14FieldDescriptorProto\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\x12\x16\n\x06number\x18\x03\x20\x01(\x05R\
    \x06number\x12A\n\x05label\x18\x04\x20\x01(\x0e2+.google.protobuf.FieldD\
    escriptorProto.LabelR\x05label\x12>\n\x04type\x18\x05\x20\x01(\x0e2*.goo\
    gle.protobuf.FieldDescriptorProto.TypeR\x04type\x12\x1b\n\ttype_name\x18\
    \x06\x20\x01(\tR\x08typeName\x12\x1a\n\x08extendee\x18\x02\x20\x01(\tR\
    \x08extendee\x12#\n\rdefault_value\x18\x07\x20\x01(\tR\x0cdefaultValue\
    \x12\x1f\n\x0boneof_index\x18\t\x20\x01(\x05R\noneofIndex\x12\x1b\n\tjso\
    n_name\x18\n\x20\x01(\tR\x08jsonName\x127\n\x07options\x18\x08\x20\x01(\
    \x0b2\x1d.google.protobuf.FieldOptionsR\x07options\"\xb6\x02\n\x04Type\
    \x12\x0f\n\x0bTYPE_DOUBLE\x10\x01\x12\x0e\n\nTYPE_FLOAT\x10\x02\x12\x0e\
    \n\nTYPE_INT64\x10\x03\x12\x0f\n\x0bTYPE_UINT64\x10\x04\x12\x0e\n\nTYPE_\
    INT32\x10\x05\x12\x10\n\x0cTYPE_FIXED64\x10\x06\x12\x10\n\x0cTYPE_FIXED3\
    2\x10\x07\x12\r\n\tTYPE_BOOL\x10\x08\x12\x0f\n\x0bTYPE_STRING\x10\t\x12\
    \x0e\n\nTYPE_GROUP\x10\n\x12\x10\n\x0cTYPE_MESSAGE\x10\x0b\x12\x0e\n\nTY\
    PE_BYTES\x10\x0c\x12\x0f\n\x0bTYPE_UINT32\x10\r\x12\r\n\tTYPE_ENUM\x10\
    \x0e\x12\x11\n\rTYPE_SFIXED32\x10\x0f\x12\x11\n\rTYPE_SFIXED64\x10\x10\
    \x12\x0f\n\x0bTYPE_SINT32\x10\x11\x12\x0f\n\x0bTYPE_SINT64\x10\x12\"C\n\
    \x05Label\x12\x12\n\x0eLABEL_OPTIONAL\x10\x01\x12\x12\n\x0eLABEL_REQUIRE\
    D\x10\x02\x12\x12\n\x0eLABEL_REPEATED\x10\x03\"c\n\x14OneofDescriptorPro\
    to\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x127\n\x07options\x18\
    \x02\x20\x01(\x0b2\x1d.google.protobuf.OneofOptionsR\x07options\"\xa2\
    \x01\n\x13EnumDescriptorProto\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04n\
    ame\x12?\n\x05value\x18\x02\x20\x03(\x0b2).google.protobuf.EnumValueDesc\
    riptorProtoR\x05value\x126\n\x07options\x18\x03\x20\x01(\x0b2\x1c.google\
    .protobuf.EnumOptionsR\x07options\"\x83\x01\n\x18EnumValueDescriptorProt\
    o\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x16\n\x06number\x18\
    \x02\x20\x01(\x05R\x06number\x12;\n\x07options\x18\x03\x20\x01(\x0b2!.go\
    ogle.protobuf.EnumValueOptionsR\x07options\"\xa7\x01\n\x16ServiceDescrip\
    torProto\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12>\n\x06method\
    \x18\x02\x20\x03(\x0b2&.google.protobuf.MethodDescriptorProtoR\x06method\
    \x129\n\x07options\x18\x03\x20\x01(\x0b2\x1f.google.protobuf.ServiceOpti\
    onsR\x07options\"\x89\x02\n\x15MethodDescriptorProto\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\x12\x1d\n\ninput_type\x18\x02\x20\x01(\tR\t\
    inputType\x12\x1f\n\x0boutput_type\x18\x03\x20\x01(\tR\noutputType\x128\
    \n\x07options\x18\x04\x20\x01(\x0b2\x1e.google.protobuf.MethodOptionsR\
    \x07options\x120\n\x10client_streaming\x18\x05\x20\x01(\x08:\x05falseR\
    \x0fclientStreaming\x120\n\x10server_streaming\x18\x06\x20\x01(\x08:\x05\
    falseR\x0fserverStreaming\"\x88\x07\n\x0bFileOptions\x12!\n\x0cjava_pack\
    age\x18\x01\x20\x01(\tR\x0bjavaPackage\x120\n\x14java_outer_classname\
    \x18\x08\x20\x01(\tR\x12javaOuterClassname\x125\n\x13java_multiple_files\
    \x18\n\x20\x01(\x08:\x05falseR\x11javaMultipleFiles\x12D\n\x1djava_gener\
    ate_equals_and_hash\x18\x14\x20\x01(\x08R\x19javaGenerateEqualsAndHashB\
    \x02\x18\x01\x12:\n\x16java_string_check_utf8\x18\x1b\x20\x01(\x08:\x05f\
    alseR\x13javaStringCheckUtf8\x12S\n\x0coptimize_for\x18\t\x20\x01(\x0e2)\
    .google.protobuf.FileOptions.OptimizeMode:\x05SPEEDR\x0boptimizeFor\x12\
    \x1d\n\ngo_package\x18\x0b\x20\x01(\tR\tgoPackage\x125\n\x13cc_generic_s\
    ervices\x18\x10\x20\x01(\x08:\x05falseR\x11ccGenericServices\x129\n\x15j\
    ava_generic_services\x18\x11\x20\x01(\x08:\x05falseR\x13javaGenericServi\
    ces\x125\n\x13py_generic_services\x18\x12\x20\x01(\x08:\x05falseR\x11pyG\
    enericServices\x12%\n\ndeprecated\x18\x17\x20\x01(\x08:\x05falseR\ndepre\
    cated\x12/\n\x10cc_enable_arenas\x18\x1f\x20\x01(\x08:\x05falseR\x0eccEn\
    ableArenas\x12*\n\x11objc_class_prefix\x18$\x20\x01(\tR\x0fobjcClassPref\
    ix\x12)\n\x10csharp_namespace\x18%\x20\x01(\tR\x0fcsharpNamespace\x12X\n\
    \x14uninterpreted_option\x18\xe7\x07\x20\x03(\x0b2$.google.protobuf.Unin\
    terpretedOptionR\x13uninterpretedOption\":\n\x0cOptimizeMode\x12\t\n\x05\
    SPEED\x10\x01\x12\r\n\tCODE_SIZE\x10\x02\x12\x10\n\x0cLITE_RUNTIME\x10\
    \x03*\t\x08\xe8\x07\x10\x80\x80\x80\x80\x02\"\xc5\x02\n\x0eMessageOption\
    s\x12<\n\x17message_set_wire_format\x18\x01\x20\x01(\x08:\x05falseR\x14m\
    essageSetWireFormat\x12L\n\x1fno_standard_descriptor_accessor\x18\x02\
    \x20\x01(\x08:\x05falseR\x1cnoStandardDescriptorAccessor\x12%\n\ndepreca\
    ted\x18\x03\x20\x01(\x08:\x05falseR\ndeprecated\x12\x1b\n\tmap_entry\x18\
    \x07\x20\x01(\x08R\x08mapEntry\x12X\n\x14uninterpreted_option\x18\xe7\
    \x07\x20\x03(\x0b2$.google.protobuf.UninterpretedOptionR\x13uninterprete\
    dOption*\t\x08\xe8\x07\x10\x80\x80\x80\x80\x02\"\xdc\x03\n\x0cFieldOptio\
    ns\x12A\n\x05ctype\x18\x01\x20\x01(\x0e2#.google.protobuf.FieldOptions.C\
    Type:\x06STRINGR\x05ctype\x12\x16\n\x06packed\x18\x02\x20\x01(\x08R\x06p\
    acked\x12G\n\x06jstype\x18\x06\x20\x01(\x0e2$.google.protobuf.FieldOptio\
    ns.JSType:\tJS_NORMALR\x06jstype\x12\x19\n\x04lazy\x18\x05\x20\x01(\x08:\
    \x05falseR\x04lazy\x12%\n\ndeprecated\x18\x03\x20\x01(\x08:\x05falseR\nd\
    eprecated\x12\x19\n\x04weak\x18\n\x20\x01(\x08:\x05falseR\x04weak\x12X\n\
    \x14uninterpreted_option\x18\xe7\x07\x20\x03(\x0b2$.google.protobuf.Unin\
    terpretedOptionR\x13uninterpretedOption\"/\n\x05CType\x12\n\n\x06STRING\
    \x10\0\x12\x08\n\x04CORD\x10\x01\x12\x10\n\x0cSTRING_PIECE\x10\x02\"5\n\
    \x06JSType\x12\r\n\tJS_NORMAL\x10\0\x12\r\n\tJS_STRING\x10\x01\x12\r\n\t\
    JS_NUMBER\x10\x02*\t\x08\xe8\x07\x10\x80\x80\x80\x80\x02\"s\n\x0cOneofOp\
    tions\x12X\n\x14uninterpreted_option\x18\xe7\x07\x20\x03(\x0b2$.google.p\
    rotobuf.UninterpretedOptionR\x13uninterpretedOption*\t\x08\xe8\x07\x10\
    \x80\x80\x80\x80\x02\"\xba\x01\n\x0bEnumOptions\x12\x1f\n\x0ballow_alias\
    \x18\x02\x20\x01(\x08R\nallowAlias\x12%\n\ndeprecated\x18\x03\x20\x01(\
    \x08:\x05falseR\ndeprecated\x12X\n\x14uninterpreted_option\x18\xe7\x07\
    \x20\x03(\x0b2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOpt\
    ion*\t\x08\xe8\x07\x10\x80\x80\x80\x80\x02\"\x9e\x01\n\x10EnumValueOptio\
    ns\x12%\n\ndeprecated\x18\x01\x20\x01(\x08:\x05falseR\ndeprecated\x12X\n\
    \x14uninterpreted_option\x18\xe7\x07\x20\x03(\x0b2$.google.protobuf.Unin\
    terpretedOptionR\x13uninterpretedOption*\t\x08\xe8\x07\x10\x80\x80\x80\
    \x80\x02\"\x9c\x01\n\x0eServiceOptions\x12%\n\ndeprecated\x18!\x20\x01(\
    \x08:\x05falseR\ndeprecated\x12X\n\x14uninterpreted_option\x18\xe7\x07\
    \x20\x03(\x0b2$.google.protobuf.UninterpretedOptionR\x13uninterpretedOpt\
    ion*\t\x08\xe8\x07\x10\x80\x80\x80\x80\x02\"\x9b\x01\n\rMethodOptions\
    \x12%\n\ndeprecated\x18!\x20\x01(\x08:\x05falseR\ndeprecated\x12X\n\x14u\
    ninterpreted_option\x18\xe7\x07\x20\x03(\x0b2$.google.protobuf.Uninterpr\
    etedOptionR\x13uninterpretedOption*\t\x08\xe8\x07\x10\x80\x80\x80\x80\
    \x02\"\x9a\x03\n\x13UninterpretedOption\x12A\n\x04name\x18\x02\x20\x03(\
    \x0b2-.google.protobuf.UninterpretedOption.NamePartR\x04name\x12)\n\x10i\
    dentifier_value\x18\x03\x20\x01(\tR\x0fidentifierValue\x12,\n\x12positiv\
    e_int_value\x18\x04\x20\x01(\x04R\x10positiveIntValue\x12,\n\x12negative\
    _int_value\x18\x05\x20\x01(\x03R\x10negativeIntValue\x12!\n\x0cdouble_va\
    lue\x18\x06\x20\x01(\x01R\x0bdoubleValue\x12!\n\x0cstring_value\x18\x07\
    \x20\x01(\x0cR\x0bstringValue\x12'\n\x0faggregate_value\x18\x08\x20\x01(\
    \tR\x0eaggregateValue\x1aJ\n\x08NamePart\x12\x1b\n\tname_part\x18\x01\
    \x20\x02(\tR\x08namePart\x12!\n\x0cis_extension\x18\x02\x20\x02(\x08R\
    \x0bisExtension\"\xa7\x02\n\x0eSourceCodeInfo\x12D\n\x08location\x18\x01\
    \x20\x03(\x0b2(.google.protobuf.SourceCodeInfo.LocationR\x08location\x1a\
    \xce\x01\n\x08Location\x12\x16\n\x04path\x18\x01\x20\x03(\x05R\x04pathB\
    \x02\x10\x01\x12\x16\n\x04span\x18\x02\x20\x03(\x05R\x04spanB\x02\x10\
    \x01\x12)\n\x10leading_comments\x18\x03\x20\x01(\tR\x0fleadingComments\
    \x12+\n\x11trailing_comments\x18\x04\x20\x01(\tR\x10trailingComments\x12\
    :\n\x19leading_detached_comments\x18\x06\x20\x03(\tR\x17leadingDetachedC\
    omments\"\xd1\x01\n\x11GeneratedCodeInfo\x12M\n\nannotation\x18\x01\x20\
    \x03(\x0b2-.google.protobuf.GeneratedCodeInfo.AnnotationR\nannotation\
    \x1am\n\nAnnotation\x12\x16\n\x04path\x18\x01\x20\x03(\x05R\x04pathB\x02\
    \x10\x01\x12\x1f\n\x0bsource_file\x18\x02\x20\x01(\tR\nsourceFile\x12\
    \x14\n\x05begin\x18\x03\x20\x01(\x05R\x05begin\x12\x10\n\x03end\x18\x04\
    \x20\x01(\x05R\x03endBX\n\x13com.google.protobufB\x10DescriptorProtosH\
    \x01Z\ndescriptor\xa2\x02\x03GPB\xaa\x02\x1aGoogle.Protobuf.ReflectionJ\
    \xfe\xa4\x02\n\x07\x12\x05'\0\xa3\x06\x01\n\xaa\x0f\n\x01\x0c\x12\x03'\0\
    \x122\xc1\x0c\x20Protocol\x20Buffers\x20-\x20Google's\x20data\x20interch\
    ange\x20format\n\x20Copyright\x202008\x20Google\x20Inc.\x20\x20All\x20ri\
    ghts\x20reserved.\n\x20https://developers.google.com/protocol-buffers/\n\
    \n\x20Redistribution\x20and\x20use\x20in\x20source\x20and\x20binary\x20f\
    orms,\x20with\x20or\x20without\n\x20modification,\x20are\x20permitted\
    \x20provided\x20that\x20the\x20following\x20conditions\x20are\n\x20met:\
    \n\n\x20\x20\x20\x20\x20*\x20Redistributions\x20of\x20source\x20code\x20\
    must\x20retain\x20the\x20above\x20copyright\n\x20notice,\x20this\x20list\
    \x20of\x20conditions\x20and\x20the\x20following\x20disclaimer.\n\x20\x20\
    \x20\x20\x20*\x20Redistributions\x20in\x20binary\x20form\x20must\x20repr\
    oduce\x20the\x20above\n\x20copyright\x20notice,\x20this\x20list\x20of\
    \x20conditions\x20and\x20the\x20following\x20disclaimer\n\x20in\x20the\
    \x20documentation\x20and/or\x20other\x20materials\x20provided\x20with\
    \x20the\n\x20distribution.\n\x20\x20\x20\x20\x20*\x20Neither\x20the\x20n\
    ame\x20of\x20Google\x20Inc.\x20nor\x20the\x20names\x20of\x20its\n\x20con\
    tributors\x20may\x20be\x20used\x20to\x20endorse\x20or\x20promote\x20prod\
    ucts\x20derived\x20from\n\x20this\x20software\x20without\x20specific\x20\
    prior\x20written\x20permission.\n\n\x20THIS\x20SOFTWARE\x20IS\x20PROVIDE\
    D\x20BY\x20THE\x20COPYRIGHT\x20HOLDERS\x20AND\x20CONTRIBUTORS\n\x20\"AS\
    \x20IS\"\x20AND\x20ANY\x20EXPRESS\x20OR\x20IMPLIED\x20WARRANTIES,\x20INC\
    LUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20THE\x20IMPLIED\x20WARRANTIE\
    S\x20OF\x20MERCHANTABILITY\x20AND\x20FITNESS\x20FOR\n\x20A\x20PARTICULAR\
    \x20PURPOSE\x20ARE\x20DISCLAIMED.\x20IN\x20NO\x20EVENT\x20SHALL\x20THE\
    \x20COPYRIGHT\n\x20OWNER\x20OR\x20CONTRIBUTORS\x20BE\x20LIABLE\x20FOR\
    \x20ANY\x20DIRECT,\x20INDIRECT,\x20INCIDENTAL,\n\x20SPECIAL,\x20EXEMPLAR\
    Y,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\x20(INCLUDING,\x20BUT\x20NOT\n\x20L\
    IMITED\x20TO,\x20PROCUREMENT\x20OF\x20SUBSTITUTE\x20GOODS\x20OR\x20SERVI\
    CES;\x20LOSS\x20OF\x20USE,\n\x20DATA,\x20OR\x20PROFITS;\x20OR\x20BUSINES\
    S\x20INTERRUPTION)\x20HOWEVER\x20CAUSED\x20AND\x20ON\x20ANY\n\x20THEORY\
    \x20OF\x20LIABILITY,\x20WHETHER\x20IN\x20CONTRACT,\x20STRICT\x20LIABILIT\
    Y,\x20OR\x20TORT\n\x20(INCLUDING\x20NEGLIGENCE\x20OR\x20OTHERWISE)\x20AR\
    ISING\x20IN\x20ANY\x20WAY\x20OUT\x20OF\x20THE\x20USE\n\x20OF\x20THIS\x20\
    SOFTWARE,\x20EVEN\x20IF\x20ADVISED\x20OF\x20THE\x20POSSIBILITY\x20OF\x20\
    SUCH\x20DAMAGE.\n2\xdb\x02\x20Author:\x20kenton@google.com\x20(Kenton\
    \x20Varda)\n\x20\x20Based\x20on\x20original\x20Protocol\x20Buffers\x20de\
    sign\x20by\n\x20\x20Sanjay\x20Ghemawat,\x20Jeff\x20Dean,\x20and\x20other\
    s.\n\n\x20The\x20messages\x20in\x20this\x20file\x20describe\x20the\x20de\
    finitions\x20found\x20in\x20.proto\x20files.\n\x20A\x20valid\x20.proto\
    \x20file\x20can\x20be\x20translated\x20directly\x20to\x20a\x20FileDescri\
    ptorProto\n\x20without\x20any\x20other\x20information\x20(e.g.\x20withou\
    t\x20reading\x20its\x20imports).\n\n\x08\n\x01\x02\x12\x03)\x08\x17\n\
    \x08\n\x01\x08\x12\x03*\0!\n\x0b\n\x04\x08\xe7\x07\0\x12\x03*\0!\n\x0c\n\
    \x05\x08\xe7\x07\0\x02\x12\x03*\x07\x11\n\r\n\x06\x08\xe7\x07\0\x02\0\
    \x12\x03*\x07\x11\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03*\x07\x11\n\
    \x0c\n\x05\x08\xe7\x07\0\x07\x12\x03*\x14\x20\n\x08\n\x01\x08\x12\x03+\0\
    ,\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03+\0,\n\x0c\n\x05\x08\xe7\x07\x01\
    \x02\x12\x03+\x07\x13\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03+\x07\x13\n\
    \x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03+\x07\x13\n\x0c\n\x05\x08\
    \xe7\x07\x01\x07\x12\x03+\x16+\n\x08\n\x01\x08\x12\x03,\01\n\x0b\n\x04\
    \x08\xe7\x07\x02\x12\x03,\01\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03,\
    \x07\x1b\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03,\x07\x1b\n\x0e\n\x07\
    \x08\xe7\x07\x02\x02\0\x01\x12\x03,\x07\x1b\n\x0c\n\x05\x08\xe7\x07\x02\
    \x07\x12\x03,\x1e0\n\x08\n\x01\x08\x12\x03-\07\n\x0b\n\x04\x08\xe7\x07\
    \x03\x12\x03-\07\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03-\x07\x17\n\r\n\
    \x06\x08\xe7\x07\x03\x02\0\x12\x03-\x07\x17\n\x0e\n\x07\x08\xe7\x07\x03\
    \x02\0\x01\x12\x03-\x07\x17\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03-\x1a\
    6\n\x08\n\x01\x08\x12\x03.\0!\n\x0b\n\x04\x08\xe7\x07\x04\x12\x03.\0!\n\
    \x0c\n\x05\x08\xe7\x07\x04\x02\x12\x03.\x07\x18\n\r\n\x06\x08\xe7\x07\
    \x04\x02\0\x12\x03.\x07\x18\n\x0e\n\x07\x08\xe7\x07\x04\x02\0\x01\x12\
    \x03.\x07\x18\n\x0c\n\x05\x08\xe7\x07\x04\x07\x12\x03.\x1b\x20\n\x08\n\
    \x01\x08\x12\x032\0\x1c\n\x81\x01\n\x04\x08\xe7\x07\x05\x12\x032\0\x1c\
    \x1at\x20descriptor.proto\x20must\x20be\x20optimized\x20for\x20speed\x20\
    because\x20reflection-based\n\x20algorithms\x20don't\x20work\x20during\
    \x20bootstrapping.\n\n\x0c\n\x05\x08\xe7\x07\x05\x02\x12\x032\x07\x13\n\
    \r\n\x06\x08\xe7\x07\x05\x02\0\x12\x032\x07\x13\n\x0e\n\x07\x08\xe7\x07\
    \x05\x02\0\x01\x12\x032\x07\x13\n\x0c\n\x05\x08\xe7\x07\x05\x03\x12\x032\
    \x16\x1b\nj\n\x02\x04\0\x12\x046\08\x01\x1a^\x20The\x20protocol\x20compi\
    ler\x20can\x20output\x20a\x20FileDescriptorSet\x20containing\x20the\x20.\
    proto\n\x20files\x20it\x20parses.\n\n\n\n\x03\x04\0\x01\x12\x036\x08\x19\
    \n\x0b\n\x04\x04\0\x02\0\x12\x037\x02(\n\x0c\n\x05\x04\0\x02\0\x04\x12\
    \x037\x02\n\n\x0c\n\x05\x04\0\x02\0\x06\x12\x037\x0b\x1e\n\x0c\n\x05\x04\
    \0\x02\0\x01\x12\x037\x1f#\n\x0c\n\x05\x04\0\x02\0\x03\x12\x037&'\n/\n\
    \x02\x04\x01\x12\x04;\0X\x01\x1a#\x20Describes\x20a\x20complete\x20.prot\
    o\x20file.\n\n\n\n\x03\x04\x01\x01\x12\x03;\x08\x1b\n9\n\x04\x04\x01\x02\
    \0\x12\x03<\x02\x1b\",\x20file\x20name,\x20relative\x20to\x20root\x20of\
    \x20source\x20tree\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03<\x02\n\n\x0c\
    \n\x05\x04\x01\x02\0\x05\x12\x03<\x0b\x11\n\x0c\n\x05\x04\x01\x02\0\x01\
    \x12\x03<\x12\x16\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03<\x19\x1a\n*\n\
    \x04\x04\x01\x02\x01\x12\x03=\x02\x1e\"\x1d\x20e.g.\x20\"foo\",\x20\"foo\
    .bar\",\x20etc.\n\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03=\x02\n\n\x0c\n\
    \x05\x04\x01\x02\x01\x05\x12\x03=\x0b\x11\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03=\x12\x19\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03=\x1c\x1d\n\
    4\n\x04\x04\x01\x02\x02\x12\x03@\x02!\x1a'\x20Names\x20of\x20files\x20im\
    ported\x20by\x20this\x20file.\n\n\x0c\n\x05\x04\x01\x02\x02\x04\x12\x03@\
    \x02\n\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03@\x0b\x11\n\x0c\n\x05\x04\
    \x01\x02\x02\x01\x12\x03@\x12\x1c\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\
    \x03@\x1f\x20\nQ\n\x04\x04\x01\x02\x03\x12\x03B\x02(\x1aD\x20Indexes\x20\
    of\x20the\x20public\x20imported\x20files\x20in\x20the\x20dependency\x20l\
    ist\x20above.\n\n\x0c\n\x05\x04\x01\x02\x03\x04\x12\x03B\x02\n\n\x0c\n\
    \x05\x04\x01\x02\x03\x05\x12\x03B\x0b\x10\n\x0c\n\x05\x04\x01\x02\x03\
    \x01\x12\x03B\x11\"\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03B%'\nz\n\x04\
    \x04\x01\x02\x04\x12\x03E\x02&\x1am\x20Indexes\x20of\x20the\x20weak\x20i\
    mported\x20files\x20in\x20the\x20dependency\x20list.\n\x20For\x20Google-\
    internal\x20migration\x20only.\x20Do\x20not\x20use.\n\n\x0c\n\x05\x04\
    \x01\x02\x04\x04\x12\x03E\x02\n\n\x0c\n\x05\x04\x01\x02\x04\x05\x12\x03E\
    \x0b\x10\n\x0c\n\x05\x04\x01\x02\x04\x01\x12\x03E\x11\x20\n\x0c\n\x05\
    \x04\x01\x02\x04\x03\x12\x03E#%\n6\n\x04\x04\x01\x02\x05\x12\x03H\x02,\
    \x1a)\x20All\x20top-level\x20definitions\x20in\x20this\x20file.\n\n\x0c\
    \n\x05\x04\x01\x02\x05\x04\x12\x03H\x02\n\n\x0c\n\x05\x04\x01\x02\x05\
    \x06\x12\x03H\x0b\x1a\n\x0c\n\x05\x04\x01\x02\x05\x01\x12\x03H\x1b'\n\
    \x0c\n\x05\x04\x01\x02\x05\x03\x12\x03H*+\n\x0b\n\x04\x04\x01\x02\x06\
    \x12\x03I\x02-\n\x0c\n\x05\x04\x01\x02\x06\x04\x12\x03I\x02\n\n\x0c\n\
    \x05\x04\x01\x02\x06\x06\x12\x03I\x0b\x1e\n\x0c\n\x05\x04\x01\x02\x06\
    \x01\x12\x03I\x1f(\n\x0c\n\x05\x04\x01\x02\x06\x03\x12\x03I+,\n\x0b\n\
    \x04\x04\x01\x02\x07\x12\x03J\x02.\n\x0c\n\x05\x04\x01\x02\x07\x04\x12\
    \x03J\x02\n\n\x0c\n\x05\x04\x01\x02\x07\x06\x12\x03J\x0b!\n\x0c\n\x05\
    \x04\x01\x02\x07\x01\x12\x03J\")\n\x0c\n\x05\x04\x01\x02\x07\x03\x12\x03\
    J,-\n\x0b\n\x04\x04\x01\x02\x08\x12\x03K\x02.\n\x0c\n\x05\x04\x01\x02\
    \x08\x04\x12\x03K\x02\n\n\x0c\n\x05\x04\x01\x02\x08\x06\x12\x03K\x0b\x1f\
    \n\x0c\n\x05\x04\x01\x02\x08\x01\x12\x03K\x20)\n\x0c\n\x05\x04\x01\x02\
    \x08\x03\x12\x03K,-\n\x0b\n\x04\x04\x01\x02\t\x12\x03M\x02#\n\x0c\n\x05\
    \x04\x01\x02\t\x04\x12\x03M\x02\n\n\x0c\n\x05\x04\x01\x02\t\x06\x12\x03M\
    \x0b\x16\n\x0c\n\x05\x04\x01\x02\t\x01\x12\x03M\x17\x1e\n\x0c\n\x05\x04\
    \x01\x02\t\x03\x12\x03M!\"\n\xf4\x01\n\x04\x04\x01\x02\n\x12\x03S\x02/\
    \x1a\xe6\x01\x20This\x20field\x20contains\x20optional\x20information\x20\
    about\x20the\x20original\x20source\x20code.\n\x20You\x20may\x20safely\
    \x20remove\x20this\x20entire\x20field\x20without\x20harming\x20runtime\n\
    \x20functionality\x20of\x20the\x20descriptors\x20--\x20the\x20informatio\
    n\x20is\x20needed\x20only\x20by\n\x20development\x20tools.\n\n\x0c\n\x05\
    \x04\x01\x02\n\x04\x12\x03S\x02\n\n\x0c\n\x05\x04\x01\x02\n\x06\x12\x03S\
    \x0b\x19\n\x0c\n\x05\x04\x01\x02\n\x01\x12\x03S\x1a*\n\x0c\n\x05\x04\x01\
    \x02\n\x03\x12\x03S-.\n]\n\x04\x04\x01\x02\x0b\x12\x03W\x02\x1e\x1aP\x20\
    The\x20syntax\x20of\x20the\x20proto\x20file.\n\x20The\x20supported\x20va\
    lues\x20are\x20\"proto2\"\x20and\x20\"proto3\".\n\n\x0c\n\x05\x04\x01\
    \x02\x0b\x04\x12\x03W\x02\n\n\x0c\n\x05\x04\x01\x02\x0b\x05\x12\x03W\x0b\
    \x11\n\x0c\n\x05\x04\x01\x02\x0b\x01\x12\x03W\x12\x18\n\x0c\n\x05\x04\
    \x01\x02\x0b\x03\x12\x03W\x1b\x1d\n'\n\x02\x04\x02\x12\x04[\0y\x01\x1a\
    \x1b\x20Describes\x20a\x20message\x20type.\n\n\n\n\x03\x04\x02\x01\x12\
    \x03[\x08\x17\n\x0b\n\x04\x04\x02\x02\0\x12\x03\\\x02\x1b\n\x0c\n\x05\
    \x04\x02\x02\0\x04\x12\x03\\\x02\n\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\
    \\\x0b\x11\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\\\x12\x16\n\x0c\n\x05\
    \x04\x02\x02\0\x03\x12\x03\\\x19\x1a\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\
    ^\x02*\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03^\x02\n\n\x0c\n\x05\x04\
    \x02\x02\x01\x06\x12\x03^\x0b\x1f\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\
    \x03^\x20%\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03^()\n\x0b\n\x04\x04\
    \x02\x02\x02\x12\x03_\x02.\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03_\x02\
    \n\n\x0c\n\x05\x04\x02\x02\x02\x06\x12\x03_\x0b\x1f\n\x0c\n\x05\x04\x02\
    \x02\x02\x01\x12\x03_\x20)\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03_,-\n\
    \x0b\n\x04\x04\x02\x02\x03\x12\x03a\x02+\n\x0c\n\x05\x04\x02\x02\x03\x04\
    \x12\x03a\x02\n\n\x0c\n\x05\x04\x02\x02\x03\x06\x12\x03a\x0b\x1a\n\x0c\n\
    \x05\x04\x02\x02\x03\x01\x12\x03a\x1b&\n\x0c\n\x05\x04\x02\x02\x03\x03\
    \x12\x03a)*\n\x0b\n\x04\x04\x02\x02\x04\x12\x03b\x02-\n\x0c\n\x05\x04\
    \x02\x02\x04\x04\x12\x03b\x02\n\n\x0c\n\x05\x04\x02\x02\x04\x06\x12\x03b\
    \x0b\x1e\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03b\x1f(\n\x0c\n\x05\x04\
    \x02\x02\x04\x03\x12\x03b+,\n\x0c\n\x04\x04\x02\x03\0\x12\x04d\x02g\x03\
    \n\x0c\n\x05\x04\x02\x03\0\x01\x12\x03d\n\x18\n\r\n\x06\x04\x02\x03\0\
    \x02\0\x12\x03e\x04\x1d\n\x0e\n\x07\x04\x02\x03\0\x02\0\x04\x12\x03e\x04\
    \x0c\n\x0e\n\x07\x04\x02\x03\0\x02\0\x05\x12\x03e\r\x12\n\x0e\n\x07\x04\
    \x02\x03\0\x02\0\x01\x12\x03e\x13\x18\n\x0e\n\x07\x04\x02\x03\0\x02\0\
    \x03\x12\x03e\x1b\x1c\n\r\n\x06\x04\x02\x03\0\x02\x01\x12\x03f\x04\x1b\n\
    \x0e\n\x07\x04\x02\x03\0\x02\x01\x04\x12\x03f\x04\x0c\n\x0e\n\x07\x04\
    \x02\x03\0\x02\x01\x05\x12\x03f\r\x12\n\x0e\n\x07\x04\x02\x03\0\x02\x01\
    \x01\x12\x03f\x13\x16\n\x0e\n\x07\x04\x02\x03\0\x02\x01\x03\x12\x03f\x19\
    \x1a\n\x0b\n\x04\x04\x02\x02\x05\x12\x03h\x02.\n\x0c\n\x05\x04\x02\x02\
    \x05\x04\x12\x03h\x02\n\n\x0c\n\x05\x04\x02\x02\x05\x06\x12\x03h\x0b\x19\
    \n\x0c\n\x05\x04\x02\x02\x05\x01\x12\x03h\x1a)\n\x0c\n\x05\x04\x02\x02\
    \x05\x03\x12\x03h,-\n\x0b\n\x04\x04\x02\x02\x06\x12\x03j\x02/\n\x0c\n\
    \x05\x04\x02\x02\x06\x04\x12\x03j\x02\n\n\x0c\n\x05\x04\x02\x02\x06\x06\
    \x12\x03j\x0b\x1f\n\x0c\n\x05\x04\x02\x02\x06\x01\x12\x03j\x20*\n\x0c\n\
    \x05\x04\x02\x02\x06\x03\x12\x03j-.\n\x0b\n\x04\x04\x02\x02\x07\x12\x03l\
    \x02&\n\x0c\n\x05\x04\x02\x02\x07\x04\x12\x03l\x02\n\n\x0c\n\x05\x04\x02\
    \x02\x07\x06\x12\x03l\x0b\x19\n\x0c\n\x05\x04\x02\x02\x07\x01\x12\x03l\
    \x1a!\n\x0c\n\x05\x04\x02\x02\x07\x03\x12\x03l$%\n\xaa\x01\n\x04\x04\x02\
    \x03\x01\x12\x04q\x02t\x03\x1a\x9b\x01\x20Range\x20of\x20reserved\x20tag\
    \x20numbers.\x20Reserved\x20tag\x20numbers\x20may\x20not\x20be\x20used\
    \x20by\n\x20fields\x20or\x20extension\x20ranges\x20in\x20the\x20same\x20\
    message.\x20Reserved\x20ranges\x20may\n\x20not\x20overlap.\n\n\x0c\n\x05\
    \x04\x02\x03\x01\x01\x12\x03q\n\x17\n\x1b\n\x06\x04\x02\x03\x01\x02\0\
    \x12\x03r\x04\x1d\"\x0c\x20Inclusive.\n\n\x0e\n\x07\x04\x02\x03\x01\x02\
    \0\x04\x12\x03r\x04\x0c\n\x0e\n\x07\x04\x02\x03\x01\x02\0\x05\x12\x03r\r\
    \x12\n\x0e\n\x07\x04\x02\x03\x01\x02\0\x01\x12\x03r\x13\x18\n\x0e\n\x07\
    \x04\x02\x03\x01\x02\0\x03\x12\x03r\x1b\x1c\n\x1b\n\x06\x04\x02\x03\x01\
    \x02\x01\x12\x03s\x04\x1b\"\x0c\x20Exclusive.\n\n\x0e\n\x07\x04\x02\x03\
    \x01\x02\x01\x04\x12\x03s\x04\x0c\n\x0e\n\x07\x04\x02\x03\x01\x02\x01\
    \x05\x12\x03s\r\x12\n\x0e\n\x07\x04\x02\x03\x01\x02\x01\x01\x12\x03s\x13\
    \x16\n\x0e\n\x07\x04\x02\x03\x01\x02\x01\x03\x12\x03s\x19\x1a\n\x0b\n\
    \x04\x04\x02\x02\x08\x12\x03u\x02,\n\x0c\n\x05\x04\x02\x02\x08\x04\x12\
    \x03u\x02\n\n\x0c\n\x05\x04\x02\x02\x08\x06\x12\x03u\x0b\x18\n\x0c\n\x05\
    \x04\x02\x02\x08\x01\x12\x03u\x19'\n\x0c\n\x05\x04\x02\x02\x08\x03\x12\
    \x03u*+\n\x82\x01\n\x04\x04\x02\x02\t\x12\x03x\x02%\x1au\x20Reserved\x20\
    field\x20names,\x20which\x20may\x20not\x20be\x20used\x20by\x20fields\x20\
    in\x20the\x20same\x20message.\n\x20A\x20given\x20name\x20may\x20only\x20\
    be\x20reserved\x20once.\n\n\x0c\n\x05\x04\x02\x02\t\x04\x12\x03x\x02\n\n\
    \x0c\n\x05\x04\x02\x02\t\x05\x12\x03x\x0b\x11\n\x0c\n\x05\x04\x02\x02\t\
    \x01\x12\x03x\x12\x1f\n\x0c\n\x05\x04\x02\x02\t\x03\x12\x03x\"$\n2\n\x02\
    \x04\x03\x12\x05|\0\xc7\x01\x01\x1a%\x20Describes\x20a\x20field\x20withi\
    n\x20a\x20message.\n\n\n\n\x03\x04\x03\x01\x12\x03|\x08\x1c\n\r\n\x04\
    \x04\x03\x04\0\x12\x05}\x02\x98\x01\x03\n\x0c\n\x05\x04\x03\x04\0\x01\
    \x12\x03}\x07\x0b\nS\n\x06\x04\x03\x04\0\x02\0\x12\x04\x80\x01\x04\x1c\
    \x1aC\x200\x20is\x20reserved\x20for\x20errors.\n\x20Order\x20is\x20weird\
    \x20for\x20historical\x20reasons.\n\n\x0f\n\x07\x04\x03\x04\0\x02\0\x01\
    \x12\x04\x80\x01\x04\x0f\n\x0f\n\x07\x04\x03\x04\0\x02\0\x02\x12\x04\x80\
    \x01\x1a\x1b\n\x0e\n\x06\x04\x03\x04\0\x02\x01\x12\x04\x81\x01\x04\x1c\n\
    \x0f\n\x07\x04\x03\x04\0\x02\x01\x01\x12\x04\x81\x01\x04\x0e\n\x0f\n\x07\
    \x04\x03\x04\0\x02\x01\x02\x12\x04\x81\x01\x1a\x1b\nw\n\x06\x04\x03\x04\
    \0\x02\x02\x12\x04\x84\x01\x04\x1c\x1ag\x20Not\x20ZigZag\x20encoded.\x20\
    \x20Negative\x20numbers\x20take\x2010\x20bytes.\x20\x20Use\x20TYPE_SINT6\
    4\x20if\n\x20negative\x20values\x20are\x20likely.\n\n\x0f\n\x07\x04\x03\
    \x04\0\x02\x02\x01\x12\x04\x84\x01\x04\x0e\n\x0f\n\x07\x04\x03\x04\0\x02\
    \x02\x02\x12\x04\x84\x01\x1a\x1b\n\x0e\n\x06\x04\x03\x04\0\x02\x03\x12\
    \x04\x85\x01\x04\x1c\n\x0f\n\x07\x04\x03\x04\0\x02\x03\x01\x12\x04\x85\
    \x01\x04\x0f\n\x0f\n\x07\x04\x03\x04\0\x02\x03\x02\x12\x04\x85\x01\x1a\
    \x1b\nw\n\x06\x04\x03\x04\0\x02\x04\x12\x04\x88\x01\x04\x1c\x1ag\x20Not\
    \x20ZigZag\x20encoded.\x20\x20Negative\x20numbers\x20take\x2010\x20bytes\
    .\x20\x20Use\x20TYPE_SINT32\x20if\n\x20negative\x20values\x20are\x20like\
    ly.\n\n\x0f\n\x07\x04\x03\x04\0\x02\x04\x01\x12\x04\x88\x01\x04\x0e\n\
    \x0f\n\x07\x04\x03\x04\0\x02\x04\x02\x12\x04\x88\x01\x1a\x1b\n\x0e\n\x06\
    \x04\x03\x04\0\x02\x05\x12\x04\x89\x01\x04\x1c\n\x0f\n\x07\x04\x03\x04\0\
    \x02\x05\x01\x12\x04\x89\x01\x04\x10\n\x0f\n\x07\x04\x03\x04\0\x02\x05\
    \x02\x12\x04\x89\x01\x1a\x1b\n\x0e\n\x06\x04\x03\x04\0\x02\x06\x12\x04\
    \x8a\x01\x04\x1c\n\x0f\n\x07\x04\x03\x04\0\x02\x06\x01\x12\x04\x8a\x01\
    \x04\x10\n\x0f\n\x07\x04\x03\x04\0\x02\x06\x02\x12\x04\x8a\x01\x1a\x1b\n\
    \x0e\n\x06\x04\x03\x04\0\x02\x07\x12\x04\x8b\x01\x04\x1c\n\x0f\n\x07\x04\
    \x03\x04\0\x02\x07\x01\x12\x04\x8b\x01\x04\r\n\x0f\n\x07\x04\x03\x04\0\
    \x02\x07\x02\x12\x04\x8b\x01\x1a\x1b\n\x0e\n\x06\x04\x03\x04\0\x02\x08\
    \x12\x04\x8c\x01\x04\x1c\n\x0f\n\x07\x04\x03\x04\0\x02\x08\x01\x12\x04\
    \x8c\x01\x04\x0f\n\x0f\n\x07\x04\x03\x04\0\x02\x08\x02\x12\x04\x8c\x01\
    \x1a\x1b\n*\n\x06\x04\x03\x04\0\x02\t\x12\x04\x8d\x01\x04\x1d\"\x1a\x20T\
    ag-delimited\x20aggregate.\n\n\x0f\n\x07\x04\x03\x04\0\x02\t\x01\x12\x04\
    \x8d\x01\x04\x0e\n\x0f\n\x07\x04\x03\x04\0\x02\t\x02\x12\x04\x8d\x01\x1a\
    \x1c\n-\n\x06\x04\x03\x04\0\x02\n\x12\x04\x8e\x01\x04\x1d\"\x1d\x20Lengt\
    h-delimited\x20aggregate.\n\n\x0f\n\x07\x04\x03\x04\0\x02\n\x01\x12\x04\
    \x8e\x01\x04\x10\n\x0f\n\x07\x04\x03\x04\0\x02\n\x02\x12\x04\x8e\x01\x1a\
    \x1c\n#\n\x06\x04\x03\x04\0\x02\x0b\x12\x04\x91\x01\x04\x1d\x1a\x13\x20N\
    ew\x20in\x20version\x202.\n\n\x0f\n\x07\x04\x03\x04\0\x02\x0b\x01\x12\
    \x04\x91\x01\x04\x0e\n\x0f\n\x07\x04\x03\x04\0\x02\x0b\x02\x12\x04\x91\
    \x01\x1a\x1c\n\x0e\n\x06\x04\x03\x04\0\x02\x0c\x12\x04\x92\x01\x04\x1d\n\
    \x0f\n\x07\x04\x03\x04\0\x02\x0c\x01\x12\x04\x92\x01\x04\x0f\n\x0f\n\x07\
    \x04\x03\x04\0\x02\x0c\x02\x12\x04\x92\x01\x1a\x1c\n\x0e\n\x06\x04\x03\
    \x04\0\x02\r\x12\x04\x93\x01\x04\x1d\n\x0f\n\x07\x04\x03\x04\0\x02\r\x01\
    \x12\x04\x93\x01\x04\r\n\x0f\n\x07\x04\x03\x04\0\x02\r\x02\x12\x04\x93\
    \x01\x1a\x1c\n\x0e\n\x06\x04\x03\x04\0\x02\x0e\x12\x04\x94\x01\x04\x1d\n\
    \x0f\n\x07\x04\x03\x04\0\x02\x0e\x01\x12\x04\x94\x01\x04\x11\n\x0f\n\x07\
    \x04\x03\x04\0\x02\x0e\x02\x12\x04\x94\x01\x1a\x1c\n\x0e\n\x06\x04\x03\
    \x04\0\x02\x0f\x12\x04\x95\x01\x04\x1d\n\x0f\n\x07\x04\x03\x04\0\x02\x0f\
    \x01\x12\x04\x95\x01\x04\x11\n\x0f\n\x07\x04\x03\x04\0\x02\x0f\x02\x12\
    \x04\x95\x01\x1a\x1c\n'\n\x06\x04\x03\x04\0\x02\x10\x12\x04\x96\x01\x04\
    \x1d\"\x17\x20Uses\x20ZigZag\x20encoding.\n\n\x0f\n\x07\x04\x03\x04\0\
    \x02\x10\x01\x12\x04\x96\x01\x04\x0f\n\x0f\n\x07\x04\x03\x04\0\x02\x10\
    \x02\x12\x04\x96\x01\x1a\x1c\n'\n\x06\x04\x03\x04\0\x02\x11\x12\x04\x97\
    \x01\x04\x1d\"\x17\x20Uses\x20ZigZag\x20encoding.\n\n\x0f\n\x07\x04\x03\
    \x04\0\x02\x11\x01\x12\x04\x97\x01\x04\x0f\n\x0f\n\x07\x04\x03\x04\0\x02\
    \x11\x02\x12\x04\x97\x01\x1a\x1c\n\x0e\n\x04\x04\x03\x04\x01\x12\x06\x9a\
    \x01\x02\xa0\x01\x03\n\r\n\x05\x04\x03\x04\x01\x01\x12\x04\x9a\x01\x07\
    \x0c\n*\n\x06\x04\x03\x04\x01\x02\0\x12\x04\x9c\x01\x04\x1c\x1a\x1a\x200\
    \x20is\x20reserved\x20for\x20errors\n\n\x0f\n\x07\x04\x03\x04\x01\x02\0\
    \x01\x12\x04\x9c\x01\x04\x12\n\x0f\n\x07\x04\x03\x04\x01\x02\0\x02\x12\
    \x04\x9c\x01\x1a\x1b\n\x0e\n\x06\x04\x03\x04\x01\x02\x01\x12\x04\x9d\x01\
    \x04\x1c\n\x0f\n\x07\x04\x03\x04\x01\x02\x01\x01\x12\x04\x9d\x01\x04\x12\
    \n\x0f\n\x07\x04\x03\x04\x01\x02\x01\x02\x12\x04\x9d\x01\x1a\x1b\n8\n\
    \x06\x04\x03\x04\x01\x02\x02\x12\x04\x9e\x01\x04\x1c\"(\x20TODO(sanjay):\
    \x20Should\x20we\x20add\x20LABEL_MAP?\n\n\x0f\n\x07\x04\x03\x04\x01\x02\
    \x02\x01\x12\x04\x9e\x01\x04\x12\n\x0f\n\x07\x04\x03\x04\x01\x02\x02\x02\
    \x12\x04\x9e\x01\x1a\x1b\n\x0c\n\x04\x04\x03\x02\0\x12\x04\xa2\x01\x02\
    \x1b\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\xa2\x01\x02\n\n\r\n\x05\x04\x03\
    \x02\0\x05\x12\x04\xa2\x01\x0b\x11\n\r\n\x05\x04\x03\x02\0\x01\x12\x04\
    \xa2\x01\x12\x16\n\r\n\x05\x04\x03\x02\0\x03\x12\x04\xa2\x01\x19\x1a\n\
    \x0c\n\x04\x04\x03\x02\x01\x12\x04\xa3\x01\x02\x1c\n\r\n\x05\x04\x03\x02\
    \x01\x04\x12\x04\xa3\x01\x02\n\n\r\n\x05\x04\x03\x02\x01\x05\x12\x04\xa3\
    \x01\x0b\x10\n\r\n\x05\x04\x03\x02\x01\x01\x12\x04\xa3\x01\x11\x17\n\r\n\
    \x05\x04\x03\x02\x01\x03\x12\x04\xa3\x01\x1a\x1b\n\x0c\n\x04\x04\x03\x02\
    \x02\x12\x04\xa4\x01\x02\x1b\n\r\n\x05\x04\x03\x02\x02\x04\x12\x04\xa4\
    \x01\x02\n\n\r\n\x05\x04\x03\x02\x02\x06\x12\x04\xa4\x01\x0b\x10\n\r\n\
    \x05\x04\x03\x02\x02\x01\x12\x04\xa4\x01\x11\x16\n\r\n\x05\x04\x03\x02\
    \x02\x03\x12\x04\xa4\x01\x19\x1a\n\x9c\x01\n\x04\x04\x03\x02\x03\x12\x04\
    \xa8\x01\x02\x19\x1a\x8d\x01\x20If\x20type_name\x20is\x20set,\x20this\
    \x20need\x20not\x20be\x20set.\x20\x20If\x20both\x20this\x20and\x20type_n\
    ame\n\x20are\x20set,\x20this\x20must\x20be\x20one\x20of\x20TYPE_ENUM,\
    \x20TYPE_MESSAGE\x20or\x20TYPE_GROUP.\n\n\r\n\x05\x04\x03\x02\x03\x04\
    \x12\x04\xa8\x01\x02\n\n\r\n\x05\x04\x03\x02\x03\x06\x12\x04\xa8\x01\x0b\
    \x0f\n\r\n\x05\x04\x03\x02\x03\x01\x12\x04\xa8\x01\x10\x14\n\r\n\x05\x04\
    \x03\x02\x03\x03\x12\x04\xa8\x01\x17\x18\n\xb7\x02\n\x04\x04\x03\x02\x04\
    \x12\x04\xaf\x01\x02\x20\x1a\xa8\x02\x20For\x20message\x20and\x20enum\
    \x20types,\x20this\x20is\x20the\x20name\x20of\x20the\x20type.\x20\x20If\
    \x20the\x20name\n\x20starts\x20with\x20a\x20'.',\x20it\x20is\x20fully-qu\
    alified.\x20\x20Otherwise,\x20C++-like\x20scoping\n\x20rules\x20are\x20u\
    sed\x20to\x20find\x20the\x20type\x20(i.e.\x20first\x20the\x20nested\x20t\
    ypes\x20within\x20this\n\x20message\x20are\x20searched,\x20then\x20withi\
    n\x20the\x20parent,\x20on\x20up\x20to\x20the\x20root\n\x20namespace).\n\
    \n\r\n\x05\x04\x03\x02\x04\x04\x12\x04\xaf\x01\x02\n\n\r\n\x05\x04\x03\
    \x02\x04\x05\x12\x04\xaf\x01\x0b\x11\n\r\n\x05\x04\x03\x02\x04\x01\x12\
    \x04\xaf\x01\x12\x1b\n\r\n\x05\x04\x03\x02\x04\x03\x12\x04\xaf\x01\x1e\
    \x1f\n~\n\x04\x04\x03\x02\x05\x12\x04\xb3\x01\x02\x1f\x1ap\x20For\x20ext\
    ensions,\x20this\x20is\x20the\x20name\x20of\x20the\x20type\x20being\x20e\
    xtended.\x20\x20It\x20is\n\x20resolved\x20in\x20the\x20same\x20manner\
    \x20as\x20type_name.\n\n\r\n\x05\x04\x03\x02\x05\x04\x12\x04\xb3\x01\x02\
    \n\n\r\n\x05\x04\x03\x02\x05\x05\x12\x04\xb3\x01\x0b\x11\n\r\n\x05\x04\
    \x03\x02\x05\x01\x12\x04\xb3\x01\x12\x1a\n\r\n\x05\x04\x03\x02\x05\x03\
    \x12\x04\xb3\x01\x1d\x1e\n\xb1\x02\n\x04\x04\x03\x02\x06\x12\x04\xba\x01\
    \x02$\x1a\xa2\x02\x20For\x20numeric\x20types,\x20contains\x20the\x20orig\
    inal\x20text\x20representation\x20of\x20the\x20value.\n\x20For\x20boolea\
    ns,\x20\"true\"\x20or\x20\"false\".\n\x20For\x20strings,\x20contains\x20\
    the\x20default\x20text\x20contents\x20(not\x20escaped\x20in\x20any\x20wa\
    y).\n\x20For\x20bytes,\x20contains\x20the\x20C\x20escaped\x20value.\x20\
    \x20All\x20bytes\x20>=\x20128\x20are\x20escaped.\n\x20TODO(kenton):\x20\
    \x20Base-64\x20encode?\n\n\r\n\x05\x04\x03\x02\x06\x04\x12\x04\xba\x01\
    \x02\n\n\r\n\x05\x04\x03\x02\x06\x05\x12\x04\xba\x01\x0b\x11\n\r\n\x05\
    \x04\x03\x02\x06\x01\x12\x04\xba\x01\x12\x1f\n\r\n\x05\x04\x03\x02\x06\
    \x03\x12\x04\xba\x01\"#\n\x84\x01\n\x04\x04\x03\x02\x07\x12\x04\xbe\x01\
    \x02!\x1av\x20If\x20set,\x20gives\x20the\x20index\x20of\x20a\x20oneof\
    \x20in\x20the\x20containing\x20type's\x20oneof_decl\n\x20list.\x20\x20Th\
    is\x20field\x20is\x20a\x20member\x20of\x20that\x20oneof.\n\n\r\n\x05\x04\
    \x03\x02\x07\x04\x12\x04\xbe\x01\x02\n\n\r\n\x05\x04\x03\x02\x07\x05\x12\
    \x04\xbe\x01\x0b\x10\n\r\n\x05\x04\x03\x02\x07\x01\x12\x04\xbe\x01\x11\
    \x1c\n\r\n\x05\x04\x03\x02\x07\x03\x12\x04\xbe\x01\x1f\x20\n\xfa\x01\n\
    \x04\x04\x03\x02\x08\x12\x04\xc4\x01\x02!\x1a\xeb\x01\x20JSON\x20name\
    \x20of\x20this\x20field.\x20The\x20value\x20is\x20set\x20by\x20protocol\
    \x20compiler.\x20If\x20the\n\x20user\x20has\x20set\x20a\x20\"json_name\"\
    \x20option\x20on\x20this\x20field,\x20that\x20option's\x20value\n\x20wil\
    l\x20be\x20used.\x20Otherwise,\x20it's\x20deduced\x20from\x20the\x20fiel\
    d's\x20name\x20by\x20converting\n\x20it\x20to\x20camelCase.\n\n\r\n\x05\
    \x04\x03\x02\x08\x04\x12\x04\xc4\x01\x02\n\n\r\n\x05\x04\x03\x02\x08\x05\
    \x12\x04\xc4\x01\x0b\x11\n\r\n\x05\x04\x03\x02\x08\x01\x12\x04\xc4\x01\
    \x12\x1b\n\r\n\x05\x04\x03\x02\x08\x03\x12\x04\xc4\x01\x1e\x20\n\x0c\n\
    \x04\x04\x03\x02\t\x12\x04\xc6\x01\x02$\n\r\n\x05\x04\x03\x02\t\x04\x12\
    \x04\xc6\x01\x02\n\n\r\n\x05\x04\x03\x02\t\x06\x12\x04\xc6\x01\x0b\x17\n\
    \r\n\x05\x04\x03\x02\t\x01\x12\x04\xc6\x01\x18\x1f\n\r\n\x05\x04\x03\x02\
    \t\x03\x12\x04\xc6\x01\"#\n\"\n\x02\x04\x04\x12\x06\xca\x01\0\xcd\x01\
    \x01\x1a\x14\x20Describes\x20a\x20oneof.\n\n\x0b\n\x03\x04\x04\x01\x12\
    \x04\xca\x01\x08\x1c\n\x0c\n\x04\x04\x04\x02\0\x12\x04\xcb\x01\x02\x1b\n\
    \r\n\x05\x04\x04\x02\0\x04\x12\x04\xcb\x01\x02\n\n\r\n\x05\x04\x04\x02\0\
    \x05\x12\x04\xcb\x01\x0b\x11\n\r\n\x05\x04\x04\x02\0\x01\x12\x04\xcb\x01\
    \x12\x16\n\r\n\x05\x04\x04\x02\0\x03\x12\x04\xcb\x01\x19\x1a\n\x0c\n\x04\
    \x04\x04\x02\x01\x12\x04\xcc\x01\x02$\n\r\n\x05\x04\x04\x02\x01\x04\x12\
    \x04\xcc\x01\x02\n\n\r\n\x05\x04\x04\x02\x01\x06\x12\x04\xcc\x01\x0b\x17\
    \n\r\n\x05\x04\x04\x02\x01\x01\x12\x04\xcc\x01\x18\x1f\n\r\n\x05\x04\x04\
    \x02\x01\x03\x12\x04\xcc\x01\"#\n'\n\x02\x04\x05\x12\x06\xd0\x01\0\xd6\
    \x01\x01\x1a\x19\x20Describes\x20an\x20enum\x20type.\n\n\x0b\n\x03\x04\
    \x05\x01\x12\x04\xd0\x01\x08\x1b\n\x0c\n\x04\x04\x05\x02\0\x12\x04\xd1\
    \x01\x02\x1b\n\r\n\x05\x04\x05\x02\0\x04\x12\x04\xd1\x01\x02\n\n\r\n\x05\
    \x04\x05\x02\0\x05\x12\x04\xd1\x01\x0b\x11\n\r\n\x05\x04\x05\x02\0\x01\
    \x12\x04\xd1\x01\x12\x16\n\r\n\x05\x04\x05\x02\0\x03\x12\x04\xd1\x01\x19\
    \x1a\n\x0c\n\x04\x04\x05\x02\x01\x12\x04\xd3\x01\x02.\n\r\n\x05\x04\x05\
    \x02\x01\x04\x12\x04\xd3\x01\x02\n\n\r\n\x05\x04\x05\x02\x01\x06\x12\x04\
    \xd3\x01\x0b#\n\r\n\x05\x04\x05\x02\x01\x01\x12\x04\xd3\x01$)\n\r\n\x05\
    \x04\x05\x02\x01\x03\x12\x04\xd3\x01,-\n\x0c\n\x04\x04\x05\x02\x02\x12\
    \x04\xd5\x01\x02#\n\r\n\x05\x04\x05\x02\x02\x04\x12\x04\xd5\x01\x02\n\n\
    \r\n\x05\x04\x05\x02\x02\x06\x12\x04\xd5\x01\x0b\x16\n\r\n\x05\x04\x05\
    \x02\x02\x01\x12\x04\xd5\x01\x17\x1e\n\r\n\x05\x04\x05\x02\x02\x03\x12\
    \x04\xd5\x01!\"\n1\n\x02\x04\x06\x12\x06\xd9\x01\0\xde\x01\x01\x1a#\x20D\
    escribes\x20a\x20value\x20within\x20an\x20enum.\n\n\x0b\n\x03\x04\x06\
    \x01\x12\x04\xd9\x01\x08\x20\n\x0c\n\x04\x04\x06\x02\0\x12\x04\xda\x01\
    \x02\x1b\n\r\n\x05\x04\x06\x02\0\x04\x12\x04\xda\x01\x02\n\n\r\n\x05\x04\
    \x06\x02\0\x05\x12\x04\xda\x01\x0b\x11\n\r\n\x05\x04\x06\x02\0\x01\x12\
    \x04\xda\x01\x12\x16\n\r\n\x05\x04\x06\x02\0\x03\x12\x04\xda\x01\x19\x1a\
    \n\x0c\n\x04\x04\x06\x02\x01\x12\x04\xdb\x01\x02\x1c\n\r\n\x05\x04\x06\
    \x02\x01\x04\x12\x04\xdb\x01\x02\n\n\r\n\x05\x04\x06\x02\x01\x05\x12\x04\
    \xdb\x01\x0b\x10\n\r\n\x05\x04\x06\x02\x01\x01\x12\x04\xdb\x01\x11\x17\n\
    \r\n\x05\x04\x06\x02\x01\x03\x12\x04\xdb\x01\x1a\x1b\n\x0c\n\x04\x04\x06\
    \x02\x02\x12\x04\xdd\x01\x02(\n\r\n\x05\x04\x06\x02\x02\x04\x12\x04\xdd\
    \x01\x02\n\n\r\n\x05\x04\x06\x02\x02\x06\x12\x04\xdd\x01\x0b\x1b\n\r\n\
    \x05\x04\x06\x02\x02\x01\x12\x04\xdd\x01\x1c#\n\r\n\x05\x04\x06\x02\x02\
    \x03\x12\x04\xdd\x01&'\n$\n\x02\x04\x07\x12\x06\xe1\x01\0\xe6\x01\x01\
    \x1a\x16\x20Describes\x20a\x20service.\n\n\x0b\n\x03\x04\x07\x01\x12\x04\
    \xe1\x01\x08\x1e\n\x0c\n\x04\x04\x07\x02\0\x12\x04\xe2\x01\x02\x1b\n\r\n\
    \x05\x04\x07\x02\0\x04\x12\x04\xe2\x01\x02\n\n\r\n\x05\x04\x07\x02\0\x05\
    \x12\x04\xe2\x01\x0b\x11\n\r\n\x05\x04\x07\x02\0\x01\x12\x04\xe2\x01\x12\
    \x16\n\r\n\x05\x04\x07\x02\0\x03\x12\x04\xe2\x01\x19\x1a\n\x0c\n\x04\x04\
    \x07\x02\x01\x12\x04\xe3\x01\x02,\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04\
    \xe3\x01\x02\n\n\r\n\x05\x04\x07\x02\x01\x06\x12\x04\xe3\x01\x0b\x20\n\r\
    \n\x05\x04\x07\x02\x01\x01\x12\x04\xe3\x01!'\n\r\n\x05\x04\x07\x02\x01\
    \x03\x12\x04\xe3\x01*+\n\x0c\n\x04\x04\x07\x02\x02\x12\x04\xe5\x01\x02&\
    \n\r\n\x05\x04\x07\x02\x02\x04\x12\x04\xe5\x01\x02\n\n\r\n\x05\x04\x07\
    \x02\x02\x06\x12\x04\xe5\x01\x0b\x19\n\r\n\x05\x04\x07\x02\x02\x01\x12\
    \x04\xe5\x01\x1a!\n\r\n\x05\x04\x07\x02\x02\x03\x12\x04\xe5\x01$%\n0\n\
    \x02\x04\x08\x12\x06\xe9\x01\0\xf7\x01\x01\x1a\"\x20Describes\x20a\x20me\
    thod\x20of\x20a\x20service.\n\n\x0b\n\x03\x04\x08\x01\x12\x04\xe9\x01\
    \x08\x1d\n\x0c\n\x04\x04\x08\x02\0\x12\x04\xea\x01\x02\x1b\n\r\n\x05\x04\
    \x08\x02\0\x04\x12\x04\xea\x01\x02\n\n\r\n\x05\x04\x08\x02\0\x05\x12\x04\
    \xea\x01\x0b\x11\n\r\n\x05\x04\x08\x02\0\x01\x12\x04\xea\x01\x12\x16\n\r\
    \n\x05\x04\x08\x02\0\x03\x12\x04\xea\x01\x19\x1a\n\x97\x01\n\x04\x04\x08\
    \x02\x01\x12\x04\xee\x01\x02!\x1a\x88\x01\x20Input\x20and\x20output\x20t\
    ype\x20names.\x20\x20These\x20are\x20resolved\x20in\x20the\x20same\x20wa\
    y\x20as\n\x20FieldDescriptorProto.type_name,\x20but\x20must\x20refer\x20\
    to\x20a\x20message\x20type.\n\n\r\n\x05\x04\x08\x02\x01\x04\x12\x04\xee\
    \x01\x02\n\n\r\n\x05\x04\x08\x02\x01\x05\x12\x04\xee\x01\x0b\x11\n\r\n\
    \x05\x04\x08\x02\x01\x01\x12\x04\xee\x01\x12\x1c\n\r\n\x05\x04\x08\x02\
    \x01\x03\x12\x04\xee\x01\x1f\x20\n\x0c\n\x04\x04\x08\x02\x02\x12\x04\xef\
    \x01\x02\"\n\r\n\x05\x04\x08\x02\x02\x04\x12\x04\xef\x01\x02\n\n\r\n\x05\
    \x04\x08\x02\x02\x05\x12\x04\xef\x01\x0b\x11\n\r\n\x05\x04\x08\x02\x02\
    \x01\x12\x04\xef\x01\x12\x1d\n\r\n\x05\x04\x08\x02\x02\x03\x12\x04\xef\
    \x01\x20!\n\x0c\n\x04\x04\x08\x02\x03\x12\x04\xf1\x01\x02%\n\r\n\x05\x04\
    \x08\x02\x03\x04\x12\x04\xf1\x01\x02\n\n\r\n\x05\x04\x08\x02\x03\x06\x12\
    \x04\xf1\x01\x0b\x18\n\r\n\x05\x04\x08\x02\x03\x01\x12\x04\xf1\x01\x19\
    \x20\n\r\n\x05\x04\x08\x02\x03\x03\x12\x04\xf1\x01#$\nE\n\x04\x04\x08\
    \x02\x04\x12\x04\xf4\x01\x025\x1a7\x20Identifies\x20if\x20client\x20stre\
    ams\x20multiple\x20client\x20messages\n\n\r\n\x05\x04\x08\x02\x04\x04\
    \x12\x04\xf4\x01\x02\n\n\r\n\x05\x04\x08\x02\x04\x05\x12\x04\xf4\x01\x0b\
    \x0f\n\r\n\x05\x04\x08\x02\x04\x01\x12\x04\xf4\x01\x10\x20\n\r\n\x05\x04\
    \x08\x02\x04\x03\x12\x04\xf4\x01#$\n\r\n\x05\x04\x08\x02\x04\x08\x12\x04\
    \xf4\x01%4\n\r\n\x05\x04\x08\x02\x04\x07\x12\x04\xf4\x01.3\nE\n\x04\x04\
    \x08\x02\x05\x12\x04\xf6\x01\x025\x1a7\x20Identifies\x20if\x20server\x20\
    streams\x20multiple\x20server\x20messages\n\n\r\n\x05\x04\x08\x02\x05\
    \x04\x12\x04\xf6\x01\x02\n\n\r\n\x05\x04\x08\x02\x05\x05\x12\x04\xf6\x01\
    \x0b\x0f\n\r\n\x05\x04\x08\x02\x05\x01\x12\x04\xf6\x01\x10\x20\n\r\n\x05\
    \x04\x08\x02\x05\x03\x12\x04\xf6\x01#$\n\r\n\x05\x04\x08\x02\x05\x08\x12\
    \x04\xf6\x01%4\n\r\n\x05\x04\x08\x02\x05\x07\x12\x04\xf6\x01.3\n\xaf\x0e\
    \n\x02\x04\t\x12\x06\x9b\x02\0\xf8\x02\x012N\x20========================\
    ===========================================\n\x20Options\n2\xd0\r\x20Eac\
    h\x20of\x20the\x20definitions\x20above\x20may\x20have\x20\"options\"\x20\
    attached.\x20\x20These\x20are\n\x20just\x20annotations\x20which\x20may\
    \x20cause\x20code\x20to\x20be\x20generated\x20slightly\x20differently\n\
    \x20or\x20may\x20contain\x20hints\x20for\x20code\x20that\x20manipulates\
    \x20protocol\x20messages.\n\n\x20Clients\x20may\x20define\x20custom\x20o\
    ptions\x20as\x20extensions\x20of\x20the\x20*Options\x20messages.\n\x20Th\
    ese\x20extensions\x20may\x20not\x20yet\x20be\x20known\x20at\x20parsing\
    \x20time,\x20so\x20the\x20parser\x20cannot\n\x20store\x20the\x20values\
    \x20in\x20them.\x20\x20Instead\x20it\x20stores\x20them\x20in\x20a\x20fie\
    ld\x20in\x20the\x20*Options\n\x20message\x20called\x20uninterpreted_opti\
    on.\x20This\x20field\x20must\x20have\x20the\x20same\x20name\n\x20across\
    \x20all\x20*Options\x20messages.\x20We\x20then\x20use\x20this\x20field\
    \x20to\x20populate\x20the\n\x20extensions\x20when\x20we\x20build\x20a\
    \x20descriptor,\x20at\x20which\x20point\x20all\x20protos\x20have\x20been\
    \n\x20parsed\x20and\x20so\x20all\x20extensions\x20are\x20known.\n\n\x20E\
    xtension\x20numbers\x20for\x20custom\x20options\x20may\x20be\x20chosen\
    \x20as\x20follows:\n\x20*\x20For\x20options\x20which\x20will\x20only\x20\
    be\x20used\x20within\x20a\x20single\x20application\x20or\n\x20\x20\x20or\
    ganization,\x20or\x20for\x20experimental\x20options,\x20use\x20field\x20\
    numbers\x2050000\n\x20\x20\x20through\x2099999.\x20\x20It\x20is\x20up\
    \x20to\x20you\x20to\x20ensure\x20that\x20you\x20do\x20not\x20use\x20the\
    \n\x20\x20\x20same\x20number\x20for\x20multiple\x20options.\n\x20*\x20Fo\
    r\x20options\x20which\x20will\x20be\x20published\x20and\x20used\x20publi\
    cly\x20by\x20multiple\n\x20\x20\x20independent\x20entities,\x20e-mail\
    \x20protobuf-global-extension-registry@google.com\n\x20\x20\x20to\x20res\
    erve\x20extension\x20numbers.\x20Simply\x20provide\x20your\x20project\
    \x20name\x20(e.g.\n\x20\x20\x20Objective-C\x20plugin)\x20and\x20your\x20\
    project\x20website\x20(if\x20available)\x20--\x20there's\x20no\n\x20\x20\
    \x20need\x20to\x20explain\x20how\x20you\x20intend\x20to\x20use\x20them.\
    \x20Usually\x20you\x20only\x20need\x20one\n\x20\x20\x20extension\x20numb\
    er.\x20You\x20can\x20declare\x20multiple\x20options\x20with\x20only\x20o\
    ne\x20extension\n\x20\x20\x20number\x20by\x20putting\x20them\x20in\x20a\
    \x20sub-message.\x20See\x20the\x20Custom\x20Options\x20section\x20of\n\
    \x20\x20\x20the\x20docs\x20for\x20examples:\n\x20\x20\x20https://develop\
    ers.google.com/protocol-buffers/docs/proto#options\n\x20\x20\x20If\x20th\
    is\x20turns\x20out\x20to\x20be\x20popular,\x20a\x20web\x20service\x20wil\
    l\x20be\x20set\x20up\n\x20\x20\x20to\x20automatically\x20assign\x20optio\
    n\x20numbers.\n\n\x0b\n\x03\x04\t\x01\x12\x04\x9b\x02\x08\x13\n\xf4\x01\
    \n\x04\x04\t\x02\0\x12\x04\xa1\x02\x02#\x1a\xe5\x01\x20Sets\x20the\x20Ja\
    va\x20package\x20where\x20classes\x20generated\x20from\x20this\x20.proto\
    \x20will\x20be\n\x20placed.\x20\x20By\x20default,\x20the\x20proto\x20pac\
    kage\x20is\x20used,\x20but\x20this\x20is\x20often\n\x20inappropriate\x20\
    because\x20proto\x20packages\x20do\x20not\x20normally\x20start\x20with\
    \x20backwards\n\x20domain\x20names.\n\n\r\n\x05\x04\t\x02\0\x04\x12\x04\
    \xa1\x02\x02\n\n\r\n\x05\x04\t\x02\0\x05\x12\x04\xa1\x02\x0b\x11\n\r\n\
    \x05\x04\t\x02\0\x01\x12\x04\xa1\x02\x12\x1e\n\r\n\x05\x04\t\x02\0\x03\
    \x12\x04\xa1\x02!\"\n\xbf\x02\n\x04\x04\t\x02\x01\x12\x04\xa9\x02\x02+\
    \x1a\xb0\x02\x20If\x20set,\x20all\x20the\x20classes\x20from\x20the\x20.p\
    roto\x20file\x20are\x20wrapped\x20in\x20a\x20single\n\x20outer\x20class\
    \x20with\x20the\x20given\x20name.\x20\x20This\x20applies\x20to\x20both\
    \x20Proto1\n\x20(equivalent\x20to\x20the\x20old\x20\"--one_java_file\"\
    \x20option)\x20and\x20Proto2\x20(where\n\x20a\x20.proto\x20always\x20tra\
    nslates\x20to\x20a\x20single\x20class,\x20but\x20you\x20may\x20want\x20t\
    o\n\x20explicitly\x20choose\x20the\x20class\x20name).\n\n\r\n\x05\x04\t\
    \x02\x01\x04\x12\x04\xa9\x02\x02\n\n\r\n\x05\x04\t\x02\x01\x05\x12\x04\
    \xa9\x02\x0b\x11\n\r\n\x05\x04\t\x02\x01\x01\x12\x04\xa9\x02\x12&\n\r\n\
    \x05\x04\t\x02\x01\x03\x12\x04\xa9\x02)*\n\xa3\x03\n\x04\x04\t\x02\x02\
    \x12\x04\xb1\x02\x029\x1a\x94\x03\x20If\x20set\x20true,\x20then\x20the\
    \x20Java\x20code\x20generator\x20will\x20generate\x20a\x20separate\x20.j\
    ava\n\x20file\x20for\x20each\x20top-level\x20message,\x20enum,\x20and\
    \x20service\x20defined\x20in\x20the\x20.proto\n\x20file.\x20\x20Thus,\
    \x20these\x20types\x20will\x20*not*\x20be\x20nested\x20inside\x20the\x20\
    outer\x20class\n\x20named\x20by\x20java_outer_classname.\x20\x20However,\
    \x20the\x20outer\x20class\x20will\x20still\x20be\n\x20generated\x20to\
    \x20contain\x20the\x20file's\x20getDescriptor()\x20method\x20as\x20well\
    \x20as\x20any\n\x20top-level\x20extensions\x20defined\x20in\x20the\x20fi\
    le.\n\n\r\n\x05\x04\t\x02\x02\x04\x12\x04\xb1\x02\x02\n\n\r\n\x05\x04\t\
    \x02\x02\x05\x12\x04\xb1\x02\x0b\x0f\n\r\n\x05\x04\t\x02\x02\x01\x12\x04\
    \xb1\x02\x10#\n\r\n\x05\x04\t\x02\x02\x03\x12\x04\xb1\x02&(\n\r\n\x05\
    \x04\t\x02\x02\x08\x12\x04\xb1\x02)8\n\r\n\x05\x04\t\x02\x02\x07\x12\x04\
    \xb1\x0227\n)\n\x04\x04\t\x02\x03\x12\x04\xb4\x02\x02E\x1a\x1b\x20This\
    \x20option\x20does\x20nothing.\n\n\r\n\x05\x04\t\x02\x03\x04\x12\x04\xb4\
    \x02\x02\n\n\r\n\x05\x04\t\x02\x03\x05\x12\x04\xb4\x02\x0b\x0f\n\r\n\x05\
    \x04\t\x02\x03\x01\x12\x04\xb4\x02\x10-\n\r\n\x05\x04\t\x02\x03\x03\x12\
    \x04\xb4\x0202\n\r\n\x05\x04\t\x02\x03\x08\x12\x04\xb4\x023D\n\x10\n\x08\
    \x04\t\x02\x03\x08\xe7\x07\0\x12\x04\xb4\x024C\n\x11\n\t\x04\t\x02\x03\
    \x08\xe7\x07\0\x02\x12\x04\xb4\x024>\n\x12\n\n\x04\t\x02\x03\x08\xe7\x07\
    \0\x02\0\x12\x04\xb4\x024>\n\x13\n\x0b\x04\t\x02\x03\x08\xe7\x07\0\x02\0\
    \x01\x12\x04\xb4\x024>\n\x11\n\t\x04\t\x02\x03\x08\xe7\x07\0\x03\x12\x04\
    \xb4\x02?C\n\xe6\x02\n\x04\x04\t\x02\x04\x12\x04\xbc\x02\x02<\x1a\xd7\
    \x02\x20If\x20set\x20true,\x20then\x20the\x20Java2\x20code\x20generator\
    \x20will\x20generate\x20code\x20that\n\x20throws\x20an\x20exception\x20w\
    henever\x20an\x20attempt\x20is\x20made\x20to\x20assign\x20a\x20non-UTF-8\
    \n\x20byte\x20sequence\x20to\x20a\x20string\x20field.\n\x20Message\x20re\
    flection\x20will\x20do\x20the\x20same.\n\x20However,\x20an\x20extension\
    \x20field\x20still\x20accepts\x20non-UTF-8\x20byte\x20sequences.\n\x20Th\
    is\x20option\x20has\x20no\x20effect\x20on\x20when\x20used\x20with\x20the\
    \x20lite\x20runtime.\n\n\r\n\x05\x04\t\x02\x04\x04\x12\x04\xbc\x02\x02\n\
    \n\r\n\x05\x04\t\x02\x04\x05\x12\x04\xbc\x02\x0b\x0f\n\r\n\x05\x04\t\x02\
    \x04\x01\x12\x04\xbc\x02\x10&\n\r\n\x05\x04\t\x02\x04\x03\x12\x04\xbc\
    \x02)+\n\r\n\x05\x04\t\x02\x04\x08\x12\x04\xbc\x02,;\n\r\n\x05\x04\t\x02\
    \x04\x07\x12\x04\xbc\x025:\nL\n\x04\x04\t\x04\0\x12\x06\xc0\x02\x02\xc5\
    \x02\x03\x1a<\x20Generated\x20classes\x20can\x20be\x20optimized\x20for\
    \x20speed\x20or\x20code\x20size.\n\n\r\n\x05\x04\t\x04\0\x01\x12\x04\xc0\
    \x02\x07\x13\nD\n\x06\x04\t\x04\0\x02\0\x12\x04\xc1\x02\x04\x0e\"4\x20Ge\
    nerate\x20complete\x20code\x20for\x20parsing,\x20serialization,\n\n\x0f\
    \n\x07\x04\t\x04\0\x02\0\x01\x12\x04\xc1\x02\x04\t\n\x0f\n\x07\x04\t\x04\
    \0\x02\0\x02\x12\x04\xc1\x02\x0c\r\nG\n\x06\x04\t\x04\0\x02\x01\x12\x04\
    \xc3\x02\x04\x12\x1a\x06\x20etc.\n\"/\x20Use\x20ReflectionOps\x20to\x20i\
    mplement\x20these\x20methods.\n\n\x0f\n\x07\x04\t\x04\0\x02\x01\x01\x12\
    \x04\xc3\x02\x04\r\n\x0f\n\x07\x04\t\x04\0\x02\x01\x02\x12\x04\xc3\x02\
    \x10\x11\nG\n\x06\x04\t\x04\0\x02\x02\x12\x04\xc4\x02\x04\x15\"7\x20Gene\
    rate\x20code\x20using\x20MessageLite\x20and\x20the\x20lite\x20runtime.\n\
    \n\x0f\n\x07\x04\t\x04\0\x02\x02\x01\x12\x04\xc4\x02\x04\x10\n\x0f\n\x07\
    \x04\t\x04\0\x02\x02\x02\x12\x04\xc4\x02\x13\x14\n\x0c\n\x04\x04\t\x02\
    \x05\x12\x04\xc6\x02\x029\n\r\n\x05\x04\t\x02\x05\x04\x12\x04\xc6\x02\
    \x02\n\n\r\n\x05\x04\t\x02\x05\x06\x12\x04\xc6\x02\x0b\x17\n\r\n\x05\x04\
    \t\x02\x05\x01\x12\x04\xc6\x02\x18$\n\r\n\x05\x04\t\x02\x05\x03\x12\x04\
    \xc6\x02'(\n\r\n\x05\x04\t\x02\x05\x08\x12\x04\xc6\x02)8\n\r\n\x05\x04\t\
    \x02\x05\x07\x12\x04\xc6\x0227\n\xe2\x02\n\x04\x04\t\x02\x06\x12\x04\xcd\
    \x02\x02\"\x1a\xd3\x02\x20Sets\x20the\x20Go\x20package\x20where\x20struc\
    ts\x20generated\x20from\x20this\x20.proto\x20will\x20be\n\x20placed.\x20\
    If\x20omitted,\x20the\x20Go\x20package\x20will\x20be\x20derived\x20from\
    \x20the\x20following:\n\x20\x20\x20-\x20The\x20basename\x20of\x20the\x20\
    package\x20import\x20path,\x20if\x20provided.\n\x20\x20\x20-\x20Otherwis\
    e,\x20the\x20package\x20statement\x20in\x20the\x20.proto\x20file,\x20if\
    \x20present.\n\x20\x20\x20-\x20Otherwise,\x20the\x20basename\x20of\x20th\
    e\x20.proto\x20file,\x20without\x20extension.\n\n\r\n\x05\x04\t\x02\x06\
    \x04\x12\x04\xcd\x02\x02\n\n\r\n\x05\x04\t\x02\x06\x05\x12\x04\xcd\x02\
    \x0b\x11\n\r\n\x05\x04\t\x02\x06\x01\x12\x04\xcd\x02\x12\x1c\n\r\n\x05\
    \x04\t\x02\x06\x03\x12\x04\xcd\x02\x1f!\n\xd4\x04\n\x04\x04\t\x02\x07\
    \x12\x04\xdb\x02\x029\x1a\xc5\x04\x20Should\x20generic\x20services\x20be\
    \x20generated\x20in\x20each\x20language?\x20\x20\"Generic\"\x20services\
    \n\x20are\x20not\x20specific\x20to\x20any\x20particular\x20RPC\x20system\
    .\x20\x20They\x20are\x20generated\x20by\x20the\n\x20main\x20code\x20gene\
    rators\x20in\x20each\x20language\x20(without\x20additional\x20plugins).\
    \n\x20Generic\x20services\x20were\x20the\x20only\x20kind\x20of\x20servic\
    e\x20generation\x20supported\x20by\n\x20early\x20versions\x20of\x20googl\
    e.protobuf.\n\n\x20Generic\x20services\x20are\x20now\x20considered\x20de\
    precated\x20in\x20favor\x20of\x20using\x20plugins\n\x20that\x20generate\
    \x20code\x20specific\x20to\x20your\x20particular\x20RPC\x20system.\x20\
    \x20Therefore,\n\x20these\x20default\x20to\x20false.\x20\x20Old\x20code\
    \x20which\x20depends\x20on\x20generic\x20services\x20should\n\x20explici\
    tly\x20set\x20them\x20to\x20true.\n\n\r\n\x05\x04\t\x02\x07\x04\x12\x04\
    \xdb\x02\x02\n\n\r\n\x05\x04\t\x02\x07\x05\x12\x04\xdb\x02\x0b\x0f\n\r\n\
    \x05\x04\t\x02\x07\x01\x12\x04\xdb\x02\x10#\n\r\n\x05\x04\t\x02\x07\x03\
    \x12\x04\xdb\x02&(\n\r\n\x05\x04\t\x02\x07\x08\x12\x04\xdb\x02)8\n\r\n\
    \x05\x04\t\x02\x07\x07\x12\x04\xdb\x0227\n\x0c\n\x04\x04\t\x02\x08\x12\
    \x04\xdc\x02\x02;\n\r\n\x05\x04\t\x02\x08\x04\x12\x04\xdc\x02\x02\n\n\r\
    \n\x05\x04\t\x02\x08\x05\x12\x04\xdc\x02\x0b\x0f\n\r\n\x05\x04\t\x02\x08\
    \x01\x12\x04\xdc\x02\x10%\n\r\n\x05\x04\t\x02\x08\x03\x12\x04\xdc\x02(*\
    \n\r\n\x05\x04\t\x02\x08\x08\x12\x04\xdc\x02+:\n\r\n\x05\x04\t\x02\x08\
    \x07\x12\x04\xdc\x0249\n\x0c\n\x04\x04\t\x02\t\x12\x04\xdd\x02\x029\n\r\
    \n\x05\x04\t\x02\t\x04\x12\x04\xdd\x02\x02\n\n\r\n\x05\x04\t\x02\t\x05\
    \x12\x04\xdd\x02\x0b\x0f\n\r\n\x05\x04\t\x02\t\x01\x12\x04\xdd\x02\x10#\
    \n\r\n\x05\x04\t\x02\t\x03\x12\x04\xdd\x02&(\n\r\n\x05\x04\t\x02\t\x08\
    \x12\x04\xdd\x02)8\n\r\n\x05\x04\t\x02\t\x07\x12\x04\xdd\x0227\n\xf3\x01\
    \n\x04\x04\t\x02\n\x12\x04\xe3\x02\x020\x1a\xe4\x01\x20Is\x20this\x20fil\
    e\x20deprecated?\n\x20Depending\x20on\x20the\x20target\x20platform,\x20t\
    his\x20can\x20emit\x20Deprecated\x20annotations\n\x20for\x20everything\
    \x20in\x20the\x20file,\x20or\x20it\x20will\x20be\x20completely\x20ignore\
    d;\x20in\x20the\x20very\n\x20least,\x20this\x20is\x20a\x20formalization\
    \x20for\x20deprecating\x20files.\n\n\r\n\x05\x04\t\x02\n\x04\x12\x04\xe3\
    \x02\x02\n\n\r\n\x05\x04\t\x02\n\x05\x12\x04\xe3\x02\x0b\x0f\n\r\n\x05\
    \x04\t\x02\n\x01\x12\x04\xe3\x02\x10\x1a\n\r\n\x05\x04\t\x02\n\x03\x12\
    \x04\xe3\x02\x1d\x1f\n\r\n\x05\x04\t\x02\n\x08\x12\x04\xe3\x02\x20/\n\r\
    \n\x05\x04\t\x02\n\x07\x12\x04\xe3\x02).\n\x7f\n\x04\x04\t\x02\x0b\x12\
    \x04\xe7\x02\x026\x1aq\x20Enables\x20the\x20use\x20of\x20arenas\x20for\
    \x20the\x20proto\x20messages\x20in\x20this\x20file.\x20This\x20applies\n\
    \x20only\x20to\x20generated\x20classes\x20for\x20C++.\n\n\r\n\x05\x04\t\
    \x02\x0b\x04\x12\x04\xe7\x02\x02\n\n\r\n\x05\x04\t\x02\x0b\x05\x12\x04\
    \xe7\x02\x0b\x0f\n\r\n\x05\x04\t\x02\x0b\x01\x12\x04\xe7\x02\x10\x20\n\r\
    \n\x05\x04\t\x02\x0b\x03\x12\x04\xe7\x02#%\n\r\n\x05\x04\t\x02\x0b\x08\
    \x12\x04\xe7\x02&5\n\r\n\x05\x04\t\x02\x0b\x07\x12\x04\xe7\x02/4\n\x92\
    \x01\n\x04\x04\t\x02\x0c\x12\x04\xec\x02\x02)\x1a\x83\x01\x20Sets\x20the\
    \x20objective\x20c\x20class\x20prefix\x20which\x20is\x20prepended\x20to\
    \x20all\x20objective\x20c\n\x20generated\x20classes\x20from\x20this\x20.\
    proto.\x20There\x20is\x20no\x20default.\n\n\r\n\x05\x04\t\x02\x0c\x04\
    \x12\x04\xec\x02\x02\n\n\r\n\x05\x04\t\x02\x0c\x05\x12\x04\xec\x02\x0b\
    \x11\n\r\n\x05\x04\t\x02\x0c\x01\x12\x04\xec\x02\x12#\n\r\n\x05\x04\t\
    \x02\x0c\x03\x12\x04\xec\x02&(\nI\n\x04\x04\t\x02\r\x12\x04\xef\x02\x02(\
    \x1a;\x20Namespace\x20for\x20generated\x20classes;\x20defaults\x20to\x20\
    the\x20package.\n\n\r\n\x05\x04\t\x02\r\x04\x12\x04\xef\x02\x02\n\n\r\n\
    \x05\x04\t\x02\r\x05\x12\x04\xef\x02\x0b\x11\n\r\n\x05\x04\t\x02\r\x01\
    \x12\x04\xef\x02\x12\"\n\r\n\x05\x04\t\x02\r\x03\x12\x04\xef\x02%'\nO\n\
    \x04\x04\t\x02\x0e\x12\x04\xf2\x02\x02:\x1aA\x20The\x20parser\x20stores\
    \x20options\x20it\x20doesn't\x20recognize\x20here.\x20See\x20above.\n\n\
    \r\n\x05\x04\t\x02\x0e\x04\x12\x04\xf2\x02\x02\n\n\r\n\x05\x04\t\x02\x0e\
    \x06\x12\x04\xf2\x02\x0b\x1e\n\r\n\x05\x04\t\x02\x0e\x01\x12\x04\xf2\x02\
    \x1f3\n\r\n\x05\x04\t\x02\x0e\x03\x12\x04\xf2\x0269\nZ\n\x03\x04\t\x05\
    \x12\x04\xf5\x02\x02\x19\x1aM\x20Clients\x20can\x20define\x20custom\x20o\
    ptions\x20in\x20extensions\x20of\x20this\x20message.\x20See\x20above.\n\
    \n\x0c\n\x04\x04\t\x05\0\x12\x04\xf5\x02\r\x18\n\r\n\x05\x04\t\x05\0\x01\
    \x12\x04\xf5\x02\r\x11\n\r\n\x05\x04\t\x05\0\x02\x12\x04\xf5\x02\x15\x18\
    \n\x0c\n\x02\x04\n\x12\x06\xfa\x02\0\xb8\x03\x01\n\x0b\n\x03\x04\n\x01\
    \x12\x04\xfa\x02\x08\x16\n\xd8\x05\n\x04\x04\n\x02\0\x12\x04\x8d\x03\x02\
    <\x1a\xc9\x05\x20Set\x20true\x20to\x20use\x20the\x20old\x20proto1\x20Mes\
    sageSet\x20wire\x20format\x20for\x20extensions.\n\x20This\x20is\x20provi\
    ded\x20for\x20backwards-compatibility\x20with\x20the\x20MessageSet\x20wi\
    re\n\x20format.\x20\x20You\x20should\x20not\x20use\x20this\x20for\x20any\
    \x20other\x20reason:\x20\x20It's\x20less\n\x20efficient,\x20has\x20fewer\
    \x20features,\x20and\x20is\x20more\x20complicated.\n\n\x20The\x20message\
    \x20must\x20be\x20defined\x20exactly\x20as\x20follows:\n\x20\x20\x20mess\
    age\x20Foo\x20{\n\x20\x20\x20\x20\x20option\x20message_set_wire_format\
    \x20=\x20true;\n\x20\x20\x20\x20\x20extensions\x204\x20to\x20max;\n\x20\
    \x20\x20}\n\x20Note\x20that\x20the\x20message\x20cannot\x20have\x20any\
    \x20defined\x20fields;\x20MessageSets\x20only\n\x20have\x20extensions.\n\
    \n\x20All\x20extensions\x20of\x20your\x20type\x20must\x20be\x20singular\
    \x20messages;\x20e.g.\x20they\x20cannot\n\x20be\x20int32s,\x20enums,\x20\
    or\x20repeated\x20messages.\n\n\x20Because\x20this\x20is\x20an\x20option\
    ,\x20the\x20above\x20two\x20restrictions\x20are\x20not\x20enforced\x20by\
    \n\x20the\x20protocol\x20compiler.\n\n\r\n\x05\x04\n\x02\0\x04\x12\x04\
    \x8d\x03\x02\n\n\r\n\x05\x04\n\x02\0\x05\x12\x04\x8d\x03\x0b\x0f\n\r\n\
    \x05\x04\n\x02\0\x01\x12\x04\x8d\x03\x10'\n\r\n\x05\x04\n\x02\0\x03\x12\
    \x04\x8d\x03*+\n\r\n\x05\x04\n\x02\0\x08\x12\x04\x8d\x03,;\n\r\n\x05\x04\
    \n\x02\0\x07\x12\x04\x8d\x035:\n\xeb\x01\n\x04\x04\n\x02\x01\x12\x04\x92\
    \x03\x02D\x1a\xdc\x01\x20Disables\x20the\x20generation\x20of\x20the\x20s\
    tandard\x20\"descriptor()\"\x20accessor,\x20which\x20can\n\x20conflict\
    \x20with\x20a\x20field\x20of\x20the\x20same\x20name.\x20\x20This\x20is\
    \x20meant\x20to\x20make\x20migration\n\x20from\x20proto1\x20easier;\x20n\
    ew\x20code\x20should\x20avoid\x20fields\x20named\x20\"descriptor\".\n\n\
    \r\n\x05\x04\n\x02\x01\x04\x12\x04\x92\x03\x02\n\n\r\n\x05\x04\n\x02\x01\
    \x05\x12\x04\x92\x03\x0b\x0f\n\r\n\x05\x04\n\x02\x01\x01\x12\x04\x92\x03\
    \x10/\n\r\n\x05\x04\n\x02\x01\x03\x12\x04\x92\x0323\n\r\n\x05\x04\n\x02\
    \x01\x08\x12\x04\x92\x034C\n\r\n\x05\x04\n\x02\x01\x07\x12\x04\x92\x03=B\
    \n\xee\x01\n\x04\x04\n\x02\x02\x12\x04\x98\x03\x02/\x1a\xdf\x01\x20Is\
    \x20this\x20message\x20deprecated?\n\x20Depending\x20on\x20the\x20target\
    \x20platform,\x20this\x20can\x20emit\x20Deprecated\x20annotations\n\x20f\
    or\x20the\x20message,\x20or\x20it\x20will\x20be\x20completely\x20ignored\
    ;\x20in\x20the\x20very\x20least,\n\x20this\x20is\x20a\x20formalization\
    \x20for\x20deprecating\x20messages.\n\n\r\n\x05\x04\n\x02\x02\x04\x12\
    \x04\x98\x03\x02\n\n\r\n\x05\x04\n\x02\x02\x05\x12\x04\x98\x03\x0b\x0f\n\
    \r\n\x05\x04\n\x02\x02\x01\x12\x04\x98\x03\x10\x1a\n\r\n\x05\x04\n\x02\
    \x02\x03\x12\x04\x98\x03\x1d\x1e\n\r\n\x05\x04\n\x02\x02\x08\x12\x04\x98\
    \x03\x1f.\n\r\n\x05\x04\n\x02\x02\x07\x12\x04\x98\x03(-\n\x9e\x06\n\x04\
    \x04\n\x02\x03\x12\x04\xaf\x03\x02\x1e\x1a\x8f\x06\x20Whether\x20the\x20\
    message\x20is\x20an\x20automatically\x20generated\x20map\x20entry\x20typ\
    e\x20for\x20the\n\x20maps\x20field.\n\n\x20For\x20maps\x20fields:\n\x20\
    \x20\x20\x20\x20map<KeyType,\x20ValueType>\x20map_field\x20=\x201;\n\x20\
    The\x20parsed\x20descriptor\x20looks\x20like:\n\x20\x20\x20\x20\x20messa\
    ge\x20MapFieldEntry\x20{\n\x20\x20\x20\x20\x20\x20\x20\x20\x20option\x20\
    map_entry\x20=\x20true;\n\x20\x20\x20\x20\x20\x20\x20\x20\x20optional\
    \x20KeyType\x20key\x20=\x201;\n\x20\x20\x20\x20\x20\x20\x20\x20\x20optio\
    nal\x20ValueType\x20value\x20=\x202;\n\x20\x20\x20\x20\x20}\n\x20\x20\
    \x20\x20\x20repeated\x20MapFieldEntry\x20map_field\x20=\x201;\n\n\x20Imp\
    lementations\x20may\x20choose\x20not\x20to\x20generate\x20the\x20map_ent\
    ry=true\x20message,\x20but\n\x20use\x20a\x20native\x20map\x20in\x20the\
    \x20target\x20language\x20to\x20hold\x20the\x20keys\x20and\x20values.\n\
    \x20The\x20reflection\x20APIs\x20in\x20such\x20implementions\x20still\
    \x20need\x20to\x20work\x20as\n\x20if\x20the\x20field\x20is\x20a\x20repea\
    ted\x20message\x20field.\n\n\x20NOTE:\x20Do\x20not\x20set\x20the\x20opti\
    on\x20in\x20.proto\x20files.\x20Always\x20use\x20the\x20maps\x20syntax\n\
    \x20instead.\x20The\x20option\x20should\x20only\x20be\x20implicitly\x20s\
    et\x20by\x20the\x20proto\x20compiler\n\x20parser.\n\n\r\n\x05\x04\n\x02\
    \x03\x04\x12\x04\xaf\x03\x02\n\n\r\n\x05\x04\n\x02\x03\x05\x12\x04\xaf\
    \x03\x0b\x0f\n\r\n\x05\x04\n\x02\x03\x01\x12\x04\xaf\x03\x10\x19\n\r\n\
    \x05\x04\n\x02\x03\x03\x12\x04\xaf\x03\x1c\x1d\nO\n\x04\x04\n\x02\x04\
    \x12\x04\xb2\x03\x02:\x1aA\x20The\x20parser\x20stores\x20options\x20it\
    \x20doesn't\x20recognize\x20here.\x20See\x20above.\n\n\r\n\x05\x04\n\x02\
    \x04\x04\x12\x04\xb2\x03\x02\n\n\r\n\x05\x04\n\x02\x04\x06\x12\x04\xb2\
    \x03\x0b\x1e\n\r\n\x05\x04\n\x02\x04\x01\x12\x04\xb2\x03\x1f3\n\r\n\x05\
    \x04\n\x02\x04\x03\x12\x04\xb2\x0369\nZ\n\x03\x04\n\x05\x12\x04\xb5\x03\
    \x02\x19\x1aM\x20Clients\x20can\x20define\x20custom\x20options\x20in\x20\
    extensions\x20of\x20this\x20message.\x20See\x20above.\n\n\x0c\n\x04\x04\
    \n\x05\0\x12\x04\xb5\x03\r\x18\n\r\n\x05\x04\n\x05\0\x01\x12\x04\xb5\x03\
    \r\x11\n\r\n\x05\x04\n\x05\0\x02\x12\x04\xb5\x03\x15\x18\n\x0c\n\x02\x04\
    \x0b\x12\x06\xba\x03\0\x93\x04\x01\n\x0b\n\x03\x04\x0b\x01\x12\x04\xba\
    \x03\x08\x14\n\xa3\x02\n\x04\x04\x0b\x02\0\x12\x04\xbf\x03\x02.\x1a\x94\
    \x02\x20The\x20ctype\x20option\x20instructs\x20the\x20C++\x20code\x20gen\
    erator\x20to\x20use\x20a\x20different\n\x20representation\x20of\x20the\
    \x20field\x20than\x20it\x20normally\x20would.\x20\x20See\x20the\x20speci\
    fic\n\x20options\x20below.\x20\x20This\x20option\x20is\x20not\x20yet\x20\
    implemented\x20in\x20the\x20open\x20source\n\x20release\x20--\x20sorry,\
    \x20we'll\x20try\x20to\x20include\x20it\x20in\x20a\x20future\x20version!\
    \n\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04\xbf\x03\x02\n\n\r\n\x05\x04\x0b\
    \x02\0\x06\x12\x04\xbf\x03\x0b\x10\n\r\n\x05\x04\x0b\x02\0\x01\x12\x04\
    \xbf\x03\x11\x16\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\xbf\x03\x19\x1a\n\r\
    \n\x05\x04\x0b\x02\0\x08\x12\x04\xbf\x03\x1b-\n\r\n\x05\x04\x0b\x02\0\
    \x07\x12\x04\xbf\x03&,\n\x0e\n\x04\x04\x0b\x04\0\x12\x06\xc0\x03\x02\xc7\
    \x03\x03\n\r\n\x05\x04\x0b\x04\0\x01\x12\x04\xc0\x03\x07\x0c\n\x1f\n\x06\
    \x04\x0b\x04\0\x02\0\x12\x04\xc2\x03\x04\x0f\x1a\x0f\x20Default\x20mode.\
    \n\n\x0f\n\x07\x04\x0b\x04\0\x02\0\x01\x12\x04\xc2\x03\x04\n\n\x0f\n\x07\
    \x04\x0b\x04\0\x02\0\x02\x12\x04\xc2\x03\r\x0e\n\x0e\n\x06\x04\x0b\x04\0\
    \x02\x01\x12\x04\xc4\x03\x04\r\n\x0f\n\x07\x04\x0b\x04\0\x02\x01\x01\x12\
    \x04\xc4\x03\x04\x08\n\x0f\n\x07\x04\x0b\x04\0\x02\x01\x02\x12\x04\xc4\
    \x03\x0b\x0c\n\x0e\n\x06\x04\x0b\x04\0\x02\x02\x12\x04\xc6\x03\x04\x15\n\
    \x0f\n\x07\x04\x0b\x04\0\x02\x02\x01\x12\x04\xc6\x03\x04\x10\n\x0f\n\x07\
    \x04\x0b\x04\0\x02\x02\x02\x12\x04\xc6\x03\x13\x14\n\xda\x02\n\x04\x04\
    \x0b\x02\x01\x12\x04\xcd\x03\x02\x1b\x1a\xcb\x02\x20The\x20packed\x20opt\
    ion\x20can\x20be\x20enabled\x20for\x20repeated\x20primitive\x20fields\
    \x20to\x20enable\n\x20a\x20more\x20efficient\x20representation\x20on\x20\
    the\x20wire.\x20Rather\x20than\x20repeatedly\n\x20writing\x20the\x20tag\
    \x20and\x20type\x20for\x20each\x20element,\x20the\x20entire\x20array\x20\
    is\x20encoded\x20as\n\x20a\x20single\x20length-delimited\x20blob.\x20In\
    \x20proto3,\x20only\x20explicit\x20setting\x20it\x20to\n\x20false\x20wil\
    l\x20avoid\x20using\x20packed\x20encoding.\n\n\r\n\x05\x04\x0b\x02\x01\
    \x04\x12\x04\xcd\x03\x02\n\n\r\n\x05\x04\x0b\x02\x01\x05\x12\x04\xcd\x03\
    \x0b\x0f\n\r\n\x05\x04\x0b\x02\x01\x01\x12\x04\xcd\x03\x10\x16\n\r\n\x05\
    \x04\x0b\x02\x01\x03\x12\x04\xcd\x03\x19\x1a\n\xe4\x04\n\x04\x04\x0b\x02\
    \x02\x12\x04\xd8\x03\x023\x1a\xd5\x04\x20The\x20jstype\x20option\x20dete\
    rmines\x20the\x20JavaScript\x20type\x20used\x20for\x20values\x20of\x20th\
    e\n\x20field.\x20\x20The\x20option\x20is\x20permitted\x20only\x20for\x20\
    64\x20bit\x20integral\x20and\x20fixed\x20types\n\x20(int64,\x20uint64,\
    \x20sint64,\x20fixed64,\x20sfixed64).\x20\x20By\x20default\x20these\x20t\
    ypes\x20are\n\x20represented\x20as\x20JavaScript\x20strings.\x20\x20This\
    \x20avoids\x20loss\x20of\x20precision\x20that\x20can\n\x20happen\x20when\
    \x20a\x20large\x20value\x20is\x20converted\x20to\x20a\x20floating\x20poi\
    nt\x20JavaScript\n\x20numbers.\x20\x20Specifying\x20JS_NUMBER\x20for\x20\
    the\x20jstype\x20causes\x20the\x20generated\n\x20JavaScript\x20code\x20t\
    o\x20use\x20the\x20JavaScript\x20\"number\"\x20type\x20instead\x20of\x20\
    strings.\n\x20This\x20option\x20is\x20an\x20enum\x20to\x20permit\x20addi\
    tional\x20types\x20to\x20be\x20added,\n\x20e.g.\x20goog.math.Integer.\n\
    \n\r\n\x05\x04\x0b\x02\x02\x04\x12\x04\xd8\x03\x02\n\n\r\n\x05\x04\x0b\
    \x02\x02\x06\x12\x04\xd8\x03\x0b\x11\n\r\n\x05\x04\x0b\x02\x02\x01\x12\
    \x04\xd8\x03\x12\x18\n\r\n\x05\x04\x0b\x02\x02\x03\x12\x04\xd8\x03\x1b\
    \x1c\n\r\n\x05\x04\x0b\x02\x02\x08\x12\x04\xd8\x03\x1d2\n\r\n\x05\x04\
    \x0b\x02\x02\x07\x12\x04\xd8\x03(1\n\x0e\n\x04\x04\x0b\x04\x01\x12\x06\
    \xd9\x03\x02\xe2\x03\x03\n\r\n\x05\x04\x0b\x04\x01\x01\x12\x04\xd9\x03\
    \x07\r\n'\n\x06\x04\x0b\x04\x01\x02\0\x12\x04\xdb\x03\x04\x12\x1a\x17\
    \x20Use\x20the\x20default\x20type.\n\n\x0f\n\x07\x04\x0b\x04\x01\x02\0\
    \x01\x12\x04\xdb\x03\x04\r\n\x0f\n\x07\x04\x0b\x04\x01\x02\0\x02\x12\x04\
    \xdb\x03\x10\x11\n)\n\x06\x04\x0b\x04\x01\x02\x01\x12\x04\xde\x03\x04\
    \x12\x1a\x19\x20Use\x20JavaScript\x20strings.\n\n\x0f\n\x07\x04\x0b\x04\
    \x01\x02\x01\x01\x12\x04\xde\x03\x04\r\n\x0f\n\x07\x04\x0b\x04\x01\x02\
    \x01\x02\x12\x04\xde\x03\x10\x11\n)\n\x06\x04\x0b\x04\x01\x02\x02\x12\
    \x04\xe1\x03\x04\x12\x1a\x19\x20Use\x20JavaScript\x20numbers.\n\n\x0f\n\
    \x07\x04\x0b\x04\x01\x02\x02\x01\x12\x04\xe1\x03\x04\r\n\x0f\n\x07\x04\
    \x0b\x04\x01\x02\x02\x02\x12\x04\xe1\x03\x10\x11\n\xef\x0c\n\x04\x04\x0b\
    \x02\x03\x12\x04\x80\x04\x02)\x1a\xe0\x0c\x20Should\x20this\x20field\x20\
    be\x20parsed\x20lazily?\x20\x20Lazy\x20applies\x20only\x20to\x20message-\
    type\n\x20fields.\x20\x20It\x20means\x20that\x20when\x20the\x20outer\x20\
    message\x20is\x20initially\x20parsed,\x20the\n\x20inner\x20message's\x20\
    contents\x20will\x20not\x20be\x20parsed\x20but\x20instead\x20stored\x20i\
    n\x20encoded\n\x20form.\x20\x20The\x20inner\x20message\x20will\x20actual\
    ly\x20be\x20parsed\x20when\x20it\x20is\x20first\x20accessed.\n\n\x20This\
    \x20is\x20only\x20a\x20hint.\x20\x20Implementations\x20are\x20free\x20to\
    \x20choose\x20whether\x20to\x20use\n\x20eager\x20or\x20lazy\x20parsing\
    \x20regardless\x20of\x20the\x20value\x20of\x20this\x20option.\x20\x20How\
    ever,\n\x20setting\x20this\x20option\x20true\x20suggests\x20that\x20the\
    \x20protocol\x20author\x20believes\x20that\n\x20using\x20lazy\x20parsing\
    \x20on\x20this\x20field\x20is\x20worth\x20the\x20additional\x20bookkeepi\
    ng\n\x20overhead\x20typically\x20needed\x20to\x20implement\x20it.\n\n\
    \x20This\x20option\x20does\x20not\x20affect\x20the\x20public\x20interfac\
    e\x20of\x20any\x20generated\x20code;\n\x20all\x20method\x20signatures\
    \x20remain\x20the\x20same.\x20\x20Furthermore,\x20thread-safety\x20of\
    \x20the\n\x20interface\x20is\x20not\x20affected\x20by\x20this\x20option;\
    \x20const\x20methods\x20remain\x20safe\x20to\n\x20call\x20from\x20multip\
    le\x20threads\x20concurrently,\x20while\x20non-const\x20methods\x20conti\
    nue\n\x20to\x20require\x20exclusive\x20access.\n\n\n\x20Note\x20that\x20\
    implementations\x20may\x20choose\x20not\x20to\x20check\x20required\x20fi\
    elds\x20within\n\x20a\x20lazy\x20sub-message.\x20\x20That\x20is,\x20call\
    ing\x20IsInitialized()\x20on\x20the\x20outer\x20message\n\x20may\x20retu\
    rn\x20true\x20even\x20if\x20the\x20inner\x20message\x20has\x20missing\
    \x20required\x20fields.\n\x20This\x20is\x20necessary\x20because\x20other\
    wise\x20the\x20inner\x20message\x20would\x20have\x20to\x20be\n\x20parsed\
    \x20in\x20order\x20to\x20perform\x20the\x20check,\x20defeating\x20the\
    \x20purpose\x20of\x20lazy\n\x20parsing.\x20\x20An\x20implementation\x20w\
    hich\x20chooses\x20not\x20to\x20check\x20required\x20fields\n\x20must\
    \x20be\x20consistent\x20about\x20it.\x20\x20That\x20is,\x20for\x20any\
    \x20particular\x20sub-message,\x20the\n\x20implementation\x20must\x20eit\
    her\x20*always*\x20check\x20its\x20required\x20fields,\x20or\x20*never*\
    \n\x20check\x20its\x20required\x20fields,\x20regardless\x20of\x20whether\
    \x20or\x20not\x20the\x20message\x20has\n\x20been\x20parsed.\n\n\r\n\x05\
    \x04\x0b\x02\x03\x04\x12\x04\x80\x04\x02\n\n\r\n\x05\x04\x0b\x02\x03\x05\
    \x12\x04\x80\x04\x0b\x0f\n\r\n\x05\x04\x0b\x02\x03\x01\x12\x04\x80\x04\
    \x10\x14\n\r\n\x05\x04\x0b\x02\x03\x03\x12\x04\x80\x04\x17\x18\n\r\n\x05\
    \x04\x0b\x02\x03\x08\x12\x04\x80\x04\x19(\n\r\n\x05\x04\x0b\x02\x03\x07\
    \x12\x04\x80\x04\"'\n\xe8\x01\n\x04\x04\x0b\x02\x04\x12\x04\x86\x04\x02/\
    \x1a\xd9\x01\x20Is\x20this\x20field\x20deprecated?\n\x20Depending\x20on\
    \x20the\x20target\x20platform,\x20this\x20can\x20emit\x20Deprecated\x20a\
    nnotations\n\x20for\x20accessors,\x20or\x20it\x20will\x20be\x20completel\
    y\x20ignored;\x20in\x20the\x20very\x20least,\x20this\n\x20is\x20a\x20for\
    malization\x20for\x20deprecating\x20fields.\n\n\r\n\x05\x04\x0b\x02\x04\
    \x04\x12\x04\x86\x04\x02\n\n\r\n\x05\x04\x0b\x02\x04\x05\x12\x04\x86\x04\
    \x0b\x0f\n\r\n\x05\x04\x0b\x02\x04\x01\x12\x04\x86\x04\x10\x1a\n\r\n\x05\
    \x04\x0b\x02\x04\x03\x12\x04\x86\x04\x1d\x1e\n\r\n\x05\x04\x0b\x02\x04\
    \x08\x12\x04\x86\x04\x1f.\n\r\n\x05\x04\x0b\x02\x04\x07\x12\x04\x86\x04(\
    -\n?\n\x04\x04\x0b\x02\x05\x12\x04\x89\x04\x02*\x1a1\x20For\x20Google-in\
    ternal\x20migration\x20only.\x20Do\x20not\x20use.\n\n\r\n\x05\x04\x0b\
    \x02\x05\x04\x12\x04\x89\x04\x02\n\n\r\n\x05\x04\x0b\x02\x05\x05\x12\x04\
    \x89\x04\x0b\x0f\n\r\n\x05\x04\x0b\x02\x05\x01\x12\x04\x89\x04\x10\x14\n\
    \r\n\x05\x04\x0b\x02\x05\x03\x12\x04\x89\x04\x17\x19\n\r\n\x05\x04\x0b\
    \x02\x05\x08\x12\x04\x89\x04\x1a)\n\r\n\x05\x04\x0b\x02\x05\x07\x12\x04\
    \x89\x04#(\nO\n\x04\x04\x0b\x02\x06\x12\x04\x8d\x04\x02:\x1aA\x20The\x20\
    parser\x20stores\x20options\x20it\x20doesn't\x20recognize\x20here.\x20Se\
    e\x20above.\n\n\r\n\x05\x04\x0b\x02\x06\x04\x12\x04\x8d\x04\x02\n\n\r\n\
    \x05\x04\x0b\x02\x06\x06\x12\x04\x8d\x04\x0b\x1e\n\r\n\x05\x04\x0b\x02\
    \x06\x01\x12\x04\x8d\x04\x1f3\n\r\n\x05\x04\x0b\x02\x06\x03\x12\x04\x8d\
    \x0469\nZ\n\x03\x04\x0b\x05\x12\x04\x90\x04\x02\x19\x1aM\x20Clients\x20c\
    an\x20define\x20custom\x20options\x20in\x20extensions\x20of\x20this\x20m\
    essage.\x20See\x20above.\n\n\x0c\n\x04\x04\x0b\x05\0\x12\x04\x90\x04\r\
    \x18\n\r\n\x05\x04\x0b\x05\0\x01\x12\x04\x90\x04\r\x11\n\r\n\x05\x04\x0b\
    \x05\0\x02\x12\x04\x90\x04\x15\x18\n\x0c\n\x02\x04\x0c\x12\x06\x95\x04\0\
    \x9b\x04\x01\n\x0b\n\x03\x04\x0c\x01\x12\x04\x95\x04\x08\x14\nO\n\x04\
    \x04\x0c\x02\0\x12\x04\x97\x04\x02:\x1aA\x20The\x20parser\x20stores\x20o\
    ptions\x20it\x20doesn't\x20recognize\x20here.\x20See\x20above.\n\n\r\n\
    \x05\x04\x0c\x02\0\x04\x12\x04\x97\x04\x02\n\n\r\n\x05\x04\x0c\x02\0\x06\
    \x12\x04\x97\x04\x0b\x1e\n\r\n\x05\x04\x0c\x02\0\x01\x12\x04\x97\x04\x1f\
    3\n\r\n\x05\x04\x0c\x02\0\x03\x12\x04\x97\x0469\nZ\n\x03\x04\x0c\x05\x12\
    \x04\x9a\x04\x02\x19\x1aM\x20Clients\x20can\x20define\x20custom\x20optio\
    ns\x20in\x20extensions\x20of\x20this\x20message.\x20See\x20above.\n\n\
    \x0c\n\x04\x04\x0c\x05\0\x12\x04\x9a\x04\r\x18\n\r\n\x05\x04\x0c\x05\0\
    \x01\x12\x04\x9a\x04\r\x11\n\r\n\x05\x04\x0c\x05\0\x02\x12\x04\x9a\x04\
    \x15\x18\n\x0c\n\x02\x04\r\x12\x06\x9d\x04\0\xae\x04\x01\n\x0b\n\x03\x04\
    \r\x01\x12\x04\x9d\x04\x08\x13\n`\n\x04\x04\r\x02\0\x12\x04\xa1\x04\x02\
    \x20\x1aR\x20Set\x20this\x20option\x20to\x20true\x20to\x20allow\x20mappi\
    ng\x20different\x20tag\x20names\x20to\x20the\x20same\n\x20value.\n\n\r\n\
    \x05\x04\r\x02\0\x04\x12\x04\xa1\x04\x02\n\n\r\n\x05\x04\r\x02\0\x05\x12\
    \x04\xa1\x04\x0b\x0f\n\r\n\x05\x04\r\x02\0\x01\x12\x04\xa1\x04\x10\x1b\n\
    \r\n\x05\x04\r\x02\0\x03\x12\x04\xa1\x04\x1e\x1f\n\xe5\x01\n\x04\x04\r\
    \x02\x01\x12\x04\xa7\x04\x02/\x1a\xd6\x01\x20Is\x20this\x20enum\x20depre\
    cated?\n\x20Depending\x20on\x20the\x20target\x20platform,\x20this\x20can\
    \x20emit\x20Deprecated\x20annotations\n\x20for\x20the\x20enum,\x20or\x20\
    it\x20will\x20be\x20completely\x20ignored;\x20in\x20the\x20very\x20least\
    ,\x20this\n\x20is\x20a\x20formalization\x20for\x20deprecating\x20enums.\
    \n\n\r\n\x05\x04\r\x02\x01\x04\x12\x04\xa7\x04\x02\n\n\r\n\x05\x04\r\x02\
    \x01\x05\x12\x04\xa7\x04\x0b\x0f\n\r\n\x05\x04\r\x02\x01\x01\x12\x04\xa7\
    \x04\x10\x1a\n\r\n\x05\x04\r\x02\x01\x03\x12\x04\xa7\x04\x1d\x1e\n\r\n\
    \x05\x04\r\x02\x01\x08\x12\x04\xa7\x04\x1f.\n\r\n\x05\x04\r\x02\x01\x07\
    \x12\x04\xa7\x04(-\nO\n\x04\x04\r\x02\x02\x12\x04\xaa\x04\x02:\x1aA\x20T\
    he\x20parser\x20stores\x20options\x20it\x20doesn't\x20recognize\x20here.\
    \x20See\x20above.\n\n\r\n\x05\x04\r\x02\x02\x04\x12\x04\xaa\x04\x02\n\n\
    \r\n\x05\x04\r\x02\x02\x06\x12\x04\xaa\x04\x0b\x1e\n\r\n\x05\x04\r\x02\
    \x02\x01\x12\x04\xaa\x04\x1f3\n\r\n\x05\x04\r\x02\x02\x03\x12\x04\xaa\
    \x0469\nZ\n\x03\x04\r\x05\x12\x04\xad\x04\x02\x19\x1aM\x20Clients\x20can\
    \x20define\x20custom\x20options\x20in\x20extensions\x20of\x20this\x20mes\
    sage.\x20See\x20above.\n\n\x0c\n\x04\x04\r\x05\0\x12\x04\xad\x04\r\x18\n\
    \r\n\x05\x04\r\x05\0\x01\x12\x04\xad\x04\r\x11\n\r\n\x05\x04\r\x05\0\x02\
    \x12\x04\xad\x04\x15\x18\n\x0c\n\x02\x04\x0e\x12\x06\xb0\x04\0\xbc\x04\
    \x01\n\x0b\n\x03\x04\x0e\x01\x12\x04\xb0\x04\x08\x18\n\xf7\x01\n\x04\x04\
    \x0e\x02\0\x12\x04\xb5\x04\x02/\x1a\xe8\x01\x20Is\x20this\x20enum\x20val\
    ue\x20deprecated?\n\x20Depending\x20on\x20the\x20target\x20platform,\x20\
    this\x20can\x20emit\x20Deprecated\x20annotations\n\x20for\x20the\x20enum\
    \x20value,\x20or\x20it\x20will\x20be\x20completely\x20ignored;\x20in\x20\
    the\x20very\x20least,\n\x20this\x20is\x20a\x20formalization\x20for\x20de\
    precating\x20enum\x20values.\n\n\r\n\x05\x04\x0e\x02\0\x04\x12\x04\xb5\
    \x04\x02\n\n\r\n\x05\x04\x0e\x02\0\x05\x12\x04\xb5\x04\x0b\x0f\n\r\n\x05\
    \x04\x0e\x02\0\x01\x12\x04\xb5\x04\x10\x1a\n\r\n\x05\x04\x0e\x02\0\x03\
    \x12\x04\xb5\x04\x1d\x1e\n\r\n\x05\x04\x0e\x02\0\x08\x12\x04\xb5\x04\x1f\
    .\n\r\n\x05\x04\x0e\x02\0\x07\x12\x04\xb5\x04(-\nO\n\x04\x04\x0e\x02\x01\
    \x12\x04\xb8\x04\x02:\x1aA\x20The\x20parser\x20stores\x20options\x20it\
    \x20doesn't\x20recognize\x20here.\x20See\x20above.\n\n\r\n\x05\x04\x0e\
    \x02\x01\x04\x12\x04\xb8\x04\x02\n\n\r\n\x05\x04\x0e\x02\x01\x06\x12\x04\
    \xb8\x04\x0b\x1e\n\r\n\x05\x04\x0e\x02\x01\x01\x12\x04\xb8\x04\x1f3\n\r\
    \n\x05\x04\x0e\x02\x01\x03\x12\x04\xb8\x0469\nZ\n\x03\x04\x0e\x05\x12\
    \x04\xbb\x04\x02\x19\x1aM\x20Clients\x20can\x20define\x20custom\x20optio\
    ns\x20in\x20extensions\x20of\x20this\x20message.\x20See\x20above.\n\n\
    \x0c\n\x04\x04\x0e\x05\0\x12\x04\xbb\x04\r\x18\n\r\n\x05\x04\x0e\x05\0\
    \x01\x12\x04\xbb\x04\r\x11\n\r\n\x05\x04\x0e\x05\0\x02\x12\x04\xbb\x04\
    \x15\x18\n\x0c\n\x02\x04\x0f\x12\x06\xbe\x04\0\xd0\x04\x01\n\x0b\n\x03\
    \x04\x0f\x01\x12\x04\xbe\x04\x08\x16\n\xd9\x03\n\x04\x04\x0f\x02\0\x12\
    \x04\xc9\x04\x020\x1a\xdf\x01\x20Is\x20this\x20service\x20deprecated?\n\
    \x20Depending\x20on\x20the\x20target\x20platform,\x20this\x20can\x20emit\
    \x20Deprecated\x20annotations\n\x20for\x20the\x20service,\x20or\x20it\
    \x20will\x20be\x20completely\x20ignored;\x20in\x20the\x20very\x20least,\
    \n\x20this\x20is\x20a\x20formalization\x20for\x20deprecating\x20services\
    .\n2\xe8\x01\x20Note:\x20\x20Field\x20numbers\x201\x20through\x2032\x20a\
    re\x20reserved\x20for\x20Google's\x20internal\x20RPC\n\x20\x20\x20framew\
    ork.\x20\x20We\x20apologize\x20for\x20hoarding\x20these\x20numbers\x20to\
    \x20ourselves,\x20but\n\x20\x20\x20we\x20were\x20already\x20using\x20the\
    m\x20long\x20before\x20we\x20decided\x20to\x20release\x20Protocol\n\x20\
    \x20\x20Buffers.\n\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04\xc9\x04\x02\n\n\r\
    \n\x05\x04\x0f\x02\0\x05\x12\x04\xc9\x04\x0b\x0f\n\r\n\x05\x04\x0f\x02\0\
    \x01\x12\x04\xc9\x04\x10\x1a\n\r\n\x05\x04\x0f\x02\0\x03\x12\x04\xc9\x04\
    \x1d\x1f\n\r\n\x05\x04\x0f\x02\0\x08\x12\x04\xc9\x04\x20/\n\r\n\x05\x04\
    \x0f\x02\0\x07\x12\x04\xc9\x04).\nO\n\x04\x04\x0f\x02\x01\x12\x04\xcc\
    \x04\x02:\x1aA\x20The\x20parser\x20stores\x20options\x20it\x20doesn't\
    \x20recognize\x20here.\x20See\x20above.\n\n\r\n\x05\x04\x0f\x02\x01\x04\
    \x12\x04\xcc\x04\x02\n\n\r\n\x05\x04\x0f\x02\x01\x06\x12\x04\xcc\x04\x0b\
    \x1e\n\r\n\x05\x04\x0f\x02\x01\x01\x12\x04\xcc\x04\x1f3\n\r\n\x05\x04\
    \x0f\x02\x01\x03\x12\x04\xcc\x0469\nZ\n\x03\x04\x0f\x05\x12\x04\xcf\x04\
    \x02\x19\x1aM\x20Clients\x20can\x20define\x20custom\x20options\x20in\x20\
    extensions\x20of\x20this\x20message.\x20See\x20above.\n\n\x0c\n\x04\x04\
    \x0f\x05\0\x12\x04\xcf\x04\r\x18\n\r\n\x05\x04\x0f\x05\0\x01\x12\x04\xcf\
    \x04\r\x11\n\r\n\x05\x04\x0f\x05\0\x02\x12\x04\xcf\x04\x15\x18\n\x0c\n\
    \x02\x04\x10\x12\x06\xd2\x04\0\xe4\x04\x01\n\x0b\n\x03\x04\x10\x01\x12\
    \x04\xd2\x04\x08\x15\n\xd6\x03\n\x04\x04\x10\x02\0\x12\x04\xdd\x04\x020\
    \x1a\xdc\x01\x20Is\x20this\x20method\x20deprecated?\n\x20Depending\x20on\
    \x20the\x20target\x20platform,\x20this\x20can\x20emit\x20Deprecated\x20a\
    nnotations\n\x20for\x20the\x20method,\x20or\x20it\x20will\x20be\x20compl\
    etely\x20ignored;\x20in\x20the\x20very\x20least,\n\x20this\x20is\x20a\
    \x20formalization\x20for\x20deprecating\x20methods.\n2\xe8\x01\x20Note:\
    \x20\x20Field\x20numbers\x201\x20through\x2032\x20are\x20reserved\x20for\
    \x20Google's\x20internal\x20RPC\n\x20\x20\x20framework.\x20\x20We\x20apo\
    logize\x20for\x20hoarding\x20these\x20numbers\x20to\x20ourselves,\x20but\
    \n\x20\x20\x20we\x20were\x20already\x20using\x20them\x20long\x20before\
    \x20we\x20decided\x20to\x20release\x20Protocol\n\x20\x20\x20Buffers.\n\n\
    \r\n\x05\x04\x10\x02\0\x04\x12\x04\xdd\x04\x02\n\n\r\n\x05\x04\x10\x02\0\
    \x05\x12\x04\xdd\x04\x0b\x0f\n\r\n\x05\x04\x10\x02\0\x01\x12\x04\xdd\x04\
    \x10\x1a\n\r\n\x05\x04\x10\x02\0\x03\x12\x04\xdd\x04\x1d\x1f\n\r\n\x05\
    \x04\x10\x02\0\x08\x12\x04\xdd\x04\x20/\n\r\n\x05\x04\x10\x02\0\x07\x12\
    \x04\xdd\x04).\nO\n\x04\x04\x10\x02\x01\x12\x04\xe0\x04\x02:\x1aA\x20The\
    \x20parser\x20stores\x20options\x20it\x20doesn't\x20recognize\x20here.\
    \x20See\x20above.\n\n\r\n\x05\x04\x10\x02\x01\x04\x12\x04\xe0\x04\x02\n\
    \n\r\n\x05\x04\x10\x02\x01\x06\x12\x04\xe0\x04\x0b\x1e\n\r\n\x05\x04\x10\
    \x02\x01\x01\x12\x04\xe0\x04\x1f3\n\r\n\x05\x04\x10\x02\x01\x03\x12\x04\
    \xe0\x0469\nZ\n\x03\x04\x10\x05\x12\x04\xe3\x04\x02\x19\x1aM\x20Clients\
    \x20can\x20define\x20custom\x20options\x20in\x20extensions\x20of\x20this\
    \x20message.\x20See\x20above.\n\n\x0c\n\x04\x04\x10\x05\0\x12\x04\xe3\
    \x04\r\x18\n\r\n\x05\x04\x10\x05\0\x01\x12\x04\xe3\x04\r\x11\n\r\n\x05\
    \x04\x10\x05\0\x02\x12\x04\xe3\x04\x15\x18\n\x8b\x03\n\x02\x04\x11\x12\
    \x06\xed\x04\0\x81\x05\x01\x1a\xfc\x02\x20A\x20message\x20representing\
    \x20a\x20option\x20the\x20parser\x20does\x20not\x20recognize.\x20This\
    \x20only\n\x20appears\x20in\x20options\x20protos\x20created\x20by\x20the\
    \x20compiler::Parser\x20class.\n\x20DescriptorPool\x20resolves\x20these\
    \x20when\x20building\x20Descriptor\x20objects.\x20Therefore,\n\x20option\
    s\x20protos\x20in\x20descriptor\x20objects\x20(e.g.\x20returned\x20by\
    \x20Descriptor::options(),\n\x20or\x20produced\x20by\x20Descriptor::Copy\
    To())\x20will\x20never\x20have\x20UninterpretedOptions\n\x20in\x20them.\
    \n\n\x0b\n\x03\x04\x11\x01\x12\x04\xed\x04\x08\x1b\n\xcb\x02\n\x04\x04\
    \x11\x03\0\x12\x06\xf3\x04\x02\xf6\x04\x03\x1a\xba\x02\x20The\x20name\
    \x20of\x20the\x20uninterpreted\x20option.\x20\x20Each\x20string\x20repre\
    sents\x20a\x20segment\x20in\n\x20a\x20dot-separated\x20name.\x20\x20is_e\
    xtension\x20is\x20true\x20iff\x20a\x20segment\x20represents\x20an\n\x20e\
    xtension\x20(denoted\x20with\x20parentheses\x20in\x20options\x20specs\
    \x20in\x20.proto\x20files).\n\x20E.g.,{\x20[\"foo\",\x20false],\x20[\"ba\
    r.baz\",\x20true],\x20[\"qux\",\x20false]\x20}\x20represents\n\x20\"foo.\
    (bar.baz).qux\".\n\n\r\n\x05\x04\x11\x03\0\x01\x12\x04\xf3\x04\n\x12\n\
    \x0e\n\x06\x04\x11\x03\0\x02\0\x12\x04\xf4\x04\x04\"\n\x0f\n\x07\x04\x11\
    \x03\0\x02\0\x04\x12\x04\xf4\x04\x04\x0c\n\x0f\n\x07\x04\x11\x03\0\x02\0\
    \x05\x12\x04\xf4\x04\r\x13\n\x0f\n\x07\x04\x11\x03\0\x02\0\x01\x12\x04\
    \xf4\x04\x14\x1d\n\x0f\n\x07\x04\x11\x03\0\x02\0\x03\x12\x04\xf4\x04\x20\
    !\n\x0e\n\x06\x04\x11\x03\0\x02\x01\x12\x04\xf5\x04\x04#\n\x0f\n\x07\x04\
    \x11\x03\0\x02\x01\x04\x12\x04\xf5\x04\x04\x0c\n\x0f\n\x07\x04\x11\x03\0\
    \x02\x01\x05\x12\x04\xf5\x04\r\x11\n\x0f\n\x07\x04\x11\x03\0\x02\x01\x01\
    \x12\x04\xf5\x04\x12\x1e\n\x0f\n\x07\x04\x11\x03\0\x02\x01\x03\x12\x04\
    \xf5\x04!\"\n\x0c\n\x04\x04\x11\x02\0\x12\x04\xf7\x04\x02\x1d\n\r\n\x05\
    \x04\x11\x02\0\x04\x12\x04\xf7\x04\x02\n\n\r\n\x05\x04\x11\x02\0\x06\x12\
    \x04\xf7\x04\x0b\x13\n\r\n\x05\x04\x11\x02\0\x01\x12\x04\xf7\x04\x14\x18\
    \n\r\n\x05\x04\x11\x02\0\x03\x12\x04\xf7\x04\x1b\x1c\n\x9c\x01\n\x04\x04\
    \x11\x02\x01\x12\x04\xfb\x04\x02'\x1a\x8d\x01\x20The\x20value\x20of\x20t\
    he\x20uninterpreted\x20option,\x20in\x20whatever\x20type\x20the\x20token\
    izer\n\x20identified\x20it\x20as\x20during\x20parsing.\x20Exactly\x20one\
    \x20of\x20these\x20should\x20be\x20set.\n\n\r\n\x05\x04\x11\x02\x01\x04\
    \x12\x04\xfb\x04\x02\n\n\r\n\x05\x04\x11\x02\x01\x05\x12\x04\xfb\x04\x0b\
    \x11\n\r\n\x05\x04\x11\x02\x01\x01\x12\x04\xfb\x04\x12\"\n\r\n\x05\x04\
    \x11\x02\x01\x03\x12\x04\xfb\x04%&\n\x0c\n\x04\x04\x11\x02\x02\x12\x04\
    \xfc\x04\x02)\n\r\n\x05\x04\x11\x02\x02\x04\x12\x04\xfc\x04\x02\n\n\r\n\
    \x05\x04\x11\x02\x02\x05\x12\x04\xfc\x04\x0b\x11\n\r\n\x05\x04\x11\x02\
    \x02\x01\x12\x04\xfc\x04\x12$\n\r\n\x05\x04\x11\x02\x02\x03\x12\x04\xfc\
    \x04'(\n\x0c\n\x04\x04\x11\x02\x03\x12\x04\xfd\x04\x02(\n\r\n\x05\x04\
    \x11\x02\x03\x04\x12\x04\xfd\x04\x02\n\n\r\n\x05\x04\x11\x02\x03\x05\x12\
    \x04\xfd\x04\x0b\x10\n\r\n\x05\x04\x11\x02\x03\x01\x12\x04\xfd\x04\x11#\
    \n\r\n\x05\x04\x11\x02\x03\x03\x12\x04\xfd\x04&'\n\x0c\n\x04\x04\x11\x02\
    \x04\x12\x04\xfe\x04\x02#\n\r\n\x05\x04\x11\x02\x04\x04\x12\x04\xfe\x04\
    \x02\n\n\r\n\x05\x04\x11\x02\x04\x05\x12\x04\xfe\x04\x0b\x11\n\r\n\x05\
    \x04\x11\x02\x04\x01\x12\x04\xfe\x04\x12\x1e\n\r\n\x05\x04\x11\x02\x04\
    \x03\x12\x04\xfe\x04!\"\n\x0c\n\x04\x04\x11\x02\x05\x12\x04\xff\x04\x02\
    \"\n\r\n\x05\x04\x11\x02\x05\x04\x12\x04\xff\x04\x02\n\n\r\n\x05\x04\x11\
    \x02\x05\x05\x12\x04\xff\x04\x0b\x10\n\r\n\x05\x04\x11\x02\x05\x01\x12\
    \x04\xff\x04\x11\x1d\n\r\n\x05\x04\x11\x02\x05\x03\x12\x04\xff\x04\x20!\
    \n\x0c\n\x04\x04\x11\x02\x06\x12\x04\x80\x05\x02&\n\r\n\x05\x04\x11\x02\
    \x06\x04\x12\x04\x80\x05\x02\n\n\r\n\x05\x04\x11\x02\x06\x05\x12\x04\x80\
    \x05\x0b\x11\n\r\n\x05\x04\x11\x02\x06\x01\x12\x04\x80\x05\x12!\n\r\n\
    \x05\x04\x11\x02\x06\x03\x12\x04\x80\x05$%\n\xda\x01\n\x02\x04\x12\x12\
    \x06\x88\x05\0\x89\x06\x01\x1aj\x20Encapsulates\x20information\x20about\
    \x20the\x20original\x20source\x20file\x20from\x20which\x20a\n\x20FileDes\
    criptorProto\x20was\x20generated.\n2`\x20===============================\
    ====================================\n\x20Optional\x20source\x20code\x20\
    info\n\n\x0b\n\x03\x04\x12\x01\x12\x04\x88\x05\x08\x16\n\x82\x11\n\x04\
    \x04\x12\x02\0\x12\x04\xb4\x05\x02!\x1a\xf3\x10\x20A\x20Location\x20iden\
    tifies\x20a\x20piece\x20of\x20source\x20code\x20in\x20a\x20.proto\x20fil\
    e\x20which\n\x20corresponds\x20to\x20a\x20particular\x20definition.\x20\
    \x20This\x20information\x20is\x20intended\n\x20to\x20be\x20useful\x20to\
    \x20IDEs,\x20code\x20indexers,\x20documentation\x20generators,\x20and\
    \x20similar\n\x20tools.\n\n\x20For\x20example,\x20say\x20we\x20have\x20a\
    \x20file\x20like:\n\x20\x20\x20message\x20Foo\x20{\n\x20\x20\x20\x20\x20\
    optional\x20string\x20foo\x20=\x201;\n\x20\x20\x20}\n\x20Let's\x20look\
    \x20at\x20just\x20the\x20field\x20definition:\n\x20\x20\x20optional\x20s\
    tring\x20foo\x20=\x201;\n\x20\x20\x20^\x20\x20\x20\x20\x20\x20\x20^^\x20\
    \x20\x20\x20\x20^^\x20\x20^\x20\x20^^^\n\x20\x20\x20a\x20\x20\x20\x20\
    \x20\x20\x20bc\x20\x20\x20\x20\x20de\x20\x20f\x20\x20ghi\n\x20We\x20have\
    \x20the\x20following\x20locations:\n\x20\x20\x20span\x20\x20\x20path\x20\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20represents\n\x20\
    \x20\x20[a,i)\x20\x20[\x204,\x200,\x202,\x200\x20]\x20\x20\x20\x20\x20Th\
    e\x20whole\x20field\x20definition.\n\x20\x20\x20[a,b)\x20\x20[\x204,\x20\
    0,\x202,\x200,\x204\x20]\x20\x20The\x20label\x20(optional).\n\x20\x20\
    \x20[c,d)\x20\x20[\x204,\x200,\x202,\x200,\x205\x20]\x20\x20The\x20type\
    \x20(string).\n\x20\x20\x20[e,f)\x20\x20[\x204,\x200,\x202,\x200,\x201\
    \x20]\x20\x20The\x20name\x20(foo).\n\x20\x20\x20[g,h)\x20\x20[\x204,\x20\
    0,\x202,\x200,\x203\x20]\x20\x20The\x20number\x20(1).\n\n\x20Notes:\n\
    \x20-\x20A\x20location\x20may\x20refer\x20to\x20a\x20repeated\x20field\
    \x20itself\x20(i.e.\x20not\x20to\x20any\n\x20\x20\x20particular\x20index\
    \x20within\x20it).\x20\x20This\x20is\x20used\x20whenever\x20a\x20set\x20\
    of\x20elements\x20are\n\x20\x20\x20logically\x20enclosed\x20in\x20a\x20s\
    ingle\x20code\x20segment.\x20\x20For\x20example,\x20an\x20entire\n\x20\
    \x20\x20extend\x20block\x20(possibly\x20containing\x20multiple\x20extens\
    ion\x20definitions)\x20will\n\x20\x20\x20have\x20an\x20outer\x20location\
    \x20whose\x20path\x20refers\x20to\x20the\x20\"extensions\"\x20repeated\n\
    \x20\x20\x20field\x20without\x20an\x20index.\n\x20-\x20Multiple\x20locat\
    ions\x20may\x20have\x20the\x20same\x20path.\x20\x20This\x20happens\x20wh\
    en\x20a\x20single\n\x20\x20\x20logical\x20declaration\x20is\x20spread\
    \x20out\x20across\x20multiple\x20places.\x20\x20The\x20most\n\x20\x20\
    \x20obvious\x20example\x20is\x20the\x20\"extend\"\x20block\x20again\x20-\
    -\x20there\x20may\x20be\x20multiple\n\x20\x20\x20extend\x20blocks\x20in\
    \x20the\x20same\x20scope,\x20each\x20of\x20which\x20will\x20have\x20the\
    \x20same\x20path.\n\x20-\x20A\x20location's\x20span\x20is\x20not\x20alwa\
    ys\x20a\x20subset\x20of\x20its\x20parent's\x20span.\x20\x20For\n\x20\x20\
    \x20example,\x20the\x20\"extendee\"\x20of\x20an\x20extension\x20declarat\
    ion\x20appears\x20at\x20the\n\x20\x20\x20beginning\x20of\x20the\x20\"ext\
    end\"\x20block\x20and\x20is\x20shared\x20by\x20all\x20extensions\x20with\
    in\n\x20\x20\x20the\x20block.\n\x20-\x20Just\x20because\x20a\x20location\
    's\x20span\x20is\x20a\x20subset\x20of\x20some\x20other\x20location's\x20\
    span\n\x20\x20\x20does\x20not\x20mean\x20that\x20it\x20is\x20a\x20descen\
    dent.\x20\x20For\x20example,\x20a\x20\"group\"\x20defines\n\x20\x20\x20b\
    oth\x20a\x20type\x20and\x20a\x20field\x20in\x20a\x20single\x20declaratio\
    n.\x20\x20Thus,\x20the\x20locations\n\x20\x20\x20corresponding\x20to\x20\
    the\x20type\x20and\x20field\x20and\x20their\x20components\x20will\x20ove\
    rlap.\n\x20-\x20Code\x20which\x20tries\x20to\x20interpret\x20locations\
    \x20should\x20probably\x20be\x20designed\x20to\n\x20\x20\x20ignore\x20th\
    ose\x20that\x20it\x20doesn't\x20understand,\x20as\x20more\x20types\x20of\
    \x20locations\x20could\n\x20\x20\x20be\x20recorded\x20in\x20the\x20futur\
    e.\n\n\r\n\x05\x04\x12\x02\0\x04\x12\x04\xb4\x05\x02\n\n\r\n\x05\x04\x12\
    \x02\0\x06\x12\x04\xb4\x05\x0b\x13\n\r\n\x05\x04\x12\x02\0\x01\x12\x04\
    \xb4\x05\x14\x1c\n\r\n\x05\x04\x12\x02\0\x03\x12\x04\xb4\x05\x1f\x20\n\
    \x0e\n\x04\x04\x12\x03\0\x12\x06\xb5\x05\x02\x88\x06\x03\n\r\n\x05\x04\
    \x12\x03\0\x01\x12\x04\xb5\x05\n\x12\n\x83\x07\n\x06\x04\x12\x03\0\x02\0\
    \x12\x04\xcd\x05\x04*\x1a\xf2\x06\x20Identifies\x20which\x20part\x20of\
    \x20the\x20FileDescriptorProto\x20was\x20defined\x20at\x20this\n\x20loca\
    tion.\n\n\x20Each\x20element\x20is\x20a\x20field\x20number\x20or\x20an\
    \x20index.\x20\x20They\x20form\x20a\x20path\x20from\n\x20the\x20root\x20\
    FileDescriptorProto\x20to\x20the\x20place\x20where\x20the\x20definition.\
    \x20\x20For\n\x20example,\x20this\x20path:\n\x20\x20\x20[\x204,\x203,\
    \x202,\x207,\x201\x20]\n\x20refers\x20to:\n\x20\x20\x20file.message_type\
    (3)\x20\x20//\x204,\x203\n\x20\x20\x20\x20\x20\x20\x20.field(7)\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20//\x202,\x207\n\x20\x20\x20\x20\x20\x20\x20.\
    name()\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20//\x201\n\x20This\x20i\
    s\x20because\x20FileDescriptorProto.message_type\x20has\x20field\x20numb\
    er\x204:\n\x20\x20\x20repeated\x20DescriptorProto\x20message_type\x20=\
    \x204;\n\x20and\x20DescriptorProto.field\x20has\x20field\x20number\x202:\
    \n\x20\x20\x20repeated\x20FieldDescriptorProto\x20field\x20=\x202;\n\x20\
    and\x20FieldDescriptorProto.name\x20has\x20field\x20number\x201:\n\x20\
    \x20\x20optional\x20string\x20name\x20=\x201;\n\n\x20Thus,\x20the\x20abo\
    ve\x20path\x20gives\x20the\x20location\x20of\x20a\x20field\x20name.\x20\
    \x20If\x20we\x20removed\n\x20the\x20last\x20element:\n\x20\x20\x20[\x204\
    ,\x203,\x202,\x207\x20]\n\x20this\x20path\x20refers\x20to\x20the\x20whol\
    e\x20field\x20declaration\x20(from\x20the\x20beginning\n\x20of\x20the\
    \x20label\x20to\x20the\x20terminating\x20semicolon).\n\n\x0f\n\x07\x04\
    \x12\x03\0\x02\0\x04\x12\x04\xcd\x05\x04\x0c\n\x0f\n\x07\x04\x12\x03\0\
    \x02\0\x05\x12\x04\xcd\x05\r\x12\n\x0f\n\x07\x04\x12\x03\0\x02\0\x01\x12\
    \x04\xcd\x05\x13\x17\n\x0f\n\x07\x04\x12\x03\0\x02\0\x03\x12\x04\xcd\x05\
    \x1a\x1b\n\x0f\n\x07\x04\x12\x03\0\x02\0\x08\x12\x04\xcd\x05\x1c)\n\x12\
    \n\n\x04\x12\x03\0\x02\0\x08\xe7\x07\0\x12\x04\xcd\x05\x1d(\n\x13\n\x0b\
    \x04\x12\x03\0\x02\0\x08\xe7\x07\0\x02\x12\x04\xcd\x05\x1d#\n\x14\n\x0c\
    \x04\x12\x03\0\x02\0\x08\xe7\x07\0\x02\0\x12\x04\xcd\x05\x1d#\n\x15\n\r\
    \x04\x12\x03\0\x02\0\x08\xe7\x07\0\x02\0\x01\x12\x04\xcd\x05\x1d#\n\x13\
    \n\x0b\x04\x12\x03\0\x02\0\x08\xe7\x07\0\x03\x12\x04\xcd\x05$(\n\xd2\x02\
    \n\x06\x04\x12\x03\0\x02\x01\x12\x04\xd4\x05\x04*\x1a\xc1\x02\x20Always\
    \x20has\x20exactly\x20three\x20or\x20four\x20elements:\x20start\x20line,\
    \x20start\x20column,\n\x20end\x20line\x20(optional,\x20otherwise\x20assu\
    med\x20same\x20as\x20start\x20line),\x20end\x20column.\n\x20These\x20are\
    \x20packed\x20into\x20a\x20single\x20field\x20for\x20efficiency.\x20\x20\
    Note\x20that\x20line\n\x20and\x20column\x20numbers\x20are\x20zero-based\
    \x20--\x20typically\x20you\x20will\x20want\x20to\x20add\n\x201\x20to\x20\
    each\x20before\x20displaying\x20to\x20a\x20user.\n\n\x0f\n\x07\x04\x12\
    \x03\0\x02\x01\x04\x12\x04\xd4\x05\x04\x0c\n\x0f\n\x07\x04\x12\x03\0\x02\
    \x01\x05\x12\x04\xd4\x05\r\x12\n\x0f\n\x07\x04\x12\x03\0\x02\x01\x01\x12\
    \x04\xd4\x05\x13\x17\n\x0f\n\x07\x04\x12\x03\0\x02\x01\x03\x12\x04\xd4\
    \x05\x1a\x1b\n\x0f\n\x07\x04\x12\x03\0\x02\x01\x08\x12\x04\xd4\x05\x1c)\
    \n\x12\n\n\x04\x12\x03\0\x02\x01\x08\xe7\x07\0\x12\x04\xd4\x05\x1d(\n\
    \x13\n\x0b\x04\x12\x03\0\x02\x01\x08\xe7\x07\0\x02\x12\x04\xd4\x05\x1d#\
    \n\x14\n\x0c\x04\x12\x03\0\x02\x01\x08\xe7\x07\0\x02\0\x12\x04\xd4\x05\
    \x1d#\n\x15\n\r\x04\x12\x03\0\x02\x01\x08\xe7\x07\0\x02\0\x01\x12\x04\
    \xd4\x05\x1d#\n\x13\n\x0b\x04\x12\x03\0\x02\x01\x08\xe7\x07\0\x03\x12\
    \x04\xd4\x05$(\n\xa5\x0c\n\x06\x04\x12\x03\0\x02\x02\x12\x04\x85\x06\x04\
    )\x1a\x94\x0c\x20If\x20this\x20SourceCodeInfo\x20represents\x20a\x20comp\
    lete\x20declaration,\x20these\x20are\x20any\n\x20comments\x20appearing\
    \x20before\x20and\x20after\x20the\x20declaration\x20which\x20appear\x20t\
    o\x20be\n\x20attached\x20to\x20the\x20declaration.\n\n\x20A\x20series\
    \x20of\x20line\x20comments\x20appearing\x20on\x20consecutive\x20lines,\
    \x20with\x20no\x20other\n\x20tokens\x20appearing\x20on\x20those\x20lines\
    ,\x20will\x20be\x20treated\x20as\x20a\x20single\x20comment.\n\n\x20leadi\
    ng_detached_comments\x20will\x20keep\x20paragraphs\x20of\x20comments\x20\
    that\x20appear\n\x20before\x20(but\x20not\x20connected\x20to)\x20the\x20\
    current\x20element.\x20Each\x20paragraph,\n\x20separated\x20by\x20empty\
    \x20lines,\x20will\x20be\x20one\x20comment\x20element\x20in\x20the\x20re\
    peated\n\x20field.\n\n\x20Only\x20the\x20comment\x20content\x20is\x20pro\
    vided;\x20comment\x20markers\x20(e.g.\x20//)\x20are\n\x20stripped\x20out\
    .\x20\x20For\x20block\x20comments,\x20leading\x20whitespace\x20and\x20an\
    \x20asterisk\n\x20will\x20be\x20stripped\x20from\x20the\x20beginning\x20\
    of\x20each\x20line\x20other\x20than\x20the\x20first.\n\x20Newlines\x20ar\
    e\x20included\x20in\x20the\x20output.\n\n\x20Examples:\n\n\x20\x20\x20op\
    tional\x20int32\x20foo\x20=\x201;\x20\x20//\x20Comment\x20attached\x20to\
    \x20foo.\n\x20\x20\x20//\x20Comment\x20attached\x20to\x20bar.\n\x20\x20\
    \x20optional\x20int32\x20bar\x20=\x202;\n\n\x20\x20\x20optional\x20strin\
    g\x20baz\x20=\x203;\n\x20\x20\x20//\x20Comment\x20attached\x20to\x20baz.\
    \n\x20\x20\x20//\x20Another\x20line\x20attached\x20to\x20baz.\n\n\x20\
    \x20\x20//\x20Comment\x20attached\x20to\x20qux.\n\x20\x20\x20//\n\x20\
    \x20\x20//\x20Another\x20line\x20attached\x20to\x20qux.\n\x20\x20\x20opt\
    ional\x20double\x20qux\x20=\x204;\n\n\x20\x20\x20//\x20Detached\x20comme\
    nt\x20for\x20corge.\x20This\x20is\x20not\x20leading\x20or\x20trailing\
    \x20comments\n\x20\x20\x20//\x20to\x20qux\x20or\x20corge\x20because\x20t\
    here\x20are\x20blank\x20lines\x20separating\x20it\x20from\n\x20\x20\x20/\
    /\x20both.\n\n\x20\x20\x20//\x20Detached\x20comment\x20for\x20corge\x20p\
    aragraph\x202.\n\n\x20\x20\x20optional\x20string\x20corge\x20=\x205;\n\
    \x20\x20\x20/*\x20Block\x20comment\x20attached\n\x20\x20\x20\x20*\x20to\
    \x20corge.\x20\x20Leading\x20asterisks\n\x20\x20\x20\x20*\x20will\x20be\
    \x20removed.\x20*/\n\x20\x20\x20/*\x20Block\x20comment\x20attached\x20to\
    \n\x20\x20\x20\x20*\x20grault.\x20*/\n\x20\x20\x20optional\x20int32\x20g\
    rault\x20=\x206;\n\n\x20\x20\x20//\x20ignored\x20detached\x20comments.\n\
    \n\x0f\n\x07\x04\x12\x03\0\x02\x02\x04\x12\x04\x85\x06\x04\x0c\n\x0f\n\
    \x07\x04\x12\x03\0\x02\x02\x05\x12\x04\x85\x06\r\x13\n\x0f\n\x07\x04\x12\
    \x03\0\x02\x02\x01\x12\x04\x85\x06\x14$\n\x0f\n\x07\x04\x12\x03\0\x02\
    \x02\x03\x12\x04\x85\x06'(\n\x0e\n\x06\x04\x12\x03\0\x02\x03\x12\x04\x86\
    \x06\x04*\n\x0f\n\x07\x04\x12\x03\0\x02\x03\x04\x12\x04\x86\x06\x04\x0c\
    \n\x0f\n\x07\x04\x12\x03\0\x02\x03\x05\x12\x04\x86\x06\r\x13\n\x0f\n\x07\
    \x04\x12\x03\0\x02\x03\x01\x12\x04\x86\x06\x14%\n\x0f\n\x07\x04\x12\x03\
    \0\x02\x03\x03\x12\x04\x86\x06()\n\x0e\n\x06\x04\x12\x03\0\x02\x04\x12\
    \x04\x87\x06\x042\n\x0f\n\x07\x04\x12\x03\0\x02\x04\x04\x12\x04\x87\x06\
    \x04\x0c\n\x0f\n\x07\x04\x12\x03\0\x02\x04\x05\x12\x04\x87\x06\r\x13\n\
    \x0f\n\x07\x04\x12\x03\0\x02\x04\x01\x12\x04\x87\x06\x14-\n\x0f\n\x07\
    \x04\x12\x03\0\x02\x04\x03\x12\x04\x87\x0601\n\xee\x01\n\x02\x04\x13\x12\
    \x06\x8e\x06\0\xa3\x06\x01\x1a\xdf\x01\x20Describes\x20the\x20relationsh\
    ip\x20between\x20generated\x20code\x20and\x20its\x20original\x20source\n\
    \x20file.\x20A\x20GeneratedCodeInfo\x20message\x20is\x20associated\x20wi\
    th\x20only\x20one\x20generated\n\x20source\x20file,\x20but\x20may\x20con\
    tain\x20references\x20to\x20different\x20source\x20.proto\x20files.\n\n\
    \x0b\n\x03\x04\x13\x01\x12\x04\x8e\x06\x08\x19\nx\n\x04\x04\x13\x02\0\
    \x12\x04\x91\x06\x02%\x1aj\x20An\x20Annotation\x20connects\x20some\x20sp\
    an\x20of\x20text\x20in\x20generated\x20code\x20to\x20an\x20element\n\x20\
    of\x20its\x20generating\x20.proto\x20file.\n\n\r\n\x05\x04\x13\x02\0\x04\
    \x12\x04\x91\x06\x02\n\n\r\n\x05\x04\x13\x02\0\x06\x12\x04\x91\x06\x0b\
    \x15\n\r\n\x05\x04\x13\x02\0\x01\x12\x04\x91\x06\x16\x20\n\r\n\x05\x04\
    \x13\x02\0\x03\x12\x04\x91\x06#$\n\x0e\n\x04\x04\x13\x03\0\x12\x06\x92\
    \x06\x02\xa2\x06\x03\n\r\n\x05\x04\x13\x03\0\x01\x12\x04\x92\x06\n\x14\n\
    \x8f\x01\n\x06\x04\x13\x03\0\x02\0\x12\x04\x95\x06\x04*\x1a\x7f\x20Ident\
    ifies\x20the\x20element\x20in\x20the\x20original\x20source\x20.proto\x20\
    file.\x20This\x20field\n\x20is\x20formatted\x20the\x20same\x20as\x20Sour\
    ceCodeInfo.Location.path.\n\n\x0f\n\x07\x04\x13\x03\0\x02\0\x04\x12\x04\
    \x95\x06\x04\x0c\n\x0f\n\x07\x04\x13\x03\0\x02\0\x05\x12\x04\x95\x06\r\
    \x12\n\x0f\n\x07\x04\x13\x03\0\x02\0\x01\x12\x04\x95\x06\x13\x17\n\x0f\n\
    \x07\x04\x13\x03\0\x02\0\x03\x12\x04\x95\x06\x1a\x1b\n\x0f\n\x07\x04\x13\
    \x03\0\x02\0\x08\x12\x04\x95\x06\x1c)\n\x12\n\n\x04\x13\x03\0\x02\0\x08\
    \xe7\x07\0\x12\x04\x95\x06\x1d(\n\x13\n\x0b\x04\x13\x03\0\x02\0\x08\xe7\
    \x07\0\x02\x12\x04\x95\x06\x1d#\n\x14\n\x0c\x04\x13\x03\0\x02\0\x08\xe7\
    \x07\0\x02\0\x12\x04\x95\x06\x1d#\n\x15\n\r\x04\x13\x03\0\x02\0\x08\xe7\
    \x07\0\x02\0\x01\x12\x04\x95\x06\x1d#\n\x13\n\x0b\x04\x13\x03\0\x02\0\
    \x08\xe7\x07\0\x03\x12\x04\x95\x06$(\nO\n\x06\x04\x13\x03\0\x02\x01\x12\
    \x04\x98\x06\x04$\x1a?\x20Identifies\x20the\x20filesystem\x20path\x20to\
    \x20the\x20original\x20source\x20.proto.\n\n\x0f\n\x07\x04\x13\x03\0\x02\
    \x01\x04\x12\x04\x98\x06\x04\x0c\n\x0f\n\x07\x04\x13\x03\0\x02\x01\x05\
    \x12\x04\x98\x06\r\x13\n\x0f\n\x07\x04\x13\x03\0\x02\x01\x01\x12\x04\x98\
    \x06\x14\x1f\n\x0f\n\x07\x04\x13\x03\0\x02\x01\x03\x12\x04\x98\x06\"#\nw\
    \n\x06\x04\x13\x03\0\x02\x02\x12\x04\x9c\x06\x04\x1d\x1ag\x20Identifies\
    \x20the\x20starting\x20offset\x20in\x20bytes\x20in\x20the\x20generated\
    \x20code\n\x20that\x20relates\x20to\x20the\x20identified\x20object.\n\n\
    \x0f\n\x07\x04\x13\x03\0\x02\x02\x04\x12\x04\x9c\x06\x04\x0c\n\x0f\n\x07\
    \x04\x13\x03\0\x02\x02\x05\x12\x04\x9c\x06\r\x12\n\x0f\n\x07\x04\x13\x03\
    \0\x02\x02\x01\x12\x04\x9c\x06\x13\x18\n\x0f\n\x07\x04\x13\x03\0\x02\x02\
    \x03\x12\x04\x9c\x06\x1b\x1c\n\xdb\x01\n\x06\x04\x13\x03\0\x02\x03\x12\
    \x04\xa1\x06\x04\x1b\x1a\xca\x01\x20Identifies\x20the\x20ending\x20offse\
    t\x20in\x20bytes\x20in\x20the\x20generated\x20code\x20that\n\x20relates\
    \x20to\x20the\x20identified\x20offset.\x20The\x20end\x20offset\x20should\
    \x20be\x20one\x20past\n\x20the\x20last\x20relevant\x20byte\x20(so\x20the\
    \x20length\x20of\x20the\x20text\x20=\x20end\x20-\x20begin).\n\n\x0f\n\
    \x07\x04\x13\x03\0\x02\x03\x04\x12\x04\xa1\x06\x04\x0c\n\x0f\n\x07\x04\
    \x13\x03\0\x02\x03\x05\x12\x04\xa1\x06\r\x12\n\x0f\n\x07\x04\x13\x03\0\
    \x02\x03\x01\x12\x04\xa1\x06\x13\x16\n\x0f\n\x07\x04\x13\x03\0\x02\x03\
    \x03\x12\x04\xa1\x06\x19\x1a\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}