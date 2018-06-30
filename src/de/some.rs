use config;
use errors;
use serde::de::{self, Error};
use std::io;

use de::top::TopDeserializer;

pub(crate) struct SomeDeserializer<R> {
    pub(crate) _len: usize,
    pub(crate) options: config::Config,
    pub(crate) reader: R,
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut SomeDeserializer<R>
where
    R: io::Read,
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
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_bool(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_u8(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_i8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_u16(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_f64(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let term = buf.pop().unwrap();
        if term != 0x00 {
            return Err(Self::Error::custom("some: string non-zero terminator"));
        }

        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        top.deserialize_string(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let term = buf.pop().unwrap();
        if term != 0x00 {
            return Err(Self::Error::custom("some: byte_buf non-zero terminator"));
        }

        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
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
