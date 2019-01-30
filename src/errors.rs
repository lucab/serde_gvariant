//! Error handling.

use serde::{de, ser};
use std::{fmt, io, num};

error_chain! {
    foreign_links {
        Io(io::Error);
        ParseInt(num::ParseIntError);
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::from(format!("{}", msg))
    }
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        Self::from(format!("{}", msg))
    }
}
