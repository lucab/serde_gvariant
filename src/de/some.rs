use byteorder::ReadBytesExt;
use errors::{self, ResultExt};
use serde::de::{self, Error};
use std::io;

use de::cursor::CursorDeserializer;

pub(crate) struct SomeDeserializer<'a, RS: 'a> {
    pub(crate) _len: usize,
    pub(crate) end: &'a mut u64,
    pub(crate) top: &'a mut ::de::top::TopDeserializer<RS>,
}

impl<'de, 'a, RS> de::Deserializer<'de> for &'a mut SomeDeserializer<'a, RS>
where
    RS: io::Read + io::Seek,
{
    type Error = errors::Error;

    // Unsupported
    fn deserialize_any<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
    }
    forward_to_deserialize_any! {
        f32 identifier ignored_any
    }

    // Fixed size
    fn deserialize_bool<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let start = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let start = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_i8(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let start = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_u8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 2;
        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("i16: skipping {} padding bytes", padding);
        let start = self
            .top
            .reader
            .seek(io::SeekFrom::Current(padding as i64))?;

        let mut top = CursorDeserializer {
            start,
            end: *self.end + 2,
            top: &mut *self.top,
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 2;
        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("u16: skipping {} padding bytes", padding);
        let start = self
            .top
            .reader
            .seek(io::SeekFrom::Current(padding as i64))?;

        let mut top = CursorDeserializer {
            start,
            end: start + 2,
            top: &mut *self.top,
        };
        top.deserialize_u16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 4;
        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let start = self
            .top
            .reader
            .seek(io::SeekFrom::Current(padding as i64))?;

        let mut top = CursorDeserializer {
            start,
            end: start + 4,
            top: &mut *self.top,
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 4;
        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let start = self
            .top
            .reader
            .seek(io::SeekFrom::Current(padding as i64))?;

        let mut top = CursorDeserializer {
            start,
            end: start + 4,
            top: &mut *self.top,
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 8;
        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let start = self
            .top
            .reader
            .seek(io::SeekFrom::Current(padding as i64))?;

        let mut top = CursorDeserializer {
            start,
            end: start + 8,
            top: &mut *self.top,
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 8;
        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let start = self
            .top
            .reader
            .seek(io::SeekFrom::Current(padding as i64))?;

        let mut top = CursorDeserializer {
            start,
            end: start + 8,
            top: &mut *self.top,
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 8;
        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let start = self
            .top
            .reader
            .seek(io::SeekFrom::Current(padding as i64))?;

        let mut top = CursorDeserializer {
            start,
            end: start + 8,
            top: &mut *self.top,
        };
        top.deserialize_f64(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let start = self.top.reader.seek(io::SeekFrom::Current(0))?;
        self.top.reader.seek(io::SeekFrom::Start(*self.end - 1))?;
        let term = self
            .top
            .reader
            .read_u8()
            .chain_err(|| "struct: reading string length")?;
        self.top.reader.seek(io::SeekFrom::Start(start))?;
        *self.end -= 1;
        if term != 0x00 {
            return Err(Self::Error::custom("some: string non-zero terminator"));
        }

        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_string(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let start = self.top.reader.seek(io::SeekFrom::Current(0))?;
        self.top.reader.seek(io::SeekFrom::Start(*self.end - 1))?;
        let term = self
            .top
            .reader
            .read_u8()
            .chain_err(|| "struct: reading string length")?;
        self.top.reader.seek(io::SeekFrom::Start(start))?;
        *self.end -= 1;
        if term != 0x00 {
            return Err(Self::Error::custom("some: string non-zero terminator"));
        }

        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_byte_buf(visitor)
    }

    // Pending implementation
    forward_to_deserialize_any! {
        char str enum bytes
        unit unit_struct seq tuple tuple_struct map
        option newtype_struct struct
    }
}
