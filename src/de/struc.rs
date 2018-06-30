use byteorder::ReadBytesExt;
use errors::{self, ResultExt};
use serde::de::{self, Error};
use std::io;

use de::cursor::CursorDeserializer;

pub(crate) struct StructDeAccess<'a, RS: 'a> {
    pub(crate) cur_field: usize,
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) _name: &'static str,
    pub(crate) fields: &'static [&'static str],
    pub(crate) top: &'a mut ::de::top::TopDeserializer<RS>,
}

impl<'a, 'de, RS> de::SeqAccess<'de> for &'a mut StructDeAccess<'a, RS>
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
                start: &mut self.start,
                end: &mut self.end,
                fields: self.fields,
                top: &mut self.top,
            };
            trace!(
                "next field: field_name={}, field_start={:#x} - struct_name={}, struct_end={:#x}",
                self.fields[self.cur_field],
                seq_de.start,
                self._name,
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
pub(crate) struct StructDeserializer<'a, RS: 'a> {
    pub(crate) cur_field: &'a usize,
    pub(crate) start: &'a mut u64,
    pub(crate) end: &'a mut u64,
    pub(crate) fields: &'static [&'static str],
    pub(crate) top: &'a mut ::de::top::TopDeserializer<RS>,
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
        let cur = *self.start;
        *self.start += 1;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let cur = *self.start;
        *self.start += 1;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_i8(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let cur = *self.start;
        *self.start += 1;
        let mut top = CursorDeserializer {
            start: cur,
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
        let padding = (ALIGNMENT - (*self.start % ALIGNMENT)) % ALIGNMENT;
        trace!("i16: skipping {} padding bytes", padding);
        let start = self.top.reader.seek(io::SeekFrom::Current(padding as i64))?;
        *self.start += padding + ALIGNMENT;

        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 2;
        let padding = (ALIGNMENT - (*self.start % ALIGNMENT)) % ALIGNMENT;
        trace!("u16: skipping {} padding bytes", padding);
        let start = self.top.reader.seek(io::SeekFrom::Current(padding as i64))?;
        *self.start += padding + ALIGNMENT;

        let mut top = CursorDeserializer {
            start: start,
            end: *self.end,
            top: &mut *self.top,
        };
        let v = top.deserialize_u16(visitor)?;
        Ok(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 4;
        let padding = (ALIGNMENT - (*self.start % ALIGNMENT)) % ALIGNMENT;
        trace!("i32: skipping {} padding bytes", padding);
        let start = self.top.reader.seek(io::SeekFrom::Current(padding as i64))?;
        *self.start += padding + ALIGNMENT;

        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 4;
        let padding = (ALIGNMENT - (*self.start % ALIGNMENT)) % ALIGNMENT;
        trace!("u32: skipping {} padding bytes", padding);
        let start = self.top.reader.seek(io::SeekFrom::Current(padding as i64))?;
        *self.start += padding + ALIGNMENT;

        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 8;
        let padding = (ALIGNMENT - (*self.start % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let start = self.top.reader.seek(io::SeekFrom::Current(padding as i64))?;
        *self.start += padding + ALIGNMENT;

        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 8;
        let padding = (ALIGNMENT - (*self.start % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let start = self.top.reader.seek(io::SeekFrom::Current(padding as i64))?;
        *self.start += padding + ALIGNMENT;

        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        const ALIGNMENT: u64 = 8;
        let padding = (ALIGNMENT - (*self.start % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let start = self.top.reader.seek(io::SeekFrom::Current(padding as i64))?;
        *self.start += padding + ALIGNMENT;

        let mut top = CursorDeserializer {
            start,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_f64(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let cur = *self.start;
        if self.end.saturating_sub(cur) == 0 {
            trace!("empty string");
            let mut top = CursorDeserializer {
                start: 0,
                end: 0,
                top: &mut *self.top,
            };
            return top.deserialize_string(visitor);
        };

        let end = if self.cur_field.saturating_add(1) >= self.fields.len() {
            *self.end as u64
        } else {
            *self.end -= 1;
            self.top.reader.seek(io::SeekFrom::Start(*self.end))?;
            let val = self.top
                .reader
                .read_u8()
                .chain_err(|| "struct: reading string length")?;
            self.top.reader.seek(io::SeekFrom::Start(cur))?;
            val as u64
        };
        let buflen = end.checked_sub(cur)
            .ok_or_else(|| Self::Error::custom("struct: array length underflow"))?
            as usize;
        trace!(
            "string: cur={:#x}, end={:#x}, length={:#x}",
            cur,
            end,
            buflen
        );

        *self.start += buflen as u64;
        let mut top = CursorDeserializer {
            start: cur,
            end: end,
            top: &mut *self.top,
        };
        top.deserialize_string(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // Empty array.
        if self.end.saturating_sub(*self.start) == 0 {
            trace!("empty array");
            let mut top = CursorDeserializer {
                start: 0,
                end: 0,
                top: &mut *self.top,
            };
            return top.deserialize_seq(visitor);
        };

        // Non-empty array.
        let cur = *self.start;
        let end = if self.cur_field.saturating_add(1) >= self.fields.len() {
            let val = *self.end as u64;
            *self.end -= 1;
            val
        } else {
            self.top.reader.seek(io::SeekFrom::Start(*self.end - 1))?;
            let val = self.top
                .reader
                .read_u8()
                .chain_err(|| "struct: reading array length")?;
            self.top.reader.seek(io::SeekFrom::Start(cur))?;
            *self.end -= 1;
            val as u64
        };
        let buflen = end.checked_sub(cur)
            .ok_or_else(|| Self::Error::custom("struct: array length underflow"))?
            as usize;

        trace!(
            "array: cur={:#x}, end={:#x}, length={:#x}",
            cur,
            end,
            buflen
        );
        *self.start = end;
        let mut top = CursorDeserializer {
            start: cur,
            end,
            top: &mut *self.top,
        };
        top.deserialize_seq(visitor)
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
        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        self.top.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.top.reader.read_u8()? as u64;
        *self.end = self.end.saturating_sub(1);
        self.top.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        trace!("struct: len={}", buflen);
        let mut top = CursorDeserializer {
            start: cur,
            end,
            top: &mut *self.top,
        };
        top.deserialize_struct(name, fields, visitor)
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
        let padding = (ALIGNMENT - (*self.start % ALIGNMENT)) % ALIGNMENT;
        trace!("struct: skipping {} padding bytes", padding);
        let cur = self.top.reader.seek(io::SeekFrom::Current(padding as i64))?;
        *self.start += padding + ALIGNMENT;

        if self.end.saturating_sub(cur) == 0 {
            trace!("empty seq");
            let mut top = CursorDeserializer {
                start: 0,
                end: 0,
                top: &mut *self.top,
            };
            return top.deserialize_enum(enumer, variants, visitor);
        };

        let end = if self.cur_field.saturating_add(1) >= self.fields.len() {
            *self.end as u64
        } else {
            self.top.reader.seek(io::SeekFrom::Start(*self.end - 1))?;
            let val = self.top.reader.read_u8().chain_err(|| "struct enum len")?;
            self.top.reader.seek(io::SeekFrom::Start(cur))?;
            *self.end -= 1;
            val as u64
        };
        let buflen = end.checked_sub(cur)
            .ok_or_else(|| Self::Error::custom("struct: array length underflow"))?
            as usize;

        trace!(
            "enum: cur={:#x}, end={:#x}, length={:#x}",
            cur,
            self.end,
            buflen
        );
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_enum(enumer, variants, visitor)
    }
}
