use config;
use de::cursor::CursorDeserializer;
use errors;
use serde::de::{self, Error};
use std::io;

#[derive(Debug)]
pub(crate) struct TopDeserializer<RS> {
    pub(crate) reader: RS,
    pub(crate) options: config::Config,
}

impl<'a, RS> TopDeserializer<RS>
where
    RS: io::Read + io::Seek,
{
    pub(crate) fn forward(
        &'a mut self,
        kind: &'static str,
    ) -> errors::Result<CursorDeserializer<'a, RS>> {
        let start = self.reader.seek(io::SeekFrom::Current(0))?;
        let end = self.reader.seek(io::SeekFrom::End(0))?;
        let _cur = self.reader.seek(io::SeekFrom::Start(start))?;
        let _buflen = end
            .checked_sub(start)
            .ok_or_else(|| errors::Error::custom(format!("top: {} length underflow", kind)))?;

        let cd = CursorDeserializer {
            start,
            end,
            top: self,
        };
        Ok(cd)
    }
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
        let mut cd = self.forward("bool")?;
        cd.deserialize_bool(visitor)
    }

    fn deserialize_i8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("i8")?;
        cd.deserialize_i8(visitor)
    }

    fn deserialize_i16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("i16")?;
        cd.deserialize_i16(visitor)
    }

    fn deserialize_i32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("i32")?;
        cd.deserialize_i32(visitor)
    }

    fn deserialize_i64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("i64")?;
        cd.deserialize_i64(visitor)
    }

    fn deserialize_u8<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("u8")?;
        cd.deserialize_u8(visitor)
    }

    fn deserialize_u16<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("u16")?;
        cd.deserialize_u16(visitor)
    }

    fn deserialize_u32<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("u32")?;
        cd.deserialize_u32(visitor)
    }

    fn deserialize_u64<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("u64")?;
        cd.deserialize_u64(visitor)
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
        let mut cd = self.forward("f64")?;
        cd.deserialize_f64(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("string")?;
        cd.deserialize_string(visitor)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("byte_buf")?;
        cd.deserialize_byte_buf(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("option")?;
        cd.deserialize_option(visitor)
    }

    fn deserialize_seq<V>(self, visitor: V) -> errors::Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let mut cd = self.forward("seq")?;
        cd.deserialize_seq(visitor)
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
            bail!("top: too many fields in tuple");
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
        let mut cd = self.forward("struct")?;
        cd.deserialize_struct(name, fields, visitor)
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
        let mut cd = self.forward("enum")?;
        cd.deserialize_enum(name, variants, visitor)
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
