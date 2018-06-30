use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use config;
use de::seq::SeqDeAccess;
use de::some::SomeDeserializer;
use de::struc::StructDeAccess;
use de::variant::EnumDeAccess;
use errors::{self, ResultExt};
use serde::de::{self, Error};
use std::io;

pub(crate) struct TopDeserializer<RS> {
    pub(crate) reader: RS,
    pub(crate) options: config::Config,
}

impl<'de, 'a, RS> de::Deserializer<'de> for &'a mut TopDeserializer<RS>
where
    RS: io::Read + io::Seek,
{
    type Error = errors::Error;

    fn deserialize_any<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("top: any not supported"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let byte = self.reader.read_u8()?;
        let res = match byte {
            0 => false,
            _ => true,
        };
        trace!("got bool: {}", res);
        visitor.visit_bool(res)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = self.reader.read_i8()?;
        visitor.visit_i8(res)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.options.network_endian {
            self.reader.read_i16::<BigEndian>()?
        } else {
            self.reader.read_i16::<LittleEndian>()?
        };
        visitor.visit_i16(res)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.options.network_endian {
            self.reader.read_i32::<BigEndian>()?
        } else {
            self.reader.read_i32::<LittleEndian>()?
        };
        visitor.visit_i32(res)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.options.network_endian {
            self.reader.read_i64::<BigEndian>()?
        } else {
            self.reader.read_i64::<LittleEndian>()?
        };
        visitor.visit_i64(res)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = self.reader.read_u8()?;
        visitor.visit_u8(res)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.options.network_endian {
            self.reader.read_u16::<BigEndian>()?
        } else {
            self.reader.read_u16::<LittleEndian>()?
        };
        visitor.visit_u16(res)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.options.network_endian {
            self.reader.read_u32::<BigEndian>()?
        } else {
            self.reader.read_u32::<LittleEndian>()?
        };
        visitor.visit_u32(res)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.options.network_endian {
            self.reader.read_u64::<BigEndian>()?
        } else {
            self.reader.read_u64::<LittleEndian>()?
        };
        visitor.visit_u64(res)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("top: unsupported f32"))
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.options.network_endian {
            self.reader.read_f64::<BigEndian>()?
        } else {
            self.reader.read_f64::<LittleEndian>()?
        };
        visitor.visit_f64(res)
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO(lucab): consider a bufreader.
        let mut buf = Vec::with_capacity(self.options.max_string_len as usize);
        for _ in 0..buf.capacity() {
            let byte = self.reader.read_u8().chain_err(|| "string u8")?;
            if byte == 0 {
                break;
            }
            buf.push(byte);
        }
        let res = String::from_utf8_lossy(&buf).into_owned();
        trace!("got string: len={:#x}", buf.len());
        visitor.visit_string(res)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        visitor.visit_byte_buf(buf)
    }

    fn deserialize_option<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let start = self.reader.seek(io::SeekFrom::Current(0))?;
        let end = self.reader.seek(io::SeekFrom::End(0))?;
        let _cur = self.reader.seek(io::SeekFrom::Start(start))?;
        let buflen = end.checked_sub(start)
            .ok_or_else(|| Self::Error::custom("option length underflow"))?
            as usize;

        match buflen {
            // Fixed-Size inner: empty byte sequence.
            // Non-Fixed-Size inner: empty byte sequence.
            0 => visitor.visit_none(),

            // Fixed-Size inner: just data.
            // Non-Fixed-Size inner: data + 0x00.
            _ => {
                let mut sub = SomeDeserializer {
                    _len: buflen,
                    options: self.options.clone(),
                    reader: &mut self.reader,
                };
                visitor.visit_some(&mut sub)
            }
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let start = self.reader.seek(io::SeekFrom::Current(0))?;
        let end = self.reader.seek(io::SeekFrom::End(0))?;
        let _cur = self.reader.seek(io::SeekFrom::Start(start))?;
        let buflen = end.checked_sub(start)
            .ok_or_else(|| Self::Error::custom("array length underflow"))?;

        // If items are variable-sized, record where the last one ends.
        // That is, where the framing offsets start.
        let fstart = match buflen {
            0 => 0u64,
            _ => {
                self.reader.seek(io::SeekFrom::End(-1))?;
                let b = self.reader.read_u8()?;
                self.reader.seek(io::SeekFrom::Start(start))?;
                b as u64
            }
        };

        trace!(
            "SeqDe: start={:#x}, end={:#x}, length={:#x}",
            start,
            end,
            buflen,
        );
        let mut sub = SeqDeAccess {
            seq_framing_start: fstart,
            seq_fixed_width: true,
            seq_length: buflen,
            options: self.options.clone(),
            reader: &mut self.reader,
        };
        visitor.visit_seq(&mut sub)
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
        trace!("tuple_struct -> seq");
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
        let start = self.reader.seek(io::SeekFrom::Current(0))?;
        let end = self.reader.seek(io::SeekFrom::End(0))?;
        let _cur = self.reader.seek(io::SeekFrom::Start(start))?;
        let buflen = end.checked_sub(start)
            .ok_or_else(|| Self::Error::custom("option length underflow"))?;

        trace!(
            "StructDe: name={}, num_fields={}, start={:#x}, end={:#x}, length={:#x}",
            name,
            fields.len(),
            start,
            end,
            buflen,
        );
        let mut sub = StructDeAccess {
            cur_field: 0,
            end: buflen,
            _name: name,
            fields: fields,
            options: self.options.clone(),
            reader: &mut self.reader,
        };
        visitor.visit_seq(&mut sub)
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
        let mut buf: Vec<u8> = Vec::new();
        let mut sig: Vec<u8> = Vec::new();
        let mut length = self.reader.read_to_end(&mut buf)?;
        while let Some(charsig) = buf.pop() {
            length = length.saturating_sub(1);
            if charsig == 0x00 {
                break;
            }
            sig.insert(0, charsig);
        }
        trace!(
            "EnumDe: name={}, sig={}, length={}",
            enumer,
            String::from_utf8_lossy(&sig),
            length
        );

        let mut sub = EnumDeAccess {
            cur_field: 0,
            end: length as u64,
            name: enumer,
            variants: variants,
            options: self.options.clone(),
            reader: io::Cursor::new(buf),
            signature: sig,
            seq_fixed_width: true,
            seq_framing_start: 0,
            seq_length: 0,
        };
        visitor.visit_enum(&mut sub)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("top: unit_struct not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("top: newtype_struct not supported"))
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("top: unit not supported"))
    }

    forward_to_deserialize_any! {
            identifier ignored_any map char bytes str
    }
}
