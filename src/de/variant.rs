use crate::errors;
use byteorder::ReadBytesExt;
use serde::de::{self, Error};
use std::io;

use crate::de::cursor::CursorDeserializer;
//use de::some::SomeDeserializer;

pub(crate) struct EnumDeAccess<'a, RS: 'a> {
    pub(crate) cur_field: usize,
    pub(crate) start: &'a mut u64,
    pub(crate) end: &'a mut u64,
    pub(crate) name: &'static str,
    pub(crate) top: &'a mut crate::de::top::TopDeserializer<RS>,
    pub(crate) signature: Vec<u8>,
    pub(crate) variants: &'static [&'static str],
    pub(crate) seq_framing_start: &'a mut u64,
    pub(crate) seq_fixed_width: bool,
    // array: start of array (immutable)
    pub(crate) seq_start: u64,
    // array: length of array (immutable)
    pub(crate) seq_length: u64,
}

impl<'a, 'de, RS> de::EnumAccess<'de> for &'a mut EnumDeAccess<'a, RS>
where
    RS: io::Read + io::Seek,
{
    type Error = errors::Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        // Stop when all fields are done
        if self.cur_field >= self.signature.len() {
            return Err(Self::Error::custom("variant: signature overflowed"));
        }

        trace!(
            "accessing field: name={}, sig={}, cur_field={}, cur_start={:#x}, payload_end={:#x}, seq_length={:#x}, seq_fixed_width={}",
            self.name,
            String::from_utf8_lossy(&self.signature),
            self.cur_field,
            self.start,
            self.end,
            self.seq_length,
            self.seq_fixed_width,
        );

        // Deserialize next element
        let v = {
            let mut seq_de = EnumDeserializer {
                cur_field: &self.cur_field,
                start: &mut self.start,
                end: &mut self.end,
                fields: self.variants,
                top: &mut self.top,
                seq_fixed_width: &mut self.seq_fixed_width,
                seq_framing_start: &mut self.seq_framing_start,
                seq_length: &mut self.seq_length,
                seq_start: &mut self.seq_start,
                signature: &mut self.signature,
            };
            de::DeserializeSeed::deserialize(seed, &mut seq_de)?
        };
        Ok((v, self))
    }
}

impl<'a, 'de, RS> de::SeqAccess<'de> for &'a mut EnumDeAccess<'a, RS>
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
        if self.seq_fixed_width && *self.start == *self.end {
            trace!("got fixed width array: len={:#x}", self.seq_length);
            return Ok(None);
        }
        //   2. variable-width entries: all framings processed
        if !self.seq_fixed_width && *self.seq_framing_start == self.seq_start + self.seq_length {
            trace!("got variable width array: len={:#x}", self.seq_length);
            return Ok(None);
        }

        // Deserialize next element
        trace!(
            "accessing array element: cur={:#x}, seq_start={:#x}, seq_framing_start={:#x}, seq_length={:#x}",
            *self.start,
            self.seq_start,
            self.seq_framing_start,
            self.seq_length
        );
        let mut seq_de = EnumDeserializer {
            cur_field: &self.cur_field,
            end: &mut self.end,
            start: &mut self.start,
            fields: self.variants,
            top: &mut self.top,
            seq_fixed_width: &mut self.seq_fixed_width,
            seq_framing_start: &mut self.seq_framing_start,
            seq_length: &mut self.seq_length,
            seq_start: &mut self.seq_start,
            signature: &mut self.signature,
        };
        let v = de::DeserializeSeed::deserialize(seed, &mut seq_de)?;
        Ok(Some(v))
    }
}

impl<'a, 'de, RS> de::VariantAccess<'de> for &'a mut EnumDeAccess<'a, RS>
where
    RS: io::Read + io::Seek,
{
    type Error = errors::Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Err(Self::Error::custom("variant access: unit not supported"))
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        // Deserialize next element
        trace!(
            "accessing newtype: cur={}, sig={}, fixed={}, cur_start={:#x}, end={:#x}, seq_length={}, seq_fs={}",
            self.cur_field,
            self.signature[self.cur_field] as char,
            self.seq_fixed_width,
            self.start,
            self.end,
            self.seq_length,
            self.seq_framing_start,
        );

        let mut seq_de = EnumDeserializer {
            cur_field: &self.cur_field,
            start: &mut self.start,
            end: &mut self.end,
            fields: self.variants,
            top: &mut self.top,
            seq_fixed_width: &mut self.seq_fixed_width,
            seq_framing_start: &mut self.seq_framing_start,
            seq_length: &mut self.seq_length,
            seq_start: &mut self.seq_start,
            signature: &mut self.signature,
        };
        let v = de::DeserializeSeed::deserialize(seed, &mut seq_de)?;
        Ok(v)
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant access: tuple not supported"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant access: struct not supported"))
    }
}

// A Deserializer specialized on structures, with custom logic
// for non-fized-size ones.
#[derive(Debug)]
pub(crate) struct EnumDeserializer<'a, RS: 'a> {
    pub(crate) cur_field: &'a usize,
    pub(crate) end: &'a mut u64,
    pub(crate) start: &'a mut u64,
    pub(crate) fields: &'static [&'static str],
    pub(crate) top: &'a mut crate::de::top::TopDeserializer<RS>,
    pub(crate) signature: &'a mut [u8],
    pub(crate) seq_framing_start: &'a mut u64,
    pub(crate) seq_fixed_width: &'a mut bool,
    pub(crate) seq_length: &'a mut u64,
    pub(crate) seq_start: &'a mut u64,
}

impl<'de, 'a, RS> de::Deserializer<'de> for &'a mut EnumDeserializer<'a, RS>
where
    RS: io::Read + io::Seek,
{
    type Error = errors::Error;

    // Unsupported
    fn deserialize_any<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: any not supported"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: ignored_any not supported"))
    }

    fn deserialize_option<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: option not supported"))
    }

    // Fixed size
    fn deserialize_bool<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 1;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 1;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_i8(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 1;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_u8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 2;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 2;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_u16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 4;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 4;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 8;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 8;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
            top: &mut *self.top,
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        *self.seq_fixed_width = true;
        const ALIGNMENT: u64 = 8;

        let cur = *self.start;
        *self.start += ALIGNMENT;
        let mut top = CursorDeserializer {
            start: cur,
            end: cur + ALIGNMENT,
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
        if *self.seq_length == 0 {
            let mut top = CursorDeserializer {
                start,
                end: *self.end,
                top: &mut *self.top,
            };
            top.deserialize_string(visitor)
        } else {
            self.top
                .reader
                .seek(io::SeekFrom::Start(*self.seq_framing_start))?;
            let val = u64::from(self.top.reader.read_u8()?);
            let end = start + val;
            *self.seq_framing_start = self.seq_framing_start.saturating_add(1);
            let buflen = self.end.checked_sub(*self.start).ok_or_else(|| {
                Self::Error::custom(format!(
                    "variant: string length underflow - end={:#x}, start={:#x}",
                    *self.end, *self.start
                ))
            })? as usize;
            trace!(
                "string: start={:#x}, end={:#x}, buflen={:#x}, framings_start={:#x}",
                start,
                end,
                buflen,
                self.seq_framing_start
            );

            *self.start += val;
            self.top.reader.seek(io::SeekFrom::Start(start))?;
            let mut top = CursorDeserializer {
                start,
                end,
                top: &mut *self.top,
            };
            top.deserialize_string(visitor)
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let length = self
            .end
            .checked_sub(*self.start)
            .ok_or_else(|| Self::Error::custom("variant: array length underflow"))?
            as usize;

        let end_pos = self
            .end
            .checked_sub(1)
            .ok_or_else(|| Self::Error::custom("variant: array too short"))?;
        self.top.reader.seek(io::SeekFrom::Start(end_pos))?;
        let val = u64::from(self.top.reader.read_u8()?);
        let fstart = *self.start + val;
        *self.seq_framing_start = fstart;

        self.top.reader.seek(io::SeekFrom::Start(*self.start))?;
        let next_sig = self.signature.get(1).cloned().ok_or_else(|| {
            Self::Error::custom("variant: array element type missing from signature")
        })?;

        let fixed = match next_sig {
            b'y' => true,
            b'b' => true,
            _ => false,
        };
        trace!(
            "array: EnumDeAccess start={:#x}, end={:#x}, buflen={:#x}, framing_start={:#x}",
            *self.start,
            *self.end,
            length,
            fstart
        );
        let seq_start = *self.start;
        let mut sub = EnumDeAccess {
            cur_field: 0,
            signature: self.signature[1..].to_vec(),
            name: "seq",
            start: self.start,
            end: self.end,
            variants: &[],
            seq_framing_start: self.seq_framing_start,
            seq_fixed_width: fixed,
            seq_length: length as u64,
            seq_start,
            top: &mut *self.top,
        };
        visitor.visit_seq(&mut sub)
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
        let start = *self.start;

        self.top.reader.seek(io::SeekFrom::End(-1))?;
        let end = u64::from(self.top.reader.read_u8()?);

        *self.end = self.end.saturating_sub(1);
        self.top.reader.seek(io::SeekFrom::Start(start))?;
        let buflen = (end - *self.start) as usize;

        trace!(
            "struct: start={:#x}, end={:#x}, len={:#x}",
            *self.start,
            end,
            buflen
        );
        let mut top = CursorDeserializer {
            start,
            end,
            top: &mut *self.top,
        };
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
            bail!("variant: too many fields in tuple");
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
        let cur = *self.start;
        let end = *self.end;
        /*
        if *self.seq_length != 0 {
            self.top
                .reader
                .seek(io::SeekFrom::Start(*self.seq_framing_start as u64))?;
            *self.seq_framing_start = self.seq_framing_start.saturating_add(1);
            end = self.top.reader.read_u8()? as u64;
            self.top.reader.seek(io::SeekFrom::Start(cur))?;
        }
         */
        trace!(
            "variant: EnumDeAccess name={}, sig={}, start={:#x}, end={:#x}",
            enumer,
            self.signature[*self.cur_field] as char,
            cur,
            end,
        );
        let seq_start = *self.start;
        let mut sub = EnumDeAccess {
            cur_field: 0,
            end: self.end,
            start: self.start,
            name: enumer,
            variants,
            top: &mut *self.top,
            signature: self.signature[*self.cur_field..].to_vec(),
            seq_fixed_width: true,
            seq_framing_start: self.seq_framing_start,
            seq_length: *self.seq_length,
            seq_start,
        };
        let v = visitor.visit_enum(&mut sub)?;
        Ok(v)
    }

    /*
        fn deserialize_option<V>(self, visitor: V) -> errors::Result<V::Value>
        where
            V: de::Visitor<'de>,
        {
            let mut buf: Vec<u8> = Vec::new();
            //let len = self.top.reader.read_to_end(&mut buf)?;
            trace!("variant: option buflen={}", len);
            match len {
                // Fixed-Size inner: empty byte sequence.
                // Non-Fixed-Size inner: empty byte sequence.
                0 => visitor.visit_none(),

                // Fixed-Size inner: just data.
                // Non-Fixed-Size inner: data + 0x00.
                _ => {
                    let mut sub = SomeDeserializer {
                        _len: len,
                        options: self.top.options.clone(),
                        reader: io::Cursor::new(buf),
                    };
                    visitor.visit_some(&mut sub)
                }
            }
        }
    */
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: unit_struct not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: newtype_struct not supported"))
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: unit not supported"))
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: map not supported"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: byte_buf not supported"))
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: bytes not supported"))
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: str not supported"))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: char not supported"))
    }

    fn deserialize_f32<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("variant: f32 not supported"))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let charsig = *self.signature.first().unwrap_or(&b'Z');
        trace!("variant: got id={}", charsig as char);
        let (id, fixed_width) = match charsig {
            b'a' => (14, false),
            b'b' => (0, true),
            b's' => (9, false),
            b'y' => (1, true),
            c => {
                return Err(Self::Error::custom(format!(
                    "variant: unrecognized signature {}",
                    c as char
                )));
            }
        };
        *self.seq_fixed_width = fixed_width;
        visitor.visit_u8(id)
    }
}
