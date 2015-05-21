extern crate protobuf;
extern crate serde;

use serde::Serialize;

pub struct Proto<'a>(pub &'a protobuf::Message);

struct ProtoField<'a>(&'a protobuf::Message, &'a protobuf::reflect::FieldDescriptor);
struct ProtoEnum<'a>(&'a protobuf::reflect::EnumValueDescriptor);

impl<'a> serde::Serialize for Proto<'a> {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
        use serde::ser::impls::MapIteratorVisitor;
        let &Proto(message) = self;
        let fields = message.descriptor().fields();
        let len = fields.len();
        let iter = fields.iter().map(|f| (f.name(), ProtoField(message, f)));
        let visitor = MapIteratorVisitor::new(iter, Option::Some(len));
        serializer.visit_map(visitor)
    }
}

impl<'a> serde::Serialize for ProtoField<'a> {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
        use protobuf::descriptor::FieldDescriptorProto_Type as Type;

        let &ProtoField(m, f) = self;

        if f.is_repeated() {
            use serde::ser::impls::SeqIteratorVisitor;

            let len = f.len_field(m);
            let len_iter = 0..len;
            let some_len = Option::Some(len);

            match f.proto().get_field_type() {
                Type::TYPE_MESSAGE => {
                    let iter = len_iter.map(|i| Proto(f.get_rep_message_item(m, i)));
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_ENUM => {
                    let iter = len_iter.map(|i| ProtoEnum(f.get_rep_enum_item(m, i)));
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_STRING => {
                    let iter = len_iter.map(|i| f.get_rep_str_item(m, i));
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_BYTES => {
                    let iter = len_iter.map(|i| f.get_rep_bytes_item(m, i));
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_INT32 |
                Type::TYPE_SINT32 |
                Type::TYPE_SFIXED32 => {
                    let iter = len_iter.map(|i| f.get_rep_i32(m)[i]);
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_INT64 |
                Type::TYPE_SINT64 |
                Type::TYPE_SFIXED64 => {
                    let iter = len_iter.map(|i| f.get_rep_i64(m)[i]);
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_UINT32 |
                Type::TYPE_FIXED32 => {
                    let iter = len_iter.map(|i| f.get_rep_u32(m)[i]);
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_UINT64 |
                Type::TYPE_FIXED64 => {
                    let iter = len_iter.map(|i| f.get_rep_u64(m)[i]);
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_BOOL => {
                    let iter = len_iter.map(|i| f.get_rep_bool(m)[i]);
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_FLOAT => {
                    let iter = len_iter.map(|i| f.get_rep_f32(m)[i]);
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_DOUBLE => {
                    let iter = len_iter.map(|i| f.get_rep_f64(m)[i]);
                    serializer.visit_seq(SeqIteratorVisitor::new(iter, some_len))
                },
                Type::TYPE_GROUP => panic!("Protobuf groups not supported"),
            }
        } else {
            match f.proto().get_field_type() {
                Type::TYPE_MESSAGE =>
                    Proto(f.get_message(m)).serialize(serializer),
                Type::TYPE_ENUM =>
                    ProtoEnum(f.get_enum(m)).serialize(serializer),
                Type::TYPE_STRING =>
                    f.get_str(m).serialize(serializer),
                Type::TYPE_BYTES =>
                    f.get_bytes(m).serialize(serializer),
                Type::TYPE_INT32 |
                Type::TYPE_SINT32 |
                Type::TYPE_SFIXED32 =>
                    f.get_i32(m).serialize(serializer),
                Type::TYPE_INT64 |
                Type::TYPE_SINT64 |
                Type::TYPE_SFIXED64 =>
                    f.get_i64(m).serialize(serializer),
                Type::TYPE_UINT32 |
                Type::TYPE_FIXED32 =>
                    f.get_u32(m).serialize(serializer),
                Type::TYPE_UINT64 |
                Type::TYPE_FIXED64 =>
                    f.get_u64(m).serialize(serializer),
                Type::TYPE_BOOL =>
                    f.get_bool(m).serialize(serializer),
                Type::TYPE_FLOAT =>
                    f.get_f32(m).serialize(serializer),
                Type::TYPE_DOUBLE =>
                    f.get_f64(m).serialize(serializer),
                Type::TYPE_GROUP =>
                    panic!("Protobuf groups not supported"),
            }
        }
    }
}

impl<'a> serde::Serialize for ProtoEnum<'a> {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: serde::Serializer {
        let &ProtoEnum(e) = self;
        serializer.visit_enum_unit("enum", e.name())
    }
}
