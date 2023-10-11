use byteorder::{WriteBytesExt, LE};
use serde::ser::{
    self, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use std::io::{self, Write};

pub struct Serializer<W> {
    pub w: W,
}

#[derive(Debug, thiserror::Error)]
pub enum SerializerError {
    #[error("The Serde type {0} is unsupported")]
    UnsupportedType(&'static str),

    #[error("{0}")]
    Custom(String),

    #[error("IO error")]
    Io(#[from] io::Error),
}

impl ser::Error for SerializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        SerializerError::Custom(msg.to_string())
    }
}

impl<'a, W> ser::Serializer for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();

    type Error = SerializerError;

    type SerializeSeq = Self;

    type SerializeTuple = Self;

    type SerializeTupleStruct = Self;

    type SerializeTupleVariant = Self;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.w.write_u8(if v { 1 } else { 0 })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.w.write_i8(v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.w.write_i16::<LE>(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.w.write_i32::<LE>(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.w.write_i64::<LE>(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.w.write_u8(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.w.write_u16::<LE>(v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.w.write_u32::<LE>(v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.w.write_u64::<LE>(v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.w.write_f32::<LE>(v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.w.write_f64::<LE>(v)?;
        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError::UnsupportedType("char"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if v.len() == 0 {
            self.w.write_u32::<LE>(0)?;
            return Ok(());
        }
        self.w.write_u32::<LE>(v.len() as u32 + 1)?;
        for byte in v.as_bytes() {
            self.w.write_u8(*byte)?;
        }
        self.w.write_u8(0)?;
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError::UnsupportedType("bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(SerializerError::UnsupportedType("none"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(SerializerError::UnsupportedType("some"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(value.serialize(self)?)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        Err(SerializerError::UnsupportedType("newtype variant"))
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let real_len = len.ok_or_else(|| SerializerError::UnsupportedType("unsized seq"))?;
        self.w.write_u32::<LE>(real_len as u32)?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(SerializerError::UnsupportedType("tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self.serialize_map(Some(len))?)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(SerializerError::UnsupportedType("struct variant"))
    }
}

impl<'a, W> SerializeSeq for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(value.serialize(&mut **self)?)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeTuple for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();

    type Error = SerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl<'a, W> SerializeTupleStruct for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();

    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl<'a, W> SerializeTupleVariant for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();

    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
impl<'a, W> SerializeMap for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();

    type Error = SerializerError;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeStruct for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();
    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, W> SerializeStructVariant for &'a mut Serializer<W>
where
    W: Write,
{
    type Ok = ();

    type Error = SerializerError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        <Self as SerializeSeq>::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
