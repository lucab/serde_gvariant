use crate::config;
use crate::errors::{self, ResultExt};
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use serde::Serialize;
use serde::{self, ser, ser::Error};
use std::io;

#[derive(Debug)]
pub(crate) struct Properties {
    pub(crate) fixed_size: bool,
    pub(crate) size: u64,
}

#[derive(Debug)]
pub(crate) struct SerSeq<'a, W: 'a> {
    pub(crate) cur_offset: u64,
    pub(crate) framing_offsets: Vec<u64>,
    // PERFOPT(lucab): write-once
    pub(crate) fixed_size: bool,
    pub(crate) serializer: &'a mut Serializer<W>,
    pub(crate) size: u64,
}

impl<'a, W> ser::SerializeSeq for SerSeq<'a, W>
where
    W: io::Write,
{
    type Ok = Properties;
    type Error = errors::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> errors::Result<()>
    where
        T: Serialize,
    {
        // Serialize this element
        let p = value
            .serialize(&mut *self.serializer)
            .chain_err(|| "failed to serialize array element")?;

        // Update total array size
        self.size = self
            .size
            .checked_add(p.size)
            .ok_or_else(|| Self::Error::custom("array length overflowed"))?;

        // Update current position/offset
        self.cur_offset = self
            .cur_offset
            .checked_add(p.size)
            .ok_or_else(|| Self::Error::custom("current offset overflowed"))?;

        // Record whether elements are fixed size
        self.fixed_size = p.fixed_size;

        // If element is variable-sized, records where it ends
        if !p.fixed_size {
            self.framing_offsets.push(self.cur_offset);
            // Add value length to total size
            self.size = self
                .size
                .checked_add(1)
                .ok_or_else(|| Self::Error::custom("array length overflowed"))?;
        }

        Ok(())
    }

    // TODO(lucab): lengths longer that u8::MAX
    fn end(self) -> errors::Result<Properties> {
        // If variable-sized, append all framings offsets.
        // Framing offsets are unaligned and little-endian.
        for off in self.framing_offsets {
            self.serializer
                .writer
                .write_u8(off as u8)
                .chain_err(|| "failed to serialize array framings")?;
        }

        let p = Properties {
            fixed_size: self.fixed_size,
            // Total size already accounts for framings too.
            size: self.size,
        };
        Ok(p)
    }
}

#[derive(Debug)]
pub(crate) struct SerStruct<'a, W: 'a> {
    pub(crate) cur_field: u64,
    pub(crate) cur_offset: u64,
    pub(crate) framing_offsets: Vec<u64>,
    pub(crate) name: String,
    pub(crate) num_fields: u64,
    pub(crate) serializer: &'a mut Serializer<W>,
}

impl<'a, W> ser::SerializeStruct for SerStruct<'a, W>
where
    W: io::Write,
{
    type Ok = Properties;
    type Error = errors::Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> errors::Result<()>
    where
        T: Serialize,
    {
        // Serialize this field
        let p = value.serialize(&mut *self.serializer)?;
        self.cur_field = self
            .cur_field
            .checked_add(1)
            .ok_or_else(|| Self::Error::custom("field count overflowed"))?;
        self.cur_offset = self
            .cur_offset
            .checked_add(p.size)
            .ok_or_else(|| Self::Error::custom("current offset overflowed"))?;

        // If variable-sized and not the last field, records where it ends
        let last = self.cur_field == self.num_fields;
        if !p.fixed_size && !last {
            self.framing_offsets.push(self.cur_offset);
        }

        Ok(())
    }

    fn end(self) -> errors::Result<Properties> {
        // Fixed size
        if self.framing_offsets.is_empty() {
            let p = Properties {
                fixed_size: true,
                size: self.cur_offset as u64,
            };
            return Ok(p);
        };

        // Non-fixed size, append all framings offsets except the last one.
        // Framing offsets are unaligned and little-endian.
        let size = self.framing_offsets.last().cloned().unwrap();
        if size > u64::from(::std::u8::MAX) {
            return Err(Self::Error::custom("unsupported"));
        }
        for off in self.framing_offsets {
            self.serializer.writer.write_u8(off as u8)?;
        }

        let p = Properties {
            fixed_size: false,
            size: size as u64,
        };
        Ok(p)
    }
}

#[derive(Debug)]
pub(crate) struct Serializer<W> {
    pub(crate) current_pos: u64,
    pub(crate) writer: W,
    pub(crate) options: config::Config,
}

impl<W> Serializer<W>
where
    W: io::Write,
{
    fn pad_align(&mut self, alignment: u64) -> errors::Result<u64> {
        if alignment <= 1 {
            return Ok(0);
        }
        let padding = (alignment - (self.current_pos % alignment)) % alignment;
        // TODO(lucab): buffer writes
        for _ in 0..padding {
            self.writer.write_u8(0x00).chain_err(|| "failed to pad")?;
        }
        self.current_pos = self
            .current_pos
            .checked_add(padding)
            .ok_or_else(|| errors::Error::custom("alignment padding overflowed"))?;
        Ok(padding)
    }
}

impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = Properties;
    type Error = errors::Error;

    type SerializeSeq = SerSeq<'a, W>;
    type SerializeTuple = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeMap = ser::Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = SerStruct<'a, W>;
    type SerializeStructVariant = ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_unit(self) -> errors::Result<Self::Ok> {
        self.writer
            .write_u8(0x00)
            .chain_err(|| "failed to serialize unit")?;
        let p = Properties {
            fixed_size: true,
            size: 1,
        };
        Ok(p)
    }

    fn serialize_unit_struct(self, _: &'static str) -> errors::Result<Self::Ok> {
        self.writer
            .write_u8(0x00)
            .chain_err(|| "failed to serialize unit struct")?;
        let p = Properties {
            fixed_size: true,
            size: 1,
        };
        Ok(p)
    }

    fn serialize_bool(self, v: bool) -> errors::Result<Self::Ok> {
        let size = 1;
        let _pad = self.pad_align(size)?;
        let byte: u8 = if v { 1 } else { 0 };
        self.writer
            .write_u8(byte)
            .chain_err(|| "failed to serialize bool")?;
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_u8(self, v: u8) -> errors::Result<Self::Ok> {
        let size = 1;
        let _pad = self.pad_align(size)?;
        self.writer
            .write_u8(v)
            .chain_err(|| "failed to serialize u8")?;
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_u16(self, v: u16) -> errors::Result<Self::Ok> {
        let size = 2;
        let _pad = self.pad_align(size)?;
        if self.options.network_endian {
            self.writer
                .write_u16::<BigEndian>(v)
                .chain_err(|| "failed to serialize u16")?;
        } else {
            self.writer
                .write_u16::<LittleEndian>(v)
                .chain_err(|| "failed to serialize u16")?;
        }
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_u32(self, v: u32) -> errors::Result<Self::Ok> {
        let size = 4;
        let _pad = self.pad_align(size)?;
        if self.options.network_endian {
            self.writer
                .write_u32::<BigEndian>(v)
                .chain_err(|| "failed to serialize u32")?;
        } else {
            self.writer
                .write_u32::<LittleEndian>(v)
                .chain_err(|| "failed to serialize u32")?;
        }
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_u64(self, v: u64) -> errors::Result<Self::Ok> {
        let size = 8;
        let _pad = self.pad_align(size)?;
        if self.options.network_endian {
            self.writer
                .write_u64::<BigEndian>(v)
                .chain_err(|| "failed to serialize u64")?;
        } else {
            self.writer
                .write_u64::<LittleEndian>(v)
                .chain_err(|| "failed to serialize u64")?;
        }
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_i8(self, v: i8) -> errors::Result<Self::Ok> {
        let size = 1;
        let _pad = self.pad_align(size)?;
        self.writer
            .write_i8(v)
            .chain_err(|| "failed to serialize u8")?;
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_i16(self, v: i16) -> errors::Result<Self::Ok> {
        let size = 2;
        let _pad = self.pad_align(size)?;
        if self.options.network_endian {
            self.writer
                .write_i16::<BigEndian>(v)
                .chain_err(|| "failed to serialize i16")?;
        } else {
            self.writer
                .write_i16::<LittleEndian>(v)
                .chain_err(|| "failed to serialize i16")?;
        }
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_i32(self, v: i32) -> errors::Result<Self::Ok> {
        let size = 4;
        let _pad = self.pad_align(size)?;
        if self.options.network_endian {
            self.writer
                .write_i32::<BigEndian>(v)
                .chain_err(|| "failed to serialize i32")?;
        } else {
            self.writer
                .write_i32::<LittleEndian>(v)
                .chain_err(|| "failed to serialize i32")?;
        }
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_i64(self, v: i64) -> errors::Result<Self::Ok> {
        let size = 8;
        let _pad = self.pad_align(size)?;
        if self.options.network_endian {
            self.writer
                .write_i64::<BigEndian>(v)
                .chain_err(|| "failed to serialize i64")?;
        } else {
            self.writer
                .write_i64::<LittleEndian>(v)
                .chain_err(|| "failed to serialize i64")?;
        }
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_f32(self, v: f32) -> errors::Result<Self::Ok> {
        // Internally promote to f64.
        let size = 8;
        let _pad = self.pad_align(size)?;
        let double = f64::from(v);
        if self.options.network_endian {
            self.writer
                .write_f64::<BigEndian>(double)
                .chain_err(|| "failed to serialize f64")?;
        } else {
            self.writer
                .write_f64::<LittleEndian>(double)
                .chain_err(|| "failed to serialize f64")?;
        }
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_f64(self, v: f64) -> errors::Result<Self::Ok> {
        let size = 8;
        let _pad = self.pad_align(size)?;
        if self.options.network_endian {
            self.writer
                .write_f64::<BigEndian>(v)
                .chain_err(|| "failed to serialize f64")?;
        } else {
            self.writer
                .write_f64::<LittleEndian>(v)
                .chain_err(|| "failed to serialize f64")?;
        }
        let p = Properties {
            fixed_size: true,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_str(self, v: &str) -> errors::Result<Self::Ok> {
        let size =
            v.len()
                .checked_add(1)
                .ok_or_else(|| Self::Error::custom("string length overflowed"))? as u64;
        for b in v.as_bytes() {
            self.writer
                .write_u8(*b)
                .chain_err(|| "failed to serialize string character")?;
        }
        self.writer
            .write_u8(0x00)
            .chain_err(|| "failed to serialize string terminator")?;
        let p = Properties {
            fixed_size: false,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_char(self, _c: char) -> errors::Result<Self::Ok> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_bytes(self, v: &[u8]) -> errors::Result<Self::Ok> {
        let size = v.len() as u64;
        for b in v {
            self.writer
                .write_u8(*b)
                .chain_err(|| "failed to serialize byte element")?;
        }
        let p = Properties {
            fixed_size: false,
            size,
        };
        self.current_pos += size;
        Ok(p)
    }

    fn serialize_none(self) -> errors::Result<Self::Ok> {
        // Fixed-Size inner: empty byte sequence.
        // Non-Fixed-Size inner: empty byte sequence.
        let p = Properties {
            fixed_size: true,
            size: 0,
        };
        Ok(p)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> errors::Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        let buf: Vec<u8> = Vec::new();

        let mut first = Serializer {
            current_pos: self.current_pos,
            writer: buf,
            options: self.options.clone(),
        };

        // Fixed-Size inner: just data.
        // Non-Fixed-Size inner: data + 0x00.
        let mut prop = value.serialize(&mut first)?;
        if !prop.fixed_size {
            let terminator = 0u8;
            terminator.serialize(&mut first)?;
            prop.size = prop
                .size
                .checked_add(1)
                .ok_or_else(|| Self::Error::custom("option-some length overflowed"))?;
        };
        self.writer.write_all(&first.writer)?;
        self.current_pos += prop.size;
        Ok(prop)
    }

    fn serialize_seq(self, len_hint: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let framings = match len_hint {
            Some(l) => Vec::with_capacity(l),
            None => vec![],
        };

        let s = Self::SerializeSeq {
            cur_offset: 0,
            framing_offsets: framings,
            fixed_size: false,
            serializer: self,
            size: 0,
        };
        Ok(s)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let s = Self::SerializeStruct {
            cur_field: 0,
            cur_offset: 0,
            framing_offsets: vec![],
            name: name.to_string(),
            num_fields: len as u64,
            serializer: self,
        };
        Ok(s)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Self::Error::custom("unsupported: tuple variant"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Self::Error::custom("unsupported: map"))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Self::Error::custom("unsupported: struct variant"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> errors::Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        Err(Self::Error::custom("unsupported: newtype struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> errors::Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        Err(Self::Error::custom("unsupported: newtype variant"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> errors::Result<Self::Ok> {
        Err(Self::Error::custom("unsupported: unit variant"))
    }
}
