use errors::{self, ResultExt};
use serde;
use std::io;

/// A configuration object whose settings will be used while
/// serializing and deserializing.
#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) max_string_len: usize,
    pub(crate) network_endian: bool,
}

impl Config {
    /// Builds a new configuration object, with default settings.
    pub fn new() -> Config {
        Config {
            max_string_len: 8192,
            network_endian: false,
        }
    }

    /// Sets whether to use network (i.e. big) endianness.
    pub fn network_endian(self, ne: bool) -> Config {
        let mut cfg = self;
        cfg.network_endian = ne;
        cfg
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
    pub fn deserialize_slice<'a, T: serde::Deserialize<'a>>(
        &self,
        bytes: &'a [u8],
    ) -> errors::Result<T> {
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
