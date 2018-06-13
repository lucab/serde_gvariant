use byteorder::ReadBytesExt;
use config;
use errors::{self, ResultExt};
use serde::de::{self, Error};
use std::io;

use de::top::TopDeserializer;

pub(crate) struct StructDeAccess<RS> {
    pub(crate) cur_field: usize,
    pub(crate) end: u64,
    pub(crate) _name: &'static str,
    pub(crate) fields: &'static [&'static str],
    pub(crate) options: config::Config,
    pub(crate) reader: RS,
}

impl<'a, 'de, RS> de::SeqAccess<'de> for &'a mut StructDeAccess<RS>
where
    RS: io::Read + io::Seek,
{
    type Error = errors::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> errors::Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        // Stop when all fields are done
        if self.cur_field >= self.fields.len() {
            trace!("struct done: name={}, fields={}", self._name, self.fields.len());
            return Ok(None);
        }

        // Deserialize next element
        let v = {
            let mut seq_de = StructDeserializer {
                cur_field: &self.cur_field,
                end: &mut self.end,
                fields: self.fields,
                reader: &mut self.reader,
                options: self.options.clone(),
            };
            // eprintln!("struct: name={}, cur={}, fields={}, end={}", self._name, seq_de.cur_field, seq_de.num_fields, seq_de.end);
            de::DeserializeSeed::deserialize(seed, &mut seq_de)?
        };
        self.cur_field += 1;
        Ok(Some(v))
    }
}

// A Deserializer specialized on structures, with custom logic
// for non-fized-size ones.
pub(crate) struct StructDeserializer<'a, RS> {
    pub(crate) cur_field: &'a usize,
    pub(crate) end: &'a mut u64,
    pub(crate) fields: &'static [&'static str],
    pub(crate) options: config::Config,
    pub(crate) reader: RS,
}

impl<'de, 'a, RS> de::Deserializer<'de> for &'a mut StructDeserializer<'a, RS>
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
            option newtype_struct struct
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
        if self.end.saturating_sub(cur) == 0 {
            let buf = vec![0x00];
            trace!("empty string");
            let mut top = TopDeserializer {
                reader: buf.as_slice(),
                options: self.options.clone(),
            };
            return top.deserialize_string(visitor);
        };

        let end = if self.cur_field.saturating_add(1) >= self.fields.len() {
            *self.end as u64
        } else {
            self.reader.seek(io::SeekFrom::Start(*self.end - 1))?;
            let val = self.reader.read_u8().chain_err(|| "struct string len")?;
            self.reader.seek(io::SeekFrom::Start(cur))?;
            *self.end -= 1;
            val as u64
        };
        let buflen = (end - cur) as usize;
        let mut buf = Vec::with_capacity(buflen);
        unsafe { buf.set_len(buflen) };
        self.reader
            .read_exact(&mut buf)
            .chain_err(|| "struct string")?;
        let mut top = TopDeserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        let v = top.deserialize_string(visitor)?;
        Ok(v)
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        if self.end.saturating_sub(cur) == 0 {
            let buf = vec![];
            trace!("empty seq");
            let mut top = TopDeserializer {
                reader: buf.as_slice(),
                options: self.options.clone(),
            };
            return top.deserialize_seq(visitor);
        };

        let end = if self.cur_field.saturating_add(1) >= self.fields.len() {
            let val = *self.end as u64;
            *self.end -= 1;
            val
        } else {
            self.reader.seek(io::SeekFrom::Start(*self.end - 1))?;
            let val = self.reader.read_u8().chain_err(|| "struct seq len")?;
            self.reader.seek(io::SeekFrom::Start(cur))?;
            *self.end -= 1;
            val as u64
        };
        let buflen = (end - cur) as usize;
        let mut buf = Vec::with_capacity(buflen);
        unsafe { buf.set_len(buflen) };
        self.reader.read_exact(&mut buf).chain_err(|| "struct seq")?;
        let mut top = TopDeserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        let v = top.deserialize_seq(visitor)?;
        Ok(v)
    }
}
