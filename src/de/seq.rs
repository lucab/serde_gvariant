use byteorder::ReadBytesExt;
use errors;
use serde::de::{self, Error};
use std::io;

use de::cursor::CursorDeserializer;

pub(crate) struct SeqDeAccess<'a, RS: 'a> {
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) seq_framing_start: u64,
    pub(crate) seq_fixed_width: bool,
    pub(crate) seq_length: u64,
    pub(crate) top: &'a mut ::de::top::TopDeserializer<RS>,
}

impl<'a, 'de, RS> de::SeqAccess<'de> for &'a mut SeqDeAccess<'a, RS>
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
        if self.start == self.end {
            trace!("got fixed width array: len={}", self.seq_length);
            return Ok(None);
        }
        //   2. variable-width entries: all framings processed
        if !self.seq_fixed_width && self.seq_framing_start == self.seq_length {
            trace!("got variable width array: len={}", self.seq_length);
            return Ok(None);
        }

        // Deserialize next element
        trace!(
            "accessing array element: cur_start={:#x}, end={:#x} seq_length={:#x}",
            self.start,
            self.end,
            self.seq_length
        );
        let mut seq_de = SeqDeserializer {
            start: &mut self.start,
            end: &mut self.end,
            seq_framing_start: &mut self.seq_framing_start,
            seq_length: &mut self.seq_length,
            seq_fixed_width: &mut self.seq_fixed_width,
            top: &mut self.top,
        };
        let v = de::DeserializeSeed::deserialize(seed, &mut seq_de)?;
        Ok(Some(v))
    }
}

// A Deserializer specialized on array, with custom logic
// for non-fized-size ones.
pub(crate) struct SeqDeserializer<'a, RS: 'a> {
    pub(crate) start: &'a mut u64,
    pub(crate) end: &'a mut u64,
    pub(crate) seq_framing_start: &'a mut u64,
    pub(crate) seq_fixed_width: &'a mut bool,
    pub(crate) seq_length: &'a mut u64,
    pub(crate) top: &'a mut ::de::top::TopDeserializer<RS>,
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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

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
        *self.seq_fixed_width = true;

        let cur = *self.start;
        *self.start += 2;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;

        let cur = *self.start;
        *self.start += 2;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_u16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;

        let cur = *self.start;
        *self.start += 4;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;

        let cur = *self.start;
        *self.start += 4;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;

        let cur = *self.start;
        *self.start += 8;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;

        let cur = *self.start;
        *self.start += 8;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;

        let cur = *self.start;
        *self.start += 8;
        let mut top = CursorDeserializer {
            start: cur,
            end: *self.end,
            top: &mut *self.top,
        };
        top.deserialize_f64(visitor)
    }

    // Variable size
    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = false;

        let start = *self.start;
        self.top
            .reader
            .seek(io::SeekFrom::Start(*self.seq_framing_start))?;
        let end = u64::from(self.top.reader.read_u8()?);
        *self.seq_framing_start = self.seq_framing_start.saturating_add(1);
        let buflen = end.checked_sub(start)
            .ok_or_else(|| Self::Error::custom("array: string length underflow"))?;
        trace!("string: start={}, end={}, buflen={}", start, end, buflen);

        *self.start += buflen;
        self.top.reader.seek(io::SeekFrom::Start(start))?;
        let mut top = CursorDeserializer {
            start,
            end: end,
            top: &mut *self.top,
        };
        let value = top.deserialize_string(visitor)?;
        Ok(value)
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = false;

        let start = *self.start;
        self.top.reader.seek(io::SeekFrom::End(-1))?;
        let end = u64::from(self.top.reader.read_u8()?);
        *self.seq_length = self.seq_length.saturating_sub(1);
        self.top.reader.seek(io::SeekFrom::Start(start))?;
        let buflen = end.checked_sub(start)
            .ok_or_else(|| Self::Error::custom("array: array length underflow"))?;
        trace!("seq: len={}", buflen);
        *self.start += buflen;
        let mut top = CursorDeserializer {
            start,
            end: end,
            top: &mut *self.top,
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

        let cur = *self.start;
        *self.end -= 1;

        self.top.reader.seek(io::SeekFrom::Start(*self.end))?;
        let end = self.top.reader.read_u8()? as u64;
        *self.seq_length = self.seq_length.saturating_sub(1);
        self.top.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = end.checked_sub(cur)
            .ok_or_else(|| Self::Error::custom("array: struct length underflow"))?
            as usize;

        trace!(
            "struct: start={:#x}, end={:#x}, len={:#x}",
            cur,
            end,
            buflen
        );
        let mut top = CursorDeserializer {
            start: cur,
            end: end,
            top: &mut *self.top,
        };
        *self.start += buflen as u64;
        top.deserialize_struct(name, fields, visitor)
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

        let cur = self.top.reader.seek(io::SeekFrom::Current(0))?;
        self.top.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.top.reader.read_u8()? as u64;
        *self.seq_length = self.seq_length.saturating_sub(1);
        self.top.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        trace!("enum: len={}", buflen);
        let mut top = CursorDeserializer {
            start: cur,
            end,
            top: &mut *self.top,
        };
        top.deserialize_enum(enumer, variants, visitor)
    }
}
