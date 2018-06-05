use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use config;
use errors::{self, ResultExt};
use serde::{self, ser, ser::Error};
use std::io;

pub(crate) struct Serializer<W> {
    pub(crate) writer: W,
    pub(crate) options: config::Config,
}

impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: io::Write,
{
    type Ok = ();

    type Error = errors::Error;

    type SerializeSeq = ser::Impossible<(), Self::Error>;
    type SerializeTuple = ser::Impossible<(), Self::Error>;
    type SerializeTupleStruct = ser::Impossible<(), Self::Error>;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = ser::Impossible<(), Self::Error>;
    type SerializeStruct = ser::Impossible<(), Self::Error>;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_unit(self) -> errors::Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> errors::Result<Self::Ok> {
        Ok(())
    }

    fn serialize_bool(self, v: bool) -> errors::Result<Self::Ok> {
        let byte = match v {
            false => 0u8,
            true => 1u8,
        };
        self.writer
            .write_u8(byte)
            .chain_err(|| "failed to serialize bool")
    }

    fn serialize_u8(self, v: u8) -> errors::Result<Self::Ok> {
        self.writer
            .write_u8(v)
            .chain_err(|| "failed to serialize u8")
    }

    fn serialize_u16(self, v: u16) -> errors::Result<Self::Ok> {
        if self.options.little_endian {
            self.writer
                .write_u16::<LittleEndian>(v)
                .chain_err(|| "failed to serialize u16")
        } else {
            self.writer
                .write_u16::<BigEndian>(v)
                .chain_err(|| "failed to serialize u16")
        }
    }

    fn serialize_u32(self, v: u32) -> errors::Result<Self::Ok> {
        if self.options.little_endian {
            self.writer
                .write_u32::<LittleEndian>(v)
                .chain_err(|| "failed to serialize u32")
        } else {
            self.writer
                .write_u32::<BigEndian>(v)
                .chain_err(|| "failed to serialize u32")
        }
    }

    fn serialize_u64(self, v: u64) -> errors::Result<Self::Ok> {
        if self.options.little_endian {
            self.writer
                .write_u64::<LittleEndian>(v)
                .chain_err(|| "failed to serialize u64")
        } else {
            self.writer
                .write_u64::<BigEndian>(v)
                .chain_err(|| "failed to serialize u64")
        }
    }

    fn serialize_i8(self, v: i8) -> errors::Result<Self::Ok> {
        self.writer
            .write_i8(v)
            .chain_err(|| "failed to serialize u8")
    }

    fn serialize_i16(self, v: i16) -> errors::Result<Self::Ok> {
        if self.options.little_endian {
            self.writer
                .write_i16::<LittleEndian>(v)
                .chain_err(|| "failed to serialize i16")
        } else {
            self.writer
                .write_i16::<BigEndian>(v)
                .chain_err(|| "failed to serialize i16")
        }
    }

    fn serialize_i32(self, v: i32) -> errors::Result<Self::Ok> {
        if self.options.little_endian {
            self.writer
                .write_i32::<LittleEndian>(v)
                .chain_err(|| "failed to serialize i32")
        } else {
            self.writer
                .write_i32::<BigEndian>(v)
                .chain_err(|| "failed to serialize i32")
        }
    }

    fn serialize_i64(self, v: i64) -> errors::Result<Self::Ok> {
        if self.options.little_endian {
            self.writer
                .write_i64::<LittleEndian>(v)
                .chain_err(|| "failed to serialize i64")
        } else {
            self.writer
                .write_i64::<BigEndian>(v)
                .chain_err(|| "failed to serialize i64")
        }
    }

    fn serialize_f32(self, v: f32) -> errors::Result<Self::Ok> {
        let double = f64::from(v);
        if self.options.little_endian {
            self.writer
                .write_f64::<LittleEndian>(double)
                .chain_err(|| "failed to serialize f64")
        } else {
            self.writer
                .write_f64::<BigEndian>(double)
                .chain_err(|| "failed to serialize f64")
        }
    }

    fn serialize_f64(self, v: f64) -> errors::Result<Self::Ok> {
        if self.options.little_endian {
            self.writer
                .write_f64::<LittleEndian>(v)
                .chain_err(|| "failed to serialize f64")
        } else {
            self.writer
                .write_f64::<BigEndian>(v)
                .chain_err(|| "failed to serialize f64")
        }
    }

    fn serialize_str(self, v: &str) -> errors::Result<Self::Ok> {
        for b in v.as_bytes() {
            self.writer
                .write_u8(*b)
                .chain_err(|| "failed to serialize string character")?;
        }
        self.writer
            .write_u8(0u8)
            .chain_err(|| "failed to serialize string terminator")
    }

    fn serialize_char(self, _c: char) -> errors::Result<Self::Ok> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> errors::Result<Self::Ok> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_none(self) -> errors::Result<Self::Ok> {
        Err(Self::Error::custom("unsupported"))
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> errors::Result<Self::Ok>
    where
        T: ser::Serialize,
    {
        Err(Self::Error::custom("unsupported"))
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
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Self::Error::custom("unsupported"))
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
