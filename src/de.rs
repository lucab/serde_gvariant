use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use config;
use errors::{self, ResultExt};
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
            let byte = self.reader.read_u8().chain_err(|| "string u8")?;
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
                let mut sub = SomeDeserializer {
                    _len: len,
                    options: self.options.clone(),
                    reader: buf.as_slice(),
                };
                visitor.visit_some(&mut sub)
            }
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let byte = self.reader.read_u8()?;
        if byte != 0x00 {
            return Err(Self::Error::custom("wrong unit byte"));
        }
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

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let end = self.reader.read_to_end(&mut buf)?;
        let mut sub = SeqDeAccess {
            end: end as u64,
            options: self.options.clone(),
            reader: io::Cursor::new(buf),
        };
        visitor.visit_seq(&mut sub)
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
        fields: &'static [&'static str],
        visitor: V,
    ) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let end = self.reader.read_to_end(&mut buf)?;
        let mut sub = StructDeAccess {
            cur_field: 0,
            end: end as u64,
            num_fields: fields.len(),
            options: self.options.clone(),
            reader: io::Cursor::new(buf),
        };
        visitor.visit_seq(&mut sub)
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

    forward_to_deserialize_any! {
            identifier ignored_any
    }
}

pub(crate) struct SeqDeAccess<RS> {
    pub(crate) end: u64,
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
        // Stop if EOF is reached
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        if self.end == cur {
            return Ok(None);
        }

        // Deserialize next element
        let mut seq_de = SeqDeserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        let v = de::DeserializeSeed::deserialize(seed, &mut seq_de)?;
        Ok(Some(v))
    }
}

// A Deserializer specialized on array, with custom logic
// for non-fized-size ones.
pub(crate) struct SeqDeserializer<RS> {
    pub(crate) options: config::Config,
    pub(crate) reader: RS,
}

impl<'de, 'a, RS> de::Deserializer<'de> for &'a mut SeqDeserializer<RS>
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
            option newtype_struct
    }

    // Fixed size
    fn deserialize_bool<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i8(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
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
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        let mut buf = Vec::with_capacity(buflen);
        unsafe { buf.set_len(buflen) };
        self.reader.read_exact(&mut buf).chain_err(|| "seq string")?;
        let value = {
            let mut top = Deserializer {
                reader: buf.as_slice(),
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
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        let mut buf = Vec::with_capacity(buflen);
        unsafe { buf.set_len(buflen) };
        self.reader.read_exact(&mut buf).chain_err(|| "seq seq")?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
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
        let cur = self.reader.seek(io::SeekFrom::Current(0))?;
        self.reader.seek(io::SeekFrom::End(-1))?;
        let end = self.reader.read_u8()? as u64;
        self.reader.seek(io::SeekFrom::Start(cur))?;
        let buflen = (end - cur) as usize;
        let mut buf = Vec::with_capacity(buflen);
        unsafe { buf.set_len(buflen) };
        self.reader.read_exact(&mut buf).chain_err(|| "seq seq")?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        let v = top.deserialize_struct(name, fields, visitor)?;
        Ok(v)
    }
}

pub(crate) struct SomeDeserializer<R> {
    pub(crate) _len: usize,
    pub(crate) options: config::Config,
    pub(crate) reader: R,
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut SomeDeserializer<R>
where
    R: io::Read,
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

    // Fixed size
    fn deserialize_bool<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_bool(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_u8(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_i8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_u16(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_f64(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let term = buf.pop().unwrap();
        if term != 0x00 {
            return Err(Self::Error::custom("string non-zero terminator"));
        }

        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_string(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut buf: Vec<u8> = Vec::new();
        let _len = self.reader.read_to_end(&mut buf)?;
        let term = buf.pop().unwrap();
        if term != 0x00 {
            return Err(Self::Error::custom("byte_buf non-zero terminator"));
        }

        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        top.deserialize_byte_buf(visitor)
    }

    // Pending implementation
    forward_to_deserialize_any! {
        char str enum bytes
        unit unit_struct seq tuple tuple_struct map
        option newtype_struct struct
    }
}

pub(crate) struct StructDeAccess<RS> {
    pub(crate) cur_field: usize,
    pub(crate) end: u64,
    pub(crate) num_fields: usize,
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
        if self.cur_field >= self.num_fields {
            return Ok(None);
        }

        // Deserialize next element
        let v = {
            let mut seq_de = StructDeserializer {
                cur_field: &self.cur_field,
                end: &mut self.end,
                num_fields: &self.num_fields,
                reader: &mut self.reader,
                options: self.options.clone(),
            };
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
    pub(crate) num_fields: &'a usize,
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
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i8(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i16(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i32(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_i64(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
            reader: &mut self.reader,
            options: self.options.clone(),
        };
        top.deserialize_u64(visitor)
    }

    fn deserialize_f64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut top = Deserializer {
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
        let end = if (*self.cur_field + 1) >= *self.num_fields {
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
        let mut top = Deserializer {
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
        let end = if (*self.cur_field + 1) >= *self.num_fields {
            *self.end as u64
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
        self.reader
            .read_exact(&mut buf)
            .chain_err(|| "struct seq")?;
        let mut top = Deserializer {
            reader: buf.as_slice(),
            options: self.options.clone(),
        };
        let v = top.deserialize_seq(visitor)?;
        Ok(v)
    }
}
