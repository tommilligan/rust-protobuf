// This file is generated by rust-protobuf 3.0.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
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
#![allow(unused_imports)]
#![allow(unused_results)]

//! Generated file from `rustproto.proto`

/// Extension fields
pub mod exts {

    pub const expose_oneof_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const repeated_field_vec_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17020, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };

    pub const serde_derive_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const lite_runtime_all: crate::ext::ExtFieldOptional<crate::descriptor::FileOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17035, phantom: ::std::marker::PhantomData };

    pub const expose_oneof: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17001, phantom: ::std::marker::PhantomData };

    pub const expose_fields: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const repeated_field_vec: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17020, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };

    pub const serde_derive: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17030, phantom: ::std::marker::PhantomData };

    pub const serde_derive_cfg: crate::ext::ExtFieldOptional<crate::descriptor::MessageOptions, crate::reflect::types::ProtobufTypeString> = crate::ext::ExtFieldOptional { field_number: 17031, phantom: ::std::marker::PhantomData };

    pub const expose_fields_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17003, phantom: ::std::marker::PhantomData };

    pub const generate_accessors_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17004, phantom: ::std::marker::PhantomData };

    pub const generate_getter_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17005, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_bytes_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17011, phantom: ::std::marker::PhantomData };

    pub const carllerche_bytes_for_string_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17012, phantom: ::std::marker::PhantomData };

    pub const repeated_field_vec_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17020, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_box_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17024, phantom: ::std::marker::PhantomData };

    pub const singular_field_option_field: crate::ext::ExtFieldOptional<crate::descriptor::FieldOptions, crate::reflect::types::ProtobufTypeBool> = crate::ext::ExtFieldOptional { field_number: 17025, phantom: ::std::marker::PhantomData };
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0frustproto.proto\x12\trustproto\x1a\x20google/protobuf/descriptor.p\
    roto:H\n\x10expose_oneof_all\x18\xe9\x84\x01\x20\x01(\x08\x12\x1c.google\
    .protobuf.FileOptionsR\x0eexposeOneofAll:J\n\x11expose_fields_all\x18\
    \xeb\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0fexpose\
    FieldsAll:T\n\x16generate_accessors_all\x18\xec\x84\x01\x20\x01(\x08\x12\
    \x1c.google.protobuf.FileOptionsR\x14generateAccessorsAll:N\n\x13generat\
    e_getter_all\x18\xed\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOp\
    tionsR\x11generateGetterAll:b\n\x1ecarllerche_bytes_for_bytes_all\x18\
    \xf3\x84\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1acarlle\
    rcheBytesForBytesAll:d\n\x1fcarllerche_bytes_for_string_all\x18\xf4\x84\
    \x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x1bcarllercheByte\
    sForStringAll:S\n\x16repeated_field_vec_all\x18\xfc\x84\x01\x20\x01(\x08\
    \x12\x1c.google.protobuf.FileOptionsR\x13repeatedFieldVecAll:`\n\x1dsing\
    ular_field_option_box_all\x18\x80\x85\x01\x20\x01(\x08\x12\x1c.google.pr\
    otobuf.FileOptionsR\x19singularFieldOptionBoxAll:Y\n\x19singular_field_o\
    ption_all\x18\x81\x85\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptio\
    nsR\x16singularFieldOptionAll:H\n\x10serde_derive_all\x18\x86\x85\x01\
    \x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0eserdeDeriveAll:O\n\
    \x14serde_derive_cfg_all\x18\x87\x85\x01\x20\x01(\t\x12\x1c.google.proto\
    buf.FileOptionsR\x11serdeDeriveCfgAll:H\n\x10lite_runtime_all\x18\x8b\
    \x85\x01\x20\x01(\x08\x12\x1c.google.protobuf.FileOptionsR\x0eliteRuntim\
    eAll:D\n\x0cexpose_oneof\x18\xe9\x84\x01\x20\x01(\x08\x12\x1f.google.pro\
    tobuf.MessageOptionsR\x0bexposeOneof:F\n\rexpose_fields\x18\xeb\x84\x01\
    \x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0cexposeFields:P\
    \n\x12generate_accessors\x18\xec\x84\x01\x20\x01(\x08\x12\x1f.google.pro\
    tobuf.MessageOptionsR\x11generateAccessors:J\n\x0fgenerate_getter\x18\
    \xed\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x0egen\
    erateGetter:^\n\x1acarllerche_bytes_for_bytes\x18\xf3\x84\x01\x20\x01(\
    \x08\x12\x1f.google.protobuf.MessageOptionsR\x17carllercheBytesForBytes:\
    `\n\x1bcarllerche_bytes_for_string\x18\xf4\x84\x01\x20\x01(\x08\x12\x1f.\
    google.protobuf.MessageOptionsR\x18carllercheBytesForString:O\n\x12repea\
    ted_field_vec\x18\xfc\x84\x01\x20\x01(\x08\x12\x1f.google.protobuf.Messa\
    geOptionsR\x10repeatedFieldVec:\\\n\x19singular_field_option_box\x18\x80\
    \x85\x01\x20\x01(\x08\x12\x1f.google.protobuf.MessageOptionsR\x16singula\
    rFieldOptionBox:U\n\x15singular_field_option\x18\x81\x85\x01\x20\x01(\
    \x08\x12\x1f.google.protobuf.MessageOptionsR\x13singularFieldOption:D\n\
    \x0cserde_derive\x18\x86\x85\x01\x20\x01(\x08\x12\x1f.google.protobuf.Me\
    ssageOptionsR\x0bserdeDerive:K\n\x10serde_derive_cfg\x18\x87\x85\x01\x20\
    \x01(\t\x12\x1f.google.protobuf.MessageOptionsR\x0eserdeDeriveCfg:O\n\
    \x13expose_fields_field\x18\xeb\x84\x01\x20\x01(\x08\x12\x1d.google.prot\
    obuf.FieldOptionsR\x11exposeFieldsField:Y\n\x18generate_accessors_field\
    \x18\xec\x84\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x16g\
    enerateAccessorsField:S\n\x15generate_getter_field\x18\xed\x84\x01\x20\
    \x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x13generateGetterField:g\
    \n\x20carllerche_bytes_for_bytes_field\x18\xf3\x84\x01\x20\x01(\x08\x12\
    \x1d.google.protobuf.FieldOptionsR\x1ccarllercheBytesForBytesField:i\n!c\
    arllerche_bytes_for_string_field\x18\xf4\x84\x01\x20\x01(\x08\x12\x1d.go\
    ogle.protobuf.FieldOptionsR\x1dcarllercheBytesForStringField:X\n\x18repe\
    ated_field_vec_field\x18\xfc\x84\x01\x20\x01(\x08\x12\x1d.google.protobu\
    f.FieldOptionsR\x15repeatedFieldVecField:e\n\x1fsingular_field_option_bo\
    x_field\x18\x80\x85\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOption\
    sR\x1bsingularFieldOptionBoxField:^\n\x1bsingular_field_option_field\x18\
    \x81\x85\x01\x20\x01(\x08\x12\x1d.google.protobuf.FieldOptionsR\x18singu\
    larFieldOptionFieldJ\x94#\n\x06\x12\x04\0\0V\x01\n\x08\n\x01\x0c\x12\x03\
    \0\0\x12\n\t\n\x02\x03\0\x12\x03\x02\x07)\n\xe5\x01\n\x01\x02\x12\x03\n\
    \x08\x112^\x20see\x20https://github.com/gogo/protobuf/blob/master/gogopr\
    oto/gogo.proto\n\x20for\x20the\x20original\x20idea\n2{\x20Generated\x20f\
    iles\x20can\x20be\x20customized\x20using\x20this\x20proto\n\x20or\x20usi\
    ng\x20`Customize`\x20struct\x20when\x20codegen\x20is\x20invoked\x20progr\
    ammatically.\n\n\t\n\x01\x07\x12\x04\x0c\0(\x01\n7\n\x02\x07\0\x12\x03\
    \x0e\x04+\x1a,\x20When\x20true,\x20oneof\x20field\x20is\x20generated\x20\
    public\n\n\n\n\x03\x07\0\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\0\x04\x12\
    \x03\x0e\x04\x0c\n\n\n\x03\x07\0\x05\x12\x03\x0e\r\x11\n\n\n\x03\x07\0\
    \x01\x12\x03\x0e\x12\"\n\n\n\x03\x07\0\x03\x12\x03\x0e%*\nI\n\x02\x07\
    \x01\x12\x03\x10\x04,\x1a>\x20When\x20true\x20all\x20fields\x20are\x20pu\
    blic,\x20and\x20not\x20accessors\x20generated\n\n\n\n\x03\x07\x01\x02\
    \x12\x03\x0c\x07\"\n\n\n\x03\x07\x01\x04\x12\x03\x10\x04\x0c\n\n\n\x03\
    \x07\x01\x05\x12\x03\x10\r\x11\n\n\n\x03\x07\x01\x01\x12\x03\x10\x12#\n\
    \n\n\x03\x07\x01\x03\x12\x03\x10&+\nP\n\x02\x07\x02\x12\x03\x12\x041\x1a\
    E\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20accessors\
    \x20are\x20not\x20generated\n\n\n\n\x03\x07\x02\x02\x12\x03\x0c\x07\"\n\
    \n\n\x03\x07\x02\x04\x12\x03\x12\x04\x0c\n\n\n\x03\x07\x02\x05\x12\x03\
    \x12\r\x11\n\n\n\x03\x07\x02\x01\x12\x03\x12\x12(\n\n\n\x03\x07\x02\x03\
    \x12\x03\x12+0\nL\n\x02\x07\x03\x12\x03\x14\x04.\x1aA\x20When\x20false,\
    \x20`get_`\x20is\x20not\x20generated\x20even\x20if\x20`syntax\x20=\x20\"\
    proto2\"`\n\n\n\n\x03\x07\x03\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x03\
    \x04\x12\x03\x14\x04\x0c\n\n\n\x03\x07\x03\x05\x12\x03\x14\r\x11\n\n\n\
    \x03\x07\x03\x01\x12\x03\x14\x12%\n\n\n\x03\x07\x03\x03\x12\x03\x14(-\n2\
    \n\x02\x07\x04\x12\x03\x16\x049\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20\
    `bytes`\x20fields\n\n\n\n\x03\x07\x04\x02\x12\x03\x0c\x07\"\n\n\n\x03\
    \x07\x04\x04\x12\x03\x16\x04\x0c\n\n\n\x03\x07\x04\x05\x12\x03\x16\r\x11\
    \n\n\n\x03\x07\x04\x01\x12\x03\x16\x120\n\n\n\x03\x07\x04\x03\x12\x03\
    \x1638\n3\n\x02\x07\x05\x12\x03\x18\x04:\x1a(\x20Use\x20`bytes::Bytes`\
    \x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x05\x02\x12\x03\x0c\x07\"\
    \n\n\n\x03\x07\x05\x04\x12\x03\x18\x04\x0c\n\n\n\x03\x07\x05\x05\x12\x03\
    \x18\r\x11\n\n\n\x03\x07\x05\x01\x12\x03\x18\x121\n\n\n\x03\x07\x05\x03\
    \x12\x03\x1849\n=\n\x02\x07\x06\x12\x03\x1a\x041\x1a2\x20Use\x20`std::Ve\
    c`\x20to\x20store\x20repeated\x20messages\x20fields\n\n\n\n\x03\x07\x06\
    \x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\x06\x04\x12\x03\x1a\x04\x0c\n\n\n\
    \x03\x07\x06\x05\x12\x03\x1a\r\x11\n\n\n\x03\x07\x06\x01\x12\x03\x1a\x12\
    (\n\n\n\x03\x07\x06\x03\x12\x03\x1a+0\nM\n\x02\x07\x07\x12\x03\x1c\x048\
    \x1aB\x20Use\x20`std::Option<std::Box<T>>`\x20to\x20store\x20singular\
    \x20messages\x20fields\n\n\n\n\x03\x07\x07\x02\x12\x03\x0c\x07\"\n\n\n\
    \x03\x07\x07\x04\x12\x03\x1c\x04\x0c\n\n\n\x03\x07\x07\x05\x12\x03\x1c\r\
    \x11\n\n\n\x03\x07\x07\x01\x12\x03\x1c\x12/\n\n\n\x03\x07\x07\x03\x12\
    \x03\x1c27\n\x93\x01\n\x02\x07\x08\x12\x03\x1f\x044\x1a\x87\x01\x20Use\
    \x20`std::Option<T>`\x20to\x20store\x20singular\x20messages\x20fields.\n\
    \x20Note,\x20it's\x20not\x20possible\x20to\x20have\x20recursive\x20messa\
    ges\x20with\x20this\x20option\x20enabled.\n\n\n\n\x03\x07\x08\x02\x12\
    \x03\x0c\x07\"\n\n\n\x03\x07\x08\x04\x12\x03\x1f\x04\x0c\n\n\n\x03\x07\
    \x08\x05\x12\x03\x1f\r\x11\n\n\n\x03\x07\x08\x01\x12\x03\x1f\x12+\n\n\n\
    \x03\x07\x08\x03\x12\x03\x1f.3\nJ\n\x02\x07\t\x12\x03\"\x04+\x1a?\x20Use\
    \x20`serde_derive`\x20to\x20implement\x20`Serialize`\x20and\x20`Deserial\
    ize`\n\n\n\n\x03\x07\t\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\t\x04\x12\x03\
    \"\x04\x0c\n\n\n\x03\x07\t\x05\x12\x03\"\r\x11\n\n\n\x03\x07\t\x01\x12\
    \x03\"\x12\"\n\n\n\x03\x07\t\x03\x12\x03\"%*\n3\n\x02\x07\n\x12\x03$\x04\
    1\x1a(\x20Guard\x20serde\x20annotations\x20with\x20cfg\x20attr.\n\n\n\n\
    \x03\x07\n\x02\x12\x03\x0c\x07\"\n\n\n\x03\x07\n\x04\x12\x03$\x04\x0c\n\
    \n\n\x03\x07\n\x05\x12\x03$\r\x13\n\n\n\x03\x07\n\x01\x12\x03$\x14(\n\n\
    \n\x03\x07\n\x03\x12\x03$+0\nN\n\x02\x07\x0b\x12\x03'\x04+\x1aC\x20When\
    \x20true,\x20will\x20only\x20generate\x20codes\x20that\x20works\x20with\
    \x20lite\x20runtime.\n\n\n\n\x03\x07\x0b\x02\x12\x03\x0c\x07\"\n\n\n\x03\
    \x07\x0b\x04\x12\x03'\x04\x0c\n\n\n\x03\x07\x0b\x05\x12\x03'\r\x11\n\n\n\
    \x03\x07\x0b\x01\x12\x03'\x12\"\n\n\n\x03\x07\x0b\x03\x12\x03'%*\n\t\n\
    \x01\x07\x12\x04*\0B\x01\n7\n\x02\x07\x0c\x12\x03,\x04'\x1a,\x20When\x20\
    true,\x20oneof\x20field\x20is\x20generated\x20public\n\n\n\n\x03\x07\x0c\
    \x02\x12\x03*\x07%\n\n\n\x03\x07\x0c\x04\x12\x03,\x04\x0c\n\n\n\x03\x07\
    \x0c\x05\x12\x03,\r\x11\n\n\n\x03\x07\x0c\x01\x12\x03,\x12\x1e\n\n\n\x03\
    \x07\x0c\x03\x12\x03,!&\nI\n\x02\x07\r\x12\x03.\x04(\x1a>\x20When\x20tru\
    e\x20all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\x20gener\
    ated\n\n\n\n\x03\x07\r\x02\x12\x03*\x07%\n\n\n\x03\x07\r\x04\x12\x03.\
    \x04\x0c\n\n\n\x03\x07\r\x05\x12\x03.\r\x11\n\n\n\x03\x07\r\x01\x12\x03.\
    \x12\x1f\n\n\n\x03\x07\r\x03\x12\x03.\"'\nP\n\x02\x07\x0e\x12\x030\x04-\
    \x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20acces\
    sors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x0e\x02\x12\x03*\x07%\n\
    \n\n\x03\x07\x0e\x04\x12\x030\x04\x0c\n\n\n\x03\x07\x0e\x05\x12\x030\r\
    \x11\n\n\n\x03\x07\x0e\x01\x12\x030\x12$\n\n\n\x03\x07\x0e\x03\x12\x030'\
    ,\nL\n\x02\x07\x0f\x12\x032\x04*\x1aA\x20When\x20false,\x20`get_`\x20is\
    \x20not\x20generated\x20even\x20if\x20`syntax\x20=\x20\"proto2\"`\n\n\n\
    \n\x03\x07\x0f\x02\x12\x03*\x07%\n\n\n\x03\x07\x0f\x04\x12\x032\x04\x0c\
    \n\n\n\x03\x07\x0f\x05\x12\x032\r\x11\n\n\n\x03\x07\x0f\x01\x12\x032\x12\
    !\n\n\n\x03\x07\x0f\x03\x12\x032$)\n2\n\x02\x07\x10\x12\x034\x045\x1a'\
    \x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\x03\x07\
    \x10\x02\x12\x03*\x07%\n\n\n\x03\x07\x10\x04\x12\x034\x04\x0c\n\n\n\x03\
    \x07\x10\x05\x12\x034\r\x11\n\n\n\x03\x07\x10\x01\x12\x034\x12,\n\n\n\
    \x03\x07\x10\x03\x12\x034/4\n3\n\x02\x07\x11\x12\x036\x046\x1a(\x20Use\
    \x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\x11\x02\
    \x12\x03*\x07%\n\n\n\x03\x07\x11\x04\x12\x036\x04\x0c\n\n\n\x03\x07\x11\
    \x05\x12\x036\r\x11\n\n\n\x03\x07\x11\x01\x12\x036\x12-\n\n\n\x03\x07\
    \x11\x03\x12\x03605\n<\n\x02\x07\x12\x12\x038\x04-\x1a1\x20Use\x20`std::\
    Vec`\x20to\x20store\x20repeated\x20messages\x20field\n\n\n\n\x03\x07\x12\
    \x02\x12\x03*\x07%\n\n\n\x03\x07\x12\x04\x12\x038\x04\x0c\n\n\n\x03\x07\
    \x12\x05\x12\x038\r\x11\n\n\n\x03\x07\x12\x01\x12\x038\x12$\n\n\n\x03\
    \x07\x12\x03\x12\x038',\nM\n\x02\x07\x13\x12\x03:\x044\x1aB\x20Use\x20`s\
    td::Option<std::Box<T>>`\x20to\x20store\x20singular\x20messages\x20field\
    s\n\n\n\n\x03\x07\x13\x02\x12\x03*\x07%\n\n\n\x03\x07\x13\x04\x12\x03:\
    \x04\x0c\n\n\n\x03\x07\x13\x05\x12\x03:\r\x11\n\n\n\x03\x07\x13\x01\x12\
    \x03:\x12+\n\n\n\x03\x07\x13\x03\x12\x03:.3\n\x93\x01\n\x02\x07\x14\x12\
    \x03=\x040\x1a\x87\x01\x20Use\x20`std::Option<T>`\x20to\x20store\x20sing\
    ular\x20messages\x20fields.\n\x20Note,\x20it's\x20not\x20possible\x20to\
    \x20have\x20recursive\x20messages\x20with\x20this\x20option\x20enabled.\
    \n\n\n\n\x03\x07\x14\x02\x12\x03*\x07%\n\n\n\x03\x07\x14\x04\x12\x03=\
    \x04\x0c\n\n\n\x03\x07\x14\x05\x12\x03=\r\x11\n\n\n\x03\x07\x14\x01\x12\
    \x03=\x12'\n\n\n\x03\x07\x14\x03\x12\x03=*/\nJ\n\x02\x07\x15\x12\x03?\
    \x04'\x1a?\x20Use\x20`serde_derive`\x20to\x20implement\x20`Serialize`\
    \x20and\x20`Deserialize`\n\n\n\n\x03\x07\x15\x02\x12\x03*\x07%\n\n\n\x03\
    \x07\x15\x04\x12\x03?\x04\x0c\n\n\n\x03\x07\x15\x05\x12\x03?\r\x11\n\n\n\
    \x03\x07\x15\x01\x12\x03?\x12\x1e\n\n\n\x03\x07\x15\x03\x12\x03?!&\n3\n\
    \x02\x07\x16\x12\x03A\x04-\x1a(\x20Guard\x20serde\x20annotations\x20with\
    \x20cfg\x20attr.\n\n\n\n\x03\x07\x16\x02\x12\x03*\x07%\n\n\n\x03\x07\x16\
    \x04\x12\x03A\x04\x0c\n\n\n\x03\x07\x16\x05\x12\x03A\r\x13\n\n\n\x03\x07\
    \x16\x01\x12\x03A\x14$\n\n\n\x03\x07\x16\x03\x12\x03A',\n\t\n\x01\x07\
    \x12\x04D\0V\x01\nI\n\x02\x07\x17\x12\x03F\x04.\x1a>\x20When\x20true\x20\
    all\x20fields\x20are\x20public,\x20and\x20not\x20accessors\x20generated\
    \n\n\n\n\x03\x07\x17\x02\x12\x03D\x07#\n\n\n\x03\x07\x17\x04\x12\x03F\
    \x04\x0c\n\n\n\x03\x07\x17\x05\x12\x03F\r\x11\n\n\n\x03\x07\x17\x01\x12\
    \x03F\x12%\n\n\n\x03\x07\x17\x03\x12\x03F(-\nP\n\x02\x07\x18\x12\x03H\
    \x043\x1aE\x20When\x20false,\x20`get_`,\x20`set_`,\x20`mut_`\x20etc.\x20\
    accessors\x20are\x20not\x20generated\n\n\n\n\x03\x07\x18\x02\x12\x03D\
    \x07#\n\n\n\x03\x07\x18\x04\x12\x03H\x04\x0c\n\n\n\x03\x07\x18\x05\x12\
    \x03H\r\x11\n\n\n\x03\x07\x18\x01\x12\x03H\x12*\n\n\n\x03\x07\x18\x03\
    \x12\x03H-2\nL\n\x02\x07\x19\x12\x03J\x040\x1aA\x20When\x20false,\x20`ge\
    t_`\x20is\x20not\x20generated\x20even\x20if\x20`syntax\x20=\x20\"proto2\
    \"`\n\n\n\n\x03\x07\x19\x02\x12\x03D\x07#\n\n\n\x03\x07\x19\x04\x12\x03J\
    \x04\x0c\n\n\n\x03\x07\x19\x05\x12\x03J\r\x11\n\n\n\x03\x07\x19\x01\x12\
    \x03J\x12'\n\n\n\x03\x07\x19\x03\x12\x03J*/\n2\n\x02\x07\x1a\x12\x03L\
    \x04;\x1a'\x20Use\x20`bytes::Bytes`\x20for\x20`bytes`\x20fields\n\n\n\n\
    \x03\x07\x1a\x02\x12\x03D\x07#\n\n\n\x03\x07\x1a\x04\x12\x03L\x04\x0c\n\
    \n\n\x03\x07\x1a\x05\x12\x03L\r\x11\n\n\n\x03\x07\x1a\x01\x12\x03L\x122\
    \n\n\n\x03\x07\x1a\x03\x12\x03L5:\n3\n\x02\x07\x1b\x12\x03N\x04<\x1a(\
    \x20Use\x20`bytes::Bytes`\x20for\x20`string`\x20fields\n\n\n\n\x03\x07\
    \x1b\x02\x12\x03D\x07#\n\n\n\x03\x07\x1b\x04\x12\x03N\x04\x0c\n\n\n\x03\
    \x07\x1b\x05\x12\x03N\r\x11\n\n\n\x03\x07\x1b\x01\x12\x03N\x123\n\n\n\
    \x03\x07\x1b\x03\x12\x03N6;\n<\n\x02\x07\x1c\x12\x03P\x043\x1a1\x20Use\
    \x20`std::Vec`\x20to\x20store\x20repeated\x20messages\x20field\n\n\n\n\
    \x03\x07\x1c\x02\x12\x03D\x07#\n\n\n\x03\x07\x1c\x04\x12\x03P\x04\x0c\n\
    \n\n\x03\x07\x1c\x05\x12\x03P\r\x11\n\n\n\x03\x07\x1c\x01\x12\x03P\x12*\
    \n\n\n\x03\x07\x1c\x03\x12\x03P-2\nM\n\x02\x07\x1d\x12\x03R\x04:\x1aB\
    \x20Use\x20`std::Option<std::Box<T>>`\x20to\x20store\x20singular\x20mess\
    ages\x20fields\n\n\n\n\x03\x07\x1d\x02\x12\x03D\x07#\n\n\n\x03\x07\x1d\
    \x04\x12\x03R\x04\x0c\n\n\n\x03\x07\x1d\x05\x12\x03R\r\x11\n\n\n\x03\x07\
    \x1d\x01\x12\x03R\x121\n\n\n\x03\x07\x1d\x03\x12\x03R49\n\x93\x01\n\x02\
    \x07\x1e\x12\x03U\x046\x1a\x87\x01\x20Use\x20`std::Option<T>`\x20to\x20s\
    tore\x20singular\x20messages\x20fields.\n\x20Note,\x20it's\x20not\x20pos\
    sible\x20to\x20have\x20recursive\x20messages\x20with\x20this\x20option\
    \x20enabled.\n\n\n\n\x03\x07\x1e\x02\x12\x03D\x07#\n\n\n\x03\x07\x1e\x04\
    \x12\x03U\x04\x0c\n\n\n\x03\x07\x1e\x05\x12\x03U\r\x11\n\n\n\x03\x07\x1e\
    \x01\x12\x03U\x12-\n\n\n\x03\x07\x1e\x03\x12\x03U05\
";

static file_descriptor_proto_lazy: crate::rt::Lazy<crate::descriptor::FileDescriptorProto> = crate::rt::Lazy::INIT;

fn parse_descriptor_proto() -> crate::descriptor::FileDescriptorProto {
    crate::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
