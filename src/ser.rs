use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use config;
use errors::{self, ResultExt};
use serde::Serialize;
use serde::{self, ser, ser::Error};
use std::io;

#[derive(Clone, Debug)]
pub(crate) struct Properties {
    pub(crate) fixed_size: bool,
    pub(crate) size: usize,
}

#[derive(Debug)]
pub(crate) struct Serializer<W> {
    pub(crate) writer: W,
    pub(crate) options: config::Config,
}

#[derive(Debug)]
pub(crate) struct SerStruct<'a, W: 'a> {
    pub(crate) cur_field: usize,
    pub(crate) cur_offset: usize,
    pub(crate) framing_offsets: Vec<usize>,
    pub(crate) name: String,
    pub(crate) num_fields: usize,
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
        self.cur_field += 1;
        self.cur_offset += p.size;

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
                size: self.cur_offset,
            };
            return Ok(p);
        };

        // Non-fixed size
        let size = self.framing_offsets.last().cloned().unwrap();
        if size > <u8>::max_value() as usize {
            return Err(Self::Error::custom("unsupported"));
        }

        for off in self.framing_offsets {
            self.serializer.writer.write_u8(off as u8)?;
        }
        let p = Properties {
            fixed_size: false,
            size: size,
        };
        Ok(p)
    }
}

impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = Properties;
    type Error = errors::Error;

    type SerializeSeq = ser::Impossible<Self::Ok, Self::Error>;
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
        let byte: u8 = if v { 1 } else { 0 };
        self.writer
            .write_u8(byte)
            .chain_err(|| "failed to serialize bool")?;
        let p = Properties {
            fixed_size: true,
            size: 1,
        };
        Ok(p)
    }

    fn serialize_u8(self, v: u8) -> errors::Result<Self::Ok> {
        self.writer
            .write_u8(v)
            .chain_err(|| "failed to serialize u8")?;
        let p = Properties {
            fixed_size: true,
            size: 1,
        };
        Ok(p)
    }

    fn serialize_u16(self, v: u16) -> errors::Result<Self::Ok> {
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
            size: 2,
        };
        Ok(p)
    }

    fn serialize_u32(self, v: u32) -> errors::Result<Self::Ok> {
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
            size: 3,
        };
        Ok(p)
    }

    fn serialize_u64(self, v: u64) -> errors::Result<Self::Ok> {
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
            size: 4,
        };
        Ok(p)
    }

    fn serialize_i8(self, v: i8) -> errors::Result<Self::Ok> {
        self.writer
            .write_i8(v)
            .chain_err(|| "failed to serialize u8")?;
        let p = Properties {
            fixed_size: true,
            size: 1,
        };
        Ok(p)
    }

    fn serialize_i16(self, v: i16) -> errors::Result<Self::Ok> {
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
            size: 2,
        };
        Ok(p)
    }

    fn serialize_i32(self, v: i32) -> errors::Result<Self::Ok> {
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
            size: 3,
        };
        Ok(p)
    }

    fn serialize_i64(self, v: i64) -> errors::Result<Self::Ok> {
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
            size: 4,
        };
        Ok(p)
    }

    fn serialize_f32(self, v: f32) -> errors::Result<Self::Ok> {
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
            size: 8,
        };
        Ok(p)
    }

    fn serialize_f64(self, v: f64) -> errors::Result<Self::Ok> {
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
            size: 8,
        };
        Ok(p)
    }

    fn serialize_str(self, v: &str) -> errors::Result<Self::Ok> {
        let len = v.len() + 1;
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
            size: len,
        };
        Ok(p)
    }

    fn serialize_char(self, _c: char) -> errors::Result<Self::Ok> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_bytes(self, v: &[u8]) -> errors::Result<Self::Ok> {
        let len = v.len();
        for b in v {
            self.writer
                .write_u8(*b)
                .chain_err(|| "failed to serialize byte element")?;
        }
        let p = Properties {
            fixed_size: false,
            size: len,
        };
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
        use serde::Serialize;
        let buf: Vec<u8> = Vec::new();

        let mut first = Serializer {
            writer: buf,
            options: self.options.clone(),
        };

        // Fixed-Size inner: just data.
        // Non-Fixed-Size inner: data + 0x00.
        let mut prop = value.serialize(&mut first)?;
        if !prop.fixed_size {
            let terminator = 0u8;
            terminator.serialize(&mut first)?;
            prop.size += 1;
        };
        self.writer.write(&first.writer)?;
        Ok(prop)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Self::Error::custom("unsupported"))
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

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
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
            num_fields: len,
            serializer: self,
        };
        Ok(s)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> errors::Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        Err(Self::Error::custom("unsupported"))
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
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> errors::Result<Self::Ok> {
        Err(Self::Error::custom("unsupported"))
    }
}
