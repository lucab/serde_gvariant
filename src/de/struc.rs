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
            trace!(
                "struct done: name={}, fields={}",
                self._name,
                self.fields.len()
            );
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
            trace!(
                "next field: name={}, field={}, end={}",
                self._name,
                self.fields[self.cur_field],
                seq_de.end
            );
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
        Err(Self::Error::custom("struct: any not supported"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: ignored_any not supported"))
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
        const ALIGNMENT: u64 = 2;
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        self.reader.seek(io::SeekFrom::Current(padding as i64))?;

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
        const ALIGNMENT: u64 = 2;
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        self.reader.seek(io::SeekFrom::Current(padding as i64))?;

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
        const ALIGNMENT: u64 = 4;
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        self.reader.seek(io::SeekFrom::Current(padding as i64))?;

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
        const ALIGNMENT: u64 = 4;
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        self.reader.seek(io::SeekFrom::Current(padding as i64))?;

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
        const ALIGNMENT: u64 = 8;
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        self.reader.seek(io::SeekFrom::Current(padding as i64))?;

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
        const ALIGNMENT: u64 = 8;
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        self.reader.seek(io::SeekFrom::Current(padding as i64))?;

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
        const ALIGNMENT: u64 = 8;
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        self.reader.seek(io::SeekFrom::Current(padding as i64))?;

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
                reader: io::Cursor::new(buf),
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
        trace!("string: len={}", buflen);
        let mut buf = vec![0u8; buflen];
        self.reader
            .read_exact(&mut buf)
            .chain_err(|| "struct string")?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
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
                reader: io::Cursor::new(buf),
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
        trace!("seq: len={}", buflen);
        let mut buf = vec![0u8; buflen];
        self.reader.read_exact(&mut buf).chain_err(|| "struct seq")?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        trace!("seq: cur={}, end={}", cur, end);
        let v = top.deserialize_seq(visitor)?;
        Ok(v)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        trace!("tuple -> tuple_struct");
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

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: unit_struct not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: newtype_struct not supported"))
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: unit not supported"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: map not supported"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: byte_buf not supported"))
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: bytes not supported"))
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: str not supported"))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: char not supported"))
    }

    fn deserialize_f32<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: f32 not supported"))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: identifier not supported"))
    }

    fn deserialize_option<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("struct: option not supported"))
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
        const ALIGNMENT: u64 = 8;
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        let padding = (ALIGNMENT - (cur % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} {} padding bytes", cur, padding);
        self.reader.seek(io::SeekFrom::Current(padding as i64))?;

        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        if self.end.saturating_sub(cur) == 0 {
            let buf = vec![];
            trace!("empty seq");
            let mut top = TopDeserializer {
                reader: io::Cursor::new(buf),
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
            let val = self.reader.read_u8().chain_err(|| "struct enum len")?;
            self.reader.seek(io::SeekFrom::Start(cur))?;
            *self.end -= 1;
            val as u64
        };
        let buflen = (end - cur) as usize;
        trace!("enum: len={}", buflen);
        let mut buf = vec![0u8; buflen];
        self.reader
            .read_exact(&mut buf)
            .chain_err(|| "struct enum")?;
        let mut top = TopDeserializer {
            reader: io::Cursor::new(buf),
            options: self.options.clone(),
        };
        trace!("enum: cur={}, end={}", cur, end);
        let v = top.deserialize_enum(enumer, variants, visitor)?;
        Ok(v)
    }
}
