use crate::customize::Customize;
use crate::field::rust_field_name_for_protobuf_field_name;
use crate::file::proto_path_to_rust_mod;
use crate::file_and_mod::FileAndMod;
use crate::message::message_name_to_nested_mod_name;
use crate::protobuf_name::ProtobufAbsolutePath;
use crate::protobuf_name::ProtobufIdent;
use crate::protobuf_name::ProtobufRelativePath;
use crate::rust;
use crate::rust::is_rust_keyword;
use crate::rust_name::RustIdent;
use crate::rust_name::RustIdentWithPath;
use crate::rust_name::RustRelativePath;
use crate::strx::capitalize;
use crate::syntax::Syntax;
use protobuf::descriptor::DescriptorProto;
use protobuf::descriptor::EnumDescriptorProto;
use protobuf::descriptor::EnumValueDescriptorProto;
use protobuf::descriptor::FieldDescriptorProto;
use protobuf::descriptor::FileDescriptorProto;
use protobuf::descriptor::OneofDescriptorProto;

pub(crate) struct RootScope<'a> {
    pub file_descriptors: &'a [FileDescriptorProto],
}

impl<'a> RootScope<'a> {
    fn packages(&'a self) -> Vec<FileScope<'a>> {
        self.file_descriptors
            .iter()
            .map(|fd| FileScope {
                file_descriptor: fd,
            })
            .collect()
    }

    // find enum by fully qualified name
    pub fn _find_enum(&'a self, fqn: &ProtobufAbsolutePath) -> EnumWithScope<'a> {
        match self.find_message_or_enum(fqn) {
            MessageOrEnumWithScope::Enum(e) => e,
            _ => panic!("not an enum: {}", fqn),
        }
    }

    // find message by fully qualified name
    pub fn _find_message(&'a self, fqn: &ProtobufAbsolutePath) -> MessageWithScope<'a> {
        match self.find_message_or_enum(fqn) {
            MessageOrEnumWithScope::Message(m) => m,
            _ => panic!("not a message: {}", fqn),
        }
    }

    // find message or enum by fully qualified name
    pub fn find_message_or_enum(
        &'a self,
        fqn: &ProtobufAbsolutePath,
    ) -> MessageOrEnumWithScope<'a> {
        assert!(!fqn.is_empty());
        self.packages()
            .into_iter()
            .flat_map(|p| p.find_message_or_enum_abs(fqn))
            .next()
            .expect(&format!("enum not found by name: {}", fqn))
    }
}

#[derive(Clone, Debug)]
pub(crate) struct FileScope<'a> {
    pub file_descriptor: &'a FileDescriptorProto,
}

impl<'a> FileScope<'a> {
    fn get_package(&self) -> ProtobufAbsolutePath {
        ProtobufRelativePath::from(self.file_descriptor.get_package()).into_absolute()
    }

    pub fn syntax(&self) -> Syntax {
        Syntax::parse(self.file_descriptor.get_syntax())
    }

    pub fn to_scope(&self) -> Scope<'a> {
        Scope {
            file_scope: self.clone(),
            path: Vec::new(),
        }
    }

    fn find_message_or_enum(
        &self,
        name: &ProtobufRelativePath,
    ) -> Option<MessageOrEnumWithScope<'a>> {
        self.find_messages_and_enums()
            .into_iter()
            .filter(|e| e.protobuf_name_to_package() == *name)
            .next()
    }

    fn find_message_or_enum_abs(
        &self,
        name: &ProtobufAbsolutePath,
    ) -> Option<MessageOrEnumWithScope<'a>> {
        match name.remove_prefix(&self.get_package()) {
            Some(ref rem) => self.find_message_or_enum(rem),
            None => None,
        }
    }

    // find all enums in given file descriptor
    pub fn _find_enums(&self) -> Vec<EnumWithScope<'a>> {
        let mut r = Vec::new();

        self.to_scope().walk_scopes(|scope| {
            r.extend(scope.get_enums());
        });

        r
    }

    // find all messages in given file descriptor
    pub fn _find_messages(&self) -> Vec<MessageWithScope<'a>> {
        let mut r = Vec::new();

        self.to_scope().walk_scopes(|scope| {
            r.extend(scope.get_messages());
        });

        r
    }

    // find all messages and enums in given file descriptor
    pub fn find_messages_and_enums(&self) -> Vec<MessageOrEnumWithScope<'a>> {
        let mut r = Vec::new();

        self.to_scope().walk_scopes(|scope| {
            r.extend(scope.get_messages_and_enums());
        });

        r
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Scope<'a> {
    pub file_scope: FileScope<'a>,
    pub path: Vec<&'a DescriptorProto>,
}

impl<'a> Scope<'a> {
    pub fn get_file_descriptor(&self) -> &'a FileDescriptorProto {
        self.file_scope.file_descriptor
    }

    // get message descriptors in this scope
    fn get_message_descriptors(&self) -> &'a [DescriptorProto] {
        if self.path.is_empty() {
            &self.file_scope.file_descriptor.message_type
        } else {
            &self.path.last().unwrap().nested_type
        }
    }

    // get enum descriptors in this scope
    fn get_enum_descriptors(&self) -> &'a [EnumDescriptorProto] {
        if self.path.is_empty() {
            &self.file_scope.file_descriptor.enum_type
        } else {
            &self.path.last().unwrap().enum_type
        }
    }

    // get messages with attached scopes in this scope
    pub fn get_messages(&self) -> Vec<MessageWithScope<'a>> {
        self.get_message_descriptors()
            .iter()
            .map(|m| MessageWithScope {
                scope: self.clone(),
                message: m,
            })
            .collect()
    }

    // get enums with attached scopes in this scope
    pub fn get_enums(&self) -> Vec<EnumWithScope<'a>> {
        self.get_enum_descriptors()
            .iter()
            .map(|e| EnumWithScope {
                scope: self.clone(),
                en: e,
            })
            .collect()
    }

    // get messages and enums with attached scopes in this scope
    pub fn get_messages_and_enums(&self) -> Vec<MessageOrEnumWithScope<'a>> {
        self.get_messages()
            .into_iter()
            .map(|m| MessageOrEnumWithScope::Message(m))
            .chain(
                self.get_enums()
                    .into_iter()
                    .map(|m| MessageOrEnumWithScope::Enum(m)),
            )
            .collect()
    }

    // nested scopes, i. e. scopes of nested messages
    fn nested_scopes(&self) -> Vec<Scope<'a>> {
        self.get_message_descriptors()
            .iter()
            .map(|m| {
                let mut nested = self.clone();
                nested.path.push(m);
                nested
            })
            .collect()
    }

    fn walk_scopes_impl<F: FnMut(&Scope<'a>)>(&self, callback: &mut F) {
        (*callback)(self);

        for nested in self.nested_scopes() {
            nested.walk_scopes_impl(callback);
        }
    }

    // apply callback for this scope and all nested scopes
    fn walk_scopes<F>(&self, mut callback: F)
    where
        F: FnMut(&Scope<'a>),
    {
        self.walk_scopes_impl(&mut callback);
    }

    pub fn rust_path_to_file(&self) -> RustRelativePath {
        RustRelativePath::from_components(
            self.path
                .iter()
                .map(|m| message_name_to_nested_mod_name(m.get_name())),
        )
    }

    pub fn path_str(&self) -> String {
        let v: Vec<&str> = self.path.iter().map(|m| m.get_name()).collect();
        v.join(".")
    }

    pub fn prefix(&self) -> String {
        let path_str = self.path_str();
        if path_str.is_empty() {
            path_str
        } else {
            format!("{}.", path_str)
        }
    }

    pub fn protobuf_path_to_file(&self) -> ProtobufRelativePath {
        ProtobufRelativePath::from_components(
            self.path.iter().map(|m| ProtobufIdent::from(m.get_name())),
        )
    }

    pub fn protobuf_absolute_path(&self) -> ProtobufAbsolutePath {
        let mut r = self.file_scope.get_package();
        r.push_relative(&self.protobuf_path_to_file());
        r
    }

    pub fn get_file_and_mod(&self, customize: Customize) -> FileAndMod {
        FileAndMod {
            file: self.file_scope.file_descriptor.get_name().to_owned(),
            relative_mod: self.rust_path_to_file(),
            customize,
        }
    }
}

pub(crate) trait WithScope<'a> {
    fn get_scope(&self) -> &Scope<'a>;

    fn get_file_descriptor(&self) -> &'a FileDescriptorProto {
        self.get_scope().get_file_descriptor()
    }

    // message or enum name
    fn get_name(&self) -> ProtobufIdent;

    fn escape_prefix(&self) -> &'static str;

    fn name_to_package(&self) -> String {
        let mut r = self.get_scope().prefix();
        r.push_str(self.get_name().get());
        r
    }

    fn protobuf_name_to_package(&self) -> ProtobufRelativePath {
        let r = self.get_scope().protobuf_path_to_file();
        r.append_ident(&ProtobufIdent::from(self.get_name()))
    }

    /// Return absolute name starting with dot
    fn name_absolute(&self) -> ProtobufAbsolutePath {
        let mut path = self.get_scope().protobuf_absolute_path();
        path.push_simple(self.get_name());
        path
    }

    // rust type name of this descriptor
    fn rust_name(&self) -> RustIdent {
        let mut rust_name = capitalize(self.get_name().get());

        if is_rust_keyword(&rust_name) {
            rust_name.insert_str(0, self.escape_prefix());
        }

        RustIdent::new(&rust_name)
    }

    fn rust_name_to_file(&self) -> RustIdentWithPath {
        self.get_scope()
            .rust_path_to_file()
            .into_path()
            .with_ident(self.rust_name())
    }

    // fully-qualified name of this type
    fn rust_name_with_file(&self) -> RustIdentWithPath {
        let mut r = self.rust_name_to_file();
        r.prepend_ident(proto_path_to_rust_mod(
            self.get_scope().get_file_descriptor().get_name(),
        ));
        r
    }
}

#[derive(Clone, Debug)]
pub(crate) struct MessageWithScope<'a> {
    pub scope: Scope<'a>,
    pub message: &'a DescriptorProto,
}

impl<'a> WithScope<'a> for MessageWithScope<'a> {
    fn get_scope(&self) -> &Scope<'a> {
        &self.scope
    }

    fn escape_prefix(&self) -> &'static str {
        "message_"
    }

    fn get_name(&self) -> ProtobufIdent {
        ProtobufIdent::from(self.message.get_name())
    }
}

impl<'a> MessageWithScope<'a> {
    pub fn into_scope(mut self) -> Scope<'a> {
        self.scope.path.push(self.message);
        self.scope
    }

    pub fn to_scope(&self) -> Scope<'a> {
        self.clone().into_scope()
    }

    pub fn fields(&self) -> Vec<FieldWithContext<'a>> {
        self.message
            .field
            .iter()
            .map(|f| FieldWithContext {
                field: f,
                message: self.clone(),
            })
            .collect()
    }

    pub fn oneofs(&self) -> Vec<OneofWithContext<'a>> {
        self.message
            .oneof_decl
            .iter()
            .enumerate()
            .map(|(index, oneof)| OneofWithContext {
                message: self.clone(),
                oneof: oneof,
                index: index as u32,
            })
            .collect()
    }

    pub fn oneof_by_index(&self, index: u32) -> OneofWithContext<'a> {
        self.oneofs().swap_remove(index as usize)
    }

    pub fn mod_name(&self) -> RustIdent {
        message_name_to_nested_mod_name(self.message.get_name())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct EnumWithScope<'a> {
    pub scope: Scope<'a>,
    pub en: &'a EnumDescriptorProto,
}

impl<'a> EnumWithScope<'a> {
    pub fn values(&self) -> Vec<EnumValueWithContext<'a>> {
        self.en
            .value
            .iter()
            .map(|v| EnumValueWithContext {
                en: self.clone(),
                proto: v,
            })
            .collect()
    }

    // find enum value by protobuf name
    pub fn value_by_name(&self, name: &str) -> EnumValueWithContext<'a> {
        self.values()
            .into_iter()
            .find(|v| v.proto.get_name() == name)
            .unwrap()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct EnumValueWithContext<'a> {
    pub en: EnumWithScope<'a>,
    pub proto: &'a EnumValueDescriptorProto,
}

impl<'a> EnumValueWithContext<'a> {
    pub fn rust_name(&self) -> RustIdent {
        let mut r = String::new();
        if rust::is_rust_keyword(self.proto.get_name()) {
            r.push_str("value_");
        }
        r.push_str(self.proto.get_name());
        RustIdent::new(&r)
    }
}

impl<'a> WithScope<'a> for EnumWithScope<'a> {
    fn get_scope(&self) -> &Scope<'a> {
        &self.scope
    }

    fn get_name(&self) -> ProtobufIdent {
        ProtobufIdent::from(self.en.get_name())
    }

    fn escape_prefix(&self) -> &'static str {
        "enum_"
    }
}

pub(crate) enum MessageOrEnumWithScope<'a> {
    Message(MessageWithScope<'a>),
    Enum(EnumWithScope<'a>),
}

impl<'a> WithScope<'a> for MessageOrEnumWithScope<'a> {
    fn get_scope(&self) -> &Scope<'a> {
        match self {
            &MessageOrEnumWithScope::Message(ref m) => m.get_scope(),
            &MessageOrEnumWithScope::Enum(ref e) => e.get_scope(),
        }
    }

    fn escape_prefix(&self) -> &'static str {
        match self {
            &MessageOrEnumWithScope::Message(ref m) => m.escape_prefix(),
            &MessageOrEnumWithScope::Enum(ref e) => e.escape_prefix(),
        }
    }

    fn get_name(&self) -> ProtobufIdent {
        match self {
            &MessageOrEnumWithScope::Message(ref m) => m.get_name(),
            &MessageOrEnumWithScope::Enum(ref e) => e.get_name(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct FieldWithContext<'a> {
    pub field: &'a FieldDescriptorProto,
    pub message: MessageWithScope<'a>,
}

impl<'a> FieldWithContext<'a> {
    pub fn is_oneof(&self) -> bool {
        self.field.has_oneof_index()
    }

    pub fn oneof(&self) -> Option<OneofWithContext<'a>> {
        if self.is_oneof() {
            Some(
                self.message
                    .oneof_by_index(self.field.get_oneof_index() as u32),
            )
        } else {
            None
        }
    }

    pub fn number(&self) -> u32 {
        self.field.get_number() as u32
    }

    pub fn rust_name(&self) -> RustIdent {
        rust_field_name_for_protobuf_field_name(self.name())
    }

    /// Shortcut
    pub fn name(&self) -> &str {
        self.field.get_name()
    }

    // From field to file root
    pub fn _containing_messages(&self) -> Vec<&'a DescriptorProto> {
        let mut r = Vec::new();
        r.push(self.message.message);
        r.extend(self.message.scope.path.iter().rev());
        r
    }
}

#[derive(Clone)]
pub(crate) struct OneofVariantWithContext<'a> {
    pub oneof: &'a OneofWithContext<'a>,
    pub field: &'a FieldDescriptorProto,
}

#[derive(Clone)]
pub(crate) struct OneofWithContext<'a> {
    pub oneof: &'a OneofDescriptorProto,
    pub index: u32,
    pub message: MessageWithScope<'a>,
}

impl<'a> OneofWithContext<'a> {
    pub fn field_name(&'a self) -> RustIdent {
        return rust_field_name_for_protobuf_field_name(self.oneof.get_name());
    }

    // rust type name of enum
    pub fn rust_name(&self) -> RustIdentWithPath {
        // TODO: escape name
        let type_name = RustIdent::from(capitalize(self.oneof.get_name()));
        self.message
            .to_scope()
            .rust_path_to_file()
            .into_path()
            .with_ident(type_name)
    }

    pub fn variants(&'a self) -> Vec<OneofVariantWithContext<'a>> {
        self.message
            .fields()
            .iter()
            .filter(|f| f.field.has_oneof_index() && f.field.get_oneof_index() == self.index as i32)
            .map(|f| OneofVariantWithContext {
                oneof: self,
                field: &f.field,
            })
            .collect()
    }
}
