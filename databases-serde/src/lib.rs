use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use serde::{de as sde, ser as sser};
use std::io::{Read, Seek, Write};

mod de;
mod ser;

pub use self::de::*;
pub use self::ser::*;

pub trait DatabaseRecord {
    fn database_id(&self) -> i32;
    fn database_enum_name(&self) -> &str;
}

pub fn decode_database<R, T>(mut read: R) -> Result<Vec<T>, DeserializerError>
where
    R: Read,
    T: sde::DeserializeOwned,
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

pub fn encode_database<W, T, I>(mut write: W, items: I) -> Result<(), SerializerError>
where
    W: Write + Seek,
    T: sser::Serialize,
    I: IntoIterator<Item = T>,
{
    write.write_u32::<LE>(0)?;
    let mut item_count: u32 = 0;
    for ref item in items {
        item_count += 1;
        let mut serializer = self::ser::Serializer { w: Vec::new() };
        item.serialize(&mut serializer)?;
        write.write_u32::<LE>(serializer.w.len() as u32)?;
        write.write_all(&serializer.w[..])?;
    }
    write.seek(std::io::SeekFrom::Start(0))?;
    write.write_u32::<LE>(item_count)?;
    Ok(())
}
