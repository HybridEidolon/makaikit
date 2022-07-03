use byteorder::{ReadBytesExt, LE};
use serde::de;
use std::{
    ffi::{CStr, FromBytesWithNulError},
    io::{self, Read},
    str::Utf8Error,
};

pub struct Deserializer<'a> {
    buf: &'a [u8],
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializerError {
    #[error("The type {0} is unsupported")]
    TypeUnsupported(&'static str),

    #[error("Failed to parse C string")]
    CStrParseError(#[from] FromBytesWithNulError),

    #[error("Failed to interpret C string as UTF-8")]
    Utf8Error(#[from] Utf8Error),

    #[error("{0}")]
    Custom(String),

    #[error("IO error")]
    Io(#[from] io::Error),
}

impl de::Error for DeserializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        DeserializerError::Custom(msg.to_string())
    }
}

impl<'a, 'de> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = DeserializerError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("any"))
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("bool"))
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i8(self.buf.read_i8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i16(self.buf.read_i16::<LE>()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i32(self.buf.read_i32::<LE>()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_i64(self.buf.read_i64::<LE>()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u8(self.buf.read_u8()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u16(self.buf.read_u16::<LE>()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u32(self.buf.read_u32::<LE>()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_u64(self.buf.read_u64::<LE>()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f32(self.buf.read_f32::<LE>()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_f64(self.buf.read_f64::<LE>()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("char"))
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("str"))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let len = self.buf.read_u32::<LE>()? as usize;
        let mut text_buf = Vec::with_capacity(len);
        text_buf.resize(len, 0);

        self.buf.read_exact(&mut text_buf[..])?;
        let text = CStr::from_bytes_with_nul(&text_buf[..len])?;
        let string = text.to_str()?.into();
        visitor.visit_string(string)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("bytes"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("byte_buf"))
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("option"))
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("newtype_struct"))
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("seq"))
    }

    fn deserialize_tuple<V>(mut self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        struct Access<'a, 'b> {
            deserializer: &'a mut Deserializer<'b>,
            len: usize,
        }

        impl<'a, 'b: 'a> de::SeqAccess<'b> for Access<'a, 'b> {
            type Error = DeserializerError;

            fn next_element_seed<T>(
                &mut self,
                seed: T,
            ) -> Result<Option<T::Value>, DeserializerError>
            where
                T: de::DeserializeSeed<'b>,
            {
                if self.len > 0 {
                    self.len -= 1;
                    let mut inner_deserializer = Deserializer {
                        buf: self.deserializer.buf,
                    };
                    let value = de::DeserializeSeed::deserialize(seed, &mut inner_deserializer)?;
                    self.deserializer.buf = inner_deserializer.buf;
                    Ok(Some(value))
                } else {
                    Ok(None)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.len)
            }
        }

        visitor.visit_seq(Access {
            deserializer: &mut self,
            len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("map"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("enum"))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("identifier"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(DeserializerError::TypeUnsupported("ignored_any"))
    }
}

pub fn decode_database<R, T>(mut read: R) -> Result<Vec<T>, DeserializerError>
where
    R: Read,
    T: de::DeserializeOwned,
{
    let count = read.read_u32::<LE>()? as usize;
    let mut elements = Vec::with_capacity(count);
    let mut read_buf = Vec::new();
    for _ in 0..count {
        let size = read.read_u32::<LE>()? as usize;
        read_buf.resize(size, 0);

        read.read_exact(&mut read_buf[..size])?;

        let mut deserializer = Deserializer {
            buf: &read_buf[..size],
        };
        let element = T::deserialize(&mut deserializer)?;
        elements.push(element);

        read_buf.clear();
    }
    Ok(elements)
}
