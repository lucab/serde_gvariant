use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use config;
use errors;
use serde::de::{self, Error};
use std::io;

pub(crate) struct Deserializer<R> {
    pub(crate) reader: R,
    pub(crate) options: config::Config,
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut Deserializer<R>
where
    R: io::Read,
{
    type Error = errors::Error;

    fn deserialize_any<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
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
        visitor.visit_bool(res)
    }

    fn deserialize_char<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
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
        Err(Self::Error::custom("unsupported f32"))
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

    fn deserialize_str<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO(lucab): investigate borrowing.
        Err(Self::Error::custom("unsupported"))
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO(lucab): consider a bufreader.
        let mut buf = Vec::with_capacity(self.options.max_string_len as usize);
        for _ in 0..buf.capacity() {
            let byte = self.reader.read_u8()?;
            if byte == 0 {
                break;
            }
            buf.push(byte);
        }
        let res = String::from_utf8_lossy(&buf).into_owned();
        visitor.visit_string(res)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO(lucab): investigate borrowing.
        Err(Self::Error::custom("unsupported"))
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
        let mut buf: Vec<u8> = Vec::new();
        let len = self.reader.read_to_end(&mut buf)?;
        match len {
            // Fixed-Size inner: empty byte sequence.
            // Non-Fixed-Size inner: empty byte sequence.
            0 => visitor.visit_none(),

            // Fixed-Size inner: just data.
            // Non-Fixed-Size inner: data + 0x00.
            _ => {
                // TODO(lucab): fix non-fixed trailing byte.
                let mut sub = Deserializer {
                    reader: buf.as_slice(),
                    options: self.options.clone(),
                };
                visitor.visit_some(&mut sub)
            }
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &str, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
    }

    fn deserialize_enum<V>(
        self,
        _enum: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Self::Error::custom("unsupported"))
    }
}
