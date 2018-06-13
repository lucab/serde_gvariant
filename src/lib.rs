//! Serialization library for GVariant format.
//!
//! Through the [serde][serde] serialization framework, this crate provides
//! for reading and writing complex data structures recorded in
//! [GVariant][gvariant] binary format.
//!
//! [serde]: https://serde.rs/
//! [gvariant]: https://developer.gnome.org/glib/stable/glib-GVariant.html
//!
//! ### Basic example
//!
//! ```rust
//! extern crate serde_gvariant;
//!
//! fn main() {
//!     // The object that will be serialized.
//!     let target: u32 = 42;
//!
//!     let encoded: Vec<u8> = serde_gvariant::to_vec(&target).unwrap();
//!     let decoded: u32 = serde_gvariant::from_slice(&encoded[..]).unwrap();
//!     assert_eq!(target, decoded);
//! }
//! ```

extern crate byteorder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod config;
mod de;
pub mod errors;
mod ser;

pub use config::Config;

/// Get a default configuration object.
///
/// ### Default Configuration
///  * Nesting limit: none
///  * Endianness: little
pub fn config() -> config::Config {
    config::Config::new()
}

/// Serializes a serializable object into a `Vec` of bytes using the default configuration.
pub fn to_vec<T: ?Sized>(value: &T) -> errors::Result<Vec<u8>>
where
    T: serde::Serialize,
{
    config().serialize(value)
}

/// Deserializes an object directly from a `Read`er using the default configuration.
pub fn from_reader<R, T>(reader: R) -> errors::Result<T>
where
    R: std::io::Read,
    T: serde::de::DeserializeOwned,
{
    config().deserialize_reader(reader)
}

/// Deserializes a slice of bytes into an instance of `T` using the default configuration.
pub fn from_slice<'a, T>(bytes: &'a [u8]) -> errors::Result<T>
where
    T: serde::de::Deserialize<'a>,
{
    config().deserialize_slice(bytes)
}
