use errors::{self, ResultExt};
use serde;
use std::io;

#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) limit: Option<u32>,
    pub(crate) little_endian: bool,
}

impl Config {
    pub(crate) fn new() -> Config {
        Config {
            limit: None,
            little_endian: true,
        }
    }
}

impl Config {
    /// Serializes a serializable object into a `Vec` of bytes using this configuration
    pub fn serialize<T: ?Sized + serde::Serialize>(&self, t: &T) -> errors::Result<Vec<u8>> {
        let mut buf = vec![];
        {
            let mut serializer = ::ser::Serializer {
                writer: &mut buf,
                options: self.clone(),
            };
            serde::Serialize::serialize(t, &mut serializer).chain_err(|| "failed to serialize")?;
        }
        Ok(buf)
    }

    /// Deserializes a slice of bytes into an instance of `T` using this configuration
    pub fn deserialize_slice<'a, T: serde::Deserialize<'a>>(&self, bytes: &'a [u8]) -> errors::Result<T> {
        let reader = io::BufReader::with_capacity(bytes.len(), bytes);
        let mut deserializer = ::de::Deserializer {
            reader,
            options: self.clone(),
        };
        serde::Deserialize::deserialize(&mut deserializer)
            .chain_err(|| "failed to deserialize slice")
    }

    /// Deserializes an object directly from a `Read`er using this configuration
    pub fn deserialize_reader<R: io::Read, T: serde::de::DeserializeOwned>(
        &self,
        reader: R,
    ) -> errors::Result<T> {
        //let mut deserializer = ::de::Deserializer::<R>::new(reader, &self);
        let mut deserializer = ::de::Deserializer {
            reader,
            options: self.clone(),
        };
        serde::Deserialize::deserialize(&mut deserializer)
            .chain_err(|| "failed to deserialize reader")
    }
}
