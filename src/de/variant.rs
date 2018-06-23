use byteorder::ReadBytesExt;
use config;
use errors::{self, ResultExt};
use serde::de::{self, Error};
use std::{fmt, io};

use de::some::SomeDeserializer;
use de::top::TopDeserializer;

#[derive(Debug)]
pub(crate) struct EnumDeAccess<RS: fmt::Debug> {
    pub(crate) cur_field: usize,
    pub(crate) end: u64,
    pub(crate) name: &'static str,
    pub(crate) options: config::Config,
    pub(crate) reader: RS,
    pub(crate) signature: Vec<u8>,
    pub(crate) variants: &'static [&'static str],
    pub(crate) seq_framing_start: u64,
    pub(crate) seq_fixed_width: bool,
    pub(crate) seq_length: u64,
}

impl<'a, 'de, RS> de::EnumAccess<'de> for &'a mut EnumDeAccess<RS>
where
    RS: io::Read + io::Seek + fmt::Debug,
{
    type Error = errors::Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        // Stop when all fields are done
        if self.cur_field >= self.signature.len() {
            return Err(Self::Error::custom("signature overflowed"));
        }

        trace!(
            "accessing field: name={}, sig={}, cur_field={}, end={}, seq_length={}, seq_fixed_width={}",
            self.name,
            String::from_utf8_lossy(&self.signature),
            self.cur_field,
            self.end,
            self.seq_length,
            self.seq_fixed_width,
        );

        // Deserialize next element
        let v = {
            let mut seq_de = EnumDeserializer {
                cur_field: &self.cur_field,
                end: &mut self.end,
                fields: self.variants,
                options: self.options.clone(),
                reader: &mut self.reader,
                seq_fixed_width: &mut self.seq_fixed_width,
                seq_framing_start: &mut self.seq_framing_start,
                seq_length: &mut self.seq_length,
                signature: &mut self.signature,
            };
            de::DeserializeSeed::deserialize(seed, &mut seq_de)?
        };
        Ok((v, self))
    }
}

impl<'a, 'de, RS> de::SeqAccess<'de> for &'a mut EnumDeAccess<RS>
where
    RS: io::Read + io::Seek + fmt::Debug,
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
        trace!(
            "accessing array element: cur={}, seq_framing_start={}, seq_length={}",
            cur,
            self.seq_framing_start,
            self.seq_length
        );
        let mut seq_de = EnumDeserializer {
            cur_field: &self.cur_field,
            end: &mut self.end,
            fields: self.variants,
            options: self.options.clone(),
            reader: &mut self.reader,
            seq_fixed_width: &mut self.seq_fixed_width,
            seq_framing_start: &mut self.seq_framing_start,
            seq_length: &mut self.seq_length,
            signature: &mut self.signature,
        };
        let v = de::DeserializeSeed::deserialize(seed, &mut seq_de)?;
        Ok(Some(v))
    }
}

impl<'a, 'de, RS> de::VariantAccess<'de> for &'a mut EnumDeAccess<RS>
where
    RS: io::Read + io::Seek + fmt::Debug,
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
            "accessing newtype: cur={}, sig={}, fixed={}, end={}, seq_length={}, seq_fs={}",
            self.cur_field,
            self.signature[self.cur_field] as char,
            self.seq_fixed_width,
            self.end,
            self.seq_length,
            self.seq_framing_start,
        );

        let mut seq_de = EnumDeserializer {
            cur_field: &self.cur_field,
            end: &mut self.end,
            fields: self.variants,
            options: self.options.clone(),
            reader: &mut self.reader,
            seq_fixed_width: &mut self.seq_fixed_width,
            seq_framing_start: &mut self.seq_framing_start,
            seq_length: &mut self.seq_length,
            signature: &mut self.signature,
        };
        de::DeserializeSeed::deserialize(seed, &mut seq_de)
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
pub(crate) struct EnumDeserializer<'a, RS: fmt::Debug> {
    pub(crate) cur_field: &'a usize,
    pub(crate) end: &'a mut u64,
    pub(crate) fields: &'static [&'static str],
    pub(crate) options: config::Config,
    pub(crate) reader: RS,
    pub(crate) signature: &'a mut [u8],
    pub(crate) seq_framing_start: &'a mut u64,
    pub(crate) seq_fixed_width: &'a mut bool,
    pub(crate) seq_length: &'a mut u64,
}

impl<'de, 'a, RS> de::Deserializer<'de> for &'a mut EnumDeserializer<'a, RS>
where
    RS: fmt::Debug + io::Read + io::Seek,
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

        if *self.seq_length == 0 {
            let mut buf: Vec<u8> = Vec::new();
            let buflen = self.reader.read_to_end(&mut buf)?;
            trace!("string: buflen={}", buflen);
            let mut top = TopDeserializer {
                reader: io::Cursor::new(buf),
                options: self.options.clone(),
            };
            let v = top.deserialize_string(visitor)?;
            Ok(v)
        } else {
            *self.seq_fixed_width = false;
            let start = self.reader.seek(io::SeekFrom::Current(0))?;
            self.reader
                .seek(io::SeekFrom::Start(*self.seq_framing_start))?;
            let end = self.reader.read_u8()? as u64;
            *self.seq_framing_start = self.seq_framing_start.saturating_add(1);
            let buflen = (end - start) as usize;
            trace!("string: start={}, end={}, buflen={}, fs={}", start, end, buflen, self.seq_framing_start);

            self.reader.seek(io::SeekFrom::Start(start))?;
            let mut buf = vec![0u8; buflen];
            self.reader.read_exact(&mut buf).chain_err(|| "seq string")?;
            let mut top = TopDeserializer {
                reader: io::Cursor::new(buf),
                options: self.options.clone(),
            };
            let v = top.deserialize_string(visitor)?;
            Ok(v)
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let length = self.reader.read_to_end(&mut buf)?;
        let fstart = buf.last().cloned().unwrap_or(0) as u64;
        *self.seq_framing_start = fstart;
        trace!("seq: buflen={}, framing_start={}", length, fstart);

        let fixed = match self.signature[1] {
            b'y' => true,
            b'b' => true,
            _ => false,
        };

        let mut sub = EnumDeAccess {
            cur_field: 0,
            signature: self.signature[1..].to_vec(),
            name: "seq",
            end: length as u64,
            variants: &[],
            seq_framing_start: fstart,
            seq_fixed_width: fixed,
            seq_length: length as u64,
            options: self.options.clone(),
            reader: io::Cursor::new(buf),
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
        trace!(
            "variant: name={}, sig={}",
            enumer,
            self.signature[*self.cur_field] as char
        );
        let mut sub = EnumDeAccess {
            cur_field: 0,
            end: *self.end,
            name: enumer,
            variants: variants,
            options: self.options.clone(),
            reader: &mut self.reader,
            signature: self.signature[*self.cur_field..].to_vec(),
            seq_fixed_width: true,
            seq_framing_start: *self.seq_framing_start,
            seq_length: *self.seq_length,
        };
        if *self.seq_length != 0 {
            *self.seq_framing_start = self.seq_framing_start.saturating_add(1);
        }
        visitor.visit_enum(&mut sub)
    }

    fn deserialize_option<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let len = self.reader.read_to_end(&mut buf)?;
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
                    options: self.options.clone(),
                    reader: io::Cursor::new(buf),
                };
                visitor.visit_some(&mut sub)
            }
        }
    }

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
        let charsig = self.signature.first().unwrap().clone();
        trace!("variant: got id={}", charsig as char);
        let (id, fixed_width) = match charsig {
            b'a' => (11, false),
            b'b' => (0, true),
            b's' => (9, false),
            b'y' => (1, true),
            c => {
                return Err(Self::Error::custom(format!(
                    "variant: unrecognized signature {}",
                    c as char
                )))
            }
        };
        *self.seq_fixed_width = fixed_width;
        visitor.visit_u8(id)
    }
}
