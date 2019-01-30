use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use de::seq::SeqDeAccess;
use de::some::SomeDeserializer;
use de::struc::StructDeAccess;
use de::variant::EnumDeAccess;
use errors;
use serde::de::{self, Error};
use std::io;

#[derive(Debug)]
pub(crate) struct CursorDeserializer<'a, RS: 'a> {
    pub(crate) start: u64,
    pub(crate) end: u64,
    pub(crate) top: &'a mut ::de::top::TopDeserializer<RS>,
}

impl<'de, 'a, RS> de::Deserializer<'de> for &'a mut CursorDeserializer<'a, RS>
where
    RS: io::Read + io::Seek,
{
    type Error = errors::Error;

    fn deserialize_any<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("cursor: any not supported"))
    }

    fn deserialize_bool<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let byte = self.top.reader.read_u8()?;
        self.start += 1;
        let res = match byte {
            0 => false,
            _ => true,
        };
        visitor.visit_bool(res)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = self.top.reader.read_i8()?;
        self.start += 1;
        visitor.visit_i8(res)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.top.options.network_endian {
            self.top.reader.read_i16::<BigEndian>()?
        } else {
            self.top.reader.read_i16::<LittleEndian>()?
        };
        self.start += 2;
        visitor.visit_i16(res)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.top.options.network_endian {
            self.top.reader.read_i32::<BigEndian>()?
        } else {
            self.top.reader.read_i32::<LittleEndian>()?
        };
        self.start += 4;
        visitor.visit_i32(res)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.top.options.network_endian {
            self.top.reader.read_i64::<BigEndian>()?
        } else {
            self.top.reader.read_i64::<LittleEndian>()?
        };
        self.start += 8;
        visitor.visit_i64(res)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = self.top.reader.read_u8()?;
        self.start += 1;
        visitor.visit_u8(res)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.top.options.network_endian {
            self.top.reader.read_u16::<BigEndian>()?
        } else {
            self.top.reader.read_u16::<LittleEndian>()?
        };
        self.start += 2;
        visitor.visit_u16(res)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.top.options.network_endian {
            self.top.reader.read_u32::<BigEndian>()?
        } else {
            self.top.reader.read_u32::<LittleEndian>()?
        };
        self.start += 4;
        visitor.visit_u32(res)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.top.options.network_endian {
            self.top.reader.read_u64::<BigEndian>()?
        } else {
            self.top.reader.read_u64::<LittleEndian>()?
        };
        self.start += 8;
        visitor.visit_u64(res)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("cursor: unsupported f32"))
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let res = if self.top.options.network_endian {
            self.top.reader.read_f64::<BigEndian>()?
        } else {
            self.top.reader.read_f64::<LittleEndian>()?
        };
        self.start += 8;
        visitor.visit_f64(res)
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let buflen = self.end.checked_sub(self.start).ok_or_else(|| {
            Self::Error::custom(format!(
                "cursor: string length underflow - end={:#x}, start={:#x}",
                self.end, self.start
            ))
        })? as usize;
        if buflen > self.top.options.max_string_len {
            return Err(Self::Error::custom(format!(
                "cursor: overlong string, length={}",
                buflen
            )));
        };
        if buflen == 0 {
            return visitor.visit_string("".to_string());
        };

        let mut buf = vec![0; buflen];
        self.top.reader.read_exact(&mut buf)?;
        let strlen = buf
            .iter()
            .position(|x| x == &b'\0')
            .ok_or_else(|| Self::Error::custom("cursor: non-terminated string"))?;
        let s = String::from_utf8_lossy(&buf[..strlen]).into_owned();
        trace!(
            "got string: buflen={:#x}, strlen={:#x}, string='{}'",
            buflen,
            strlen,
            s
        );
        self.start += buflen as u64;
        visitor.visit_string(s)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let buflen = self.end.checked_sub(self.start).ok_or_else(|| {
            Self::Error::custom(format!(
                "byte_buf length underflow - end={:#x}, start={:#x}",
                self.end, self.start
            ))
        })? as usize;
        let mut buf = vec![0; buflen];
        self.top.reader.read_exact(&mut buf)?;
        self.start += buflen as u64;
        visitor.visit_byte_buf(buf)
    }

    fn deserialize_option<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let buflen = self.end.checked_sub(self.start).ok_or_else(|| {
            Self::Error::custom(format!(
                "cursor: option length underflow - end={:#x}, start={:#x}",
                self.end, self.start
            ))
        })? as usize;

        match buflen {
            // Fixed-Size inner: empty byte sequence.
            // Non-Fixed-Size inner: empty byte sequence.
            0 => visitor.visit_none(),

            // Fixed-Size inner: just data.
            // Non-Fixed-Size inner: data + 0x00.
            _ => {
                let mut sub = SomeDeserializer {
                    _len: buflen,
                    end: &mut self.end,
                    top: self.top,
                };
                self.start += buflen as u64;
                visitor.visit_some(&mut sub)
            }
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let buflen = self
            .end
            .checked_sub(self.start)
            .ok_or_else(|| Self::Error::custom("cursor: array length underflow"))?;

        // If items are variable-sized, record where the last one ends.
        // That is, where the framing offsets start.
        let fstart = match buflen {
            0 => 0u64,
            _ => {
                self.top.reader.seek(io::SeekFrom::End(-1))?;
                let b = self.top.reader.read_u8()?;
                self.top.reader.seek(io::SeekFrom::Start(self.start))?;
                u64::from(b)
            }
        };

        trace!(
            "SeqDe: start={:#x}, end={:#x}, length={:#x}",
            self.start,
            self.end,
            buflen,
        );
        let mut sub = SeqDeAccess {
            start: self.start,
            end: self.end,
            seq_framing_start: fstart,
            seq_fixed_width: true,
            seq_length: buflen,
            top: self.top,
        };
        self.start += buflen as u64;
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
        let buflen = self
            .end
            .checked_sub(self.start)
            .ok_or_else(|| Self::Error::custom("option length underflow"))?;

        trace!(
            "StructDe: name={}, num_fields={}, start={:#x}, end={:#x}, length={:#x}",
            name,
            fields.len(),
            self.start,
            self.end,
            buflen,
        );
        let mut sub = StructDeAccess {
            cur_field: 0,
            start: self.start,
            end: buflen,
            _name: name,
            fields,
            top: self.top,
        };
        self.start += buflen as u64;
        visitor.visit_seq(&mut sub)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let buflen = self
            .end
            .checked_sub(self.start)
            .ok_or_else(|| Self::Error::custom("string length underflow"))?;
        let mut payload_len = buflen;
        let mut sig: Vec<u8> = Vec::new();
        while payload_len > 0 {
            payload_len -= 1;
            let _cur = self
                .top
                .reader
                .seek(io::SeekFrom::Start(self.start + payload_len))?;
            let charsig = self.top.reader.read_u8()?;
            if charsig == 0x00 {
                break;
            }
            sig.insert(0, charsig);
        }
        let _cur = self.top.reader.seek(io::SeekFrom::Start(self.start))?;

        trace!(
            "EnumDe: name={}, sig={}, start={:#x}, end={:#x}, length={:#x}, payload_length={:#x}",
            name,
            String::from_utf8_lossy(&sig),
            self.start,
            self.end,
            buflen,
            payload_len,
        );
        let mut start = self.start;
        let mut end = self.start + payload_len;
        let mut fstart = 0u64;
        let mut sub = EnumDeAccess {
            cur_field: 0,
            start: &mut start,
            end: &mut end,
            name,
            variants,
            top: self.top,
            signature: sig,
            seq_fixed_width: true,
            seq_framing_start: &mut fstart,
            seq_length: 0,
            seq_start: self.start,
        };
        self.start += buflen as u64;
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
        Err(Self::Error::custom("cursor: unit_struct not supported"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("cursor: newtype_struct not supported"))
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("cursor: unit not supported"))
    }

    forward_to_deserialize_any! {
        identifier ignored_any map char bytes str
    }
}
