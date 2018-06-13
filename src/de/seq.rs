use byteorder::ReadBytesExt;
use config;
use de::top::TopDeserializer;
use errors::{self, ResultExt};
use serde::de::{self, Error};
use std::io;

pub(crate) struct SeqDeAccess<RS> {
    pub(crate) end: u64,
    pub(crate) options: config::Config,
    pub(crate) reader: RS,
}

impl<'a, 'de, RS> de::SeqAccess<'de> for &'a mut SeqDeAccess<RS>
where
    RS: io::Read + io::Seek,
{
    type Error = errors::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> errors::Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        // Stop if EOF is reached
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        // eprintln!("seq: cur={}, end={}", cur, self.end);
        if cur == self.end {
            return Ok(None);
        }

        // Deserialize next element
        let mut seq_de = SeqDeserializer {
            end: &mut self.end,
            options: self.options.clone(),
            reader: &mut self.reader,
        };
        let v = de::DeserializeSeed::deserialize(seed, &mut seq_de)?;
        Ok(Some(v))
    }
}

// A Deserializer specialized on array, with custom logic
// for non-fized-size ones.
pub(crate) struct SeqDeserializer<'a, RS> {
    pub(crate) end: &'a mut u64,
    pub(crate) options: config::Config,
    pub(crate) reader: RS,
}

impl<'de, 'a, RS> de::Deserializer<'de> for &'a mut SeqDeserializer<'a, RS>
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

    forward_to_deserialize_any! {
            char str enum bytes byte_buf
            unit unit_struct tuple tuple_struct map
            option newtype_struct
    }

    // Fixed size
    fn deserialize_bool<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i8(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = TopDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_f64(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        let mut buf = Vec::with_capacity(buflen);
        unsafe { buf.set_len(buflen) };
        self.reader.read_exact(&mut buf).chain_err(|| "seq string")?;
        let value = {
            let mut top = TopDeserializer {
                reader: buf.as_slice(),
                options: self.options.clone(),
            };
            let v = top.deserialize_string(visitor)?;
            v
        };
        Ok(value)
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        *self.end = self.end.saturating_sub(1);
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        let mut buf = Vec::with_capacity(buflen);
        unsafe { buf.set_len(buflen) };
        self.reader.read_exact(&mut buf).chain_err(|| "seq seq")?;
        let mut top = TopDeserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        let v = top.deserialize_seq(visitor)?;
        Ok(v)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        *self.end = self.end.saturating_sub(1);
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        let mut buf = Vec::with_capacity(buflen);
        unsafe { buf.set_len(buflen) };
        self.reader.read_exact(&mut buf).chain_err(|| "seq seq")?;
        let mut top = TopDeserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        let v = top.deserialize_struct(name, fields, visitor)?;
        Ok(v)
    }
}
