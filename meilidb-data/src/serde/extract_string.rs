use serde::Serialize;
use serde::ser;

use super::SerializerError;

pub struct ExtractString;

impl ser::Serializer for ExtractString {
    type Ok = String;
    type Error = SerializerError;
    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    forward_to_unserializable_type! {
        bool => serialize_bool,
        char => serialize_char,

        i8  => serialize_i8,
        i16 => serialize_i16,
        i32 => serialize_i32,
        i64 => serialize_i64,

        u8  => serialize_u8,
        u16 => serialize_u16,
        u32 => serialize_u32,
        u64 => serialize_u64,

        f32 => serialize_f32,
        f64 => serialize_f64,
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError::UnserializableType { name: "&[u8]" })
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError::UnserializableType { name: "Option" })
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where T: Serialize,
    {
        Err(SerializerError::UnserializableType { name: "Option" })
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError::UnserializableType { name: "()" })
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError::UnserializableType { name: "unit struct" })
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str
    ) -> Result<Self::Ok, Self::Error>
    {
        Err(SerializerError::UnserializableType { name: "unit variant" })
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T
    ) -> Result<Self::Ok, Self::Error>
    where T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T
    ) -> Result<Self::Ok, Self::Error>
    where T: Serialize,
    {
        Err(SerializerError::UnserializableType { name: "newtype variant" })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(SerializerError::UnserializableType { name: "sequence" })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(SerializerError::UnserializableType { name: "tuple" })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize
    ) -> Result<Self::SerializeTupleStruct, Self::Error>
    {
        Err(SerializerError::UnserializableType { name: "tuple struct" })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize
    ) -> Result<Self::SerializeTupleVariant, Self::Error>
    {
        Err(SerializerError::UnserializableType { name: "tuple variant" })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(SerializerError::UnserializableType { name: "map" })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize
    ) -> Result<Self::SerializeStruct, Self::Error>
    {
        Err(SerializerError::UnserializableType { name: "struct" })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize
    ) -> Result<Self::SerializeStructVariant, Self::Error>
    {
        Err(SerializerError::UnserializableType { name: "struct variant" })
    }
}
