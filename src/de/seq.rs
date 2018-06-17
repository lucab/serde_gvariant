use byteorder::ReadBytesExt;
use config;
use de::top::TopDeserializer;
use errors::{self, ResultExt};
use serde::de::{self, Error};
use std::io;

pub(crate) struct SeqDeAccess<RS> {
    pub(crate) seq_framing_start: u64,
    pub(crate) seq_fixed_width: bool,
    pub(crate) seq_length: u64,
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
        // Stop conditions:
        //   1. fixed-width entries: EOF reached
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        if self.seq_fixed_width && cur == self.seq_length {
            trace!("got fixed width array: len={}", self.seq_length);
            return Ok(None);
        }
        //   2. variable-width entries: all framings processed
        if !self.seq_fixed_width && self.seq_framing_start == self.seq_length {
            trace!("got variable width array: len={}", self.seq_length);
            return Ok(None);
        }

        // Deserialize next element
        trace!("accessing array element: cur={}, seq_framing_start={}, seq_length={}", cur, self.seq_framing_start, self.seq_length);
        let mut seq_de = SeqDeserializer {
            seq_framing_start: &mut self.seq_framing_start,
            seq_length: &mut self.seq_length,
            seq_fixed_width: &mut self.seq_fixed_width,
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
    pub(crate) seq_framing_start: &'a mut u64,
    pub(crate) seq_fixed_width: &'a mut bool,
    pub(crate) seq_length: &'a mut u64,
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
            char str bytes byte_buf
            unit unit_struct map
            option newtype_struct
    }

    // Fixed size
    fn deserialize_bool<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = false;

        let start = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::Start(*self.seq_framing_start))?;
        let end = self.reader.read_u8()? as u64;
        *self.seq_framing_start = self.seq_framing_start.saturating_add(1);
        let buflen = (end - start) as usize;
        trace!("string: start={}, end={}, buflen={}", start, end, buflen);

        self.reader.seek(io::SeekFrom::Start(start))?;
        let mut buf = vec![0u8; buflen];
        self.reader.read_exact(&mut buf).chain_err(|| "seq string")?;
        let value = {
            let mut top = TopDeserializer {
                reader: io::Cursor::new(buf),
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
        *self.seq_fixed_width = false;

        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        *self.seq_length = self.seq_length.saturating_sub(1);
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        trace!("seq: len={}", buflen);
        let mut buf = vec![0u8; buflen];
        self.reader.read_exact(&mut buf).chain_err(|| "seq seq")?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
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
        *self.seq_fixed_width = false;

        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        *self.seq_length = self.seq_length.saturating_sub(1);
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        trace!("struct: len={}", buflen);
        let mut buf = vec![0u8; buflen];
        self.reader.read_exact(&mut buf).chain_err(|| "seq seq")?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        let v = top.deserialize_struct(name, fields, visitor)?;
        Ok(v)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        trace!("tuple -> tuple_struct");
        *self.seq_fixed_width = false;
        self.deserialize_tuple_struct("tuple", len, visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        trace!("tuple_struct -> struct");
        *self.seq_fixed_width = false;
        if len > 32 {
            bail!("too many fields in tuple");
        }
        let syn_fields = &[
            "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15",
            "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
            "30", "31",
        ];
        self.deserialize_struct(name, &syn_fields[..len], visitor)
    }

    fn deserialize_enum<V>(
        self,
        enumer: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = false;

        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        *self.seq_length = self.seq_length.saturating_sub(1);
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        trace!("enum: len={}", buflen);
        let mut buf = vec![0u8; buflen];
        self.reader.read_exact(&mut buf).chain_err(|| "seq enum")?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        let v = top.deserialize_enum(enumer, variants, visitor)?;
        Ok(v)
    }
}
