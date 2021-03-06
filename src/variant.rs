use crate::errors;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;
use std::{cmp, hash};

/// GVariant array, homogeneous inner type.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(transparent)]
pub struct Array {
    /// Inner elements.
    #[serde(flatten)]
    inner: Vec<Variant>,
}

impl Array {
    /// Build an array.
    pub fn from_elements(elements: Vec<Variant>) -> errors::Result<Self> {
        let array = Self { inner: elements };
        Ok(array)
    }

    /// Transform self into a `Variant`.
    pub fn into_variant(self) -> Variant {
        Variant::Vec(self)
    }

    /// Return type signature.
    pub fn signature(&self) -> String {
        // TODO(lucab): store the actual expected type, fixing the "empty array"
        //  type confusion.
        if self.inner.is_empty() {
            "av".to_string()
        } else {
            format!("a{}", self.inner[0].signature())
        }
    }
}

/// GVariant dictionary, homogeneous inner key-value types.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(transparent)]
pub struct Dictionary {
    /// Inner map.
    #[serde(flatten)]
    inner: BTreeMap<Variant, Variant>,
}

impl Dictionary {
    /// Build a dictionary.
    pub fn from_map(map: BTreeMap<Variant, Variant>) -> errors::Result<Self> {
        let dict = Self { inner: map };
        Ok(dict)
    }

    /// Transform self into a `Variant`.
    pub fn into_variant(self) -> Variant {
        Variant::Dictionary(self)
    }

    /// Return type signature.
    pub fn signature(&self) -> String {
        // TODO(lucab): store the actual expected type, fixing the "empty dict"
        //  type confusion.
        if self.inner.is_empty() {
            "{vv}".to_string()
        } else {
            let (k, v) = self.inner.iter().last().unwrap();
            format!("{{{}{}}}", k.signature(), v.signature())
        }
    }
}

/// GVariant structure, variadic tuple.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Structure {
    /// Structure fields.
    fields: Vec<Variant>,
}

impl Structure {
    /// Return type signature.
    pub fn signature(&self) -> String {
        let mut inner = String::new();
        for field in &self.fields {
            inner += &field.signature();
        }
        format!("({})", inner)
    }
}

/// All the types supported by GVariant (basic or containers).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Variant {
    /// Boolean (signature: `b`).
    Bool(bool),
    /// Byte (signature: `y`).
    U8(u8),
    /// Unsigned 16-bits integer (signature: `q`).
    U16(u16),
    /// Unsigned 32-bits integer (signature: `u`).
    U32(u32),
    /// Unsigned 64-bits integer (signature: `t`).
    U64(u64),
    /// Signed 16-bits integer (signature: `n`).
    I16(i16),
    /// Signed 32-bits integer (signature: `i`).
    I32(i32),
    /// Signed 64-bits integer (signature: `x`).
    I64(i64),
    /// Double-precision floating point number (signature: `d`).
    F64(f64),
    /// String (signature: `s`).
    String(String),
    /// DBus object path (signature: `o`).
    ObjectPath(String),
    /// DBus signature string (signature: `g`).
    Signature(String),
    /// Generic variant type (signature: `v`).
    Variant(Box<Variant>),
    /// Optional ("Maybe") container (signature: `m`).
    Option(Option<Box<Variant>>),
    /// Homogeneous array (signature: `aX`).
    Vec(Array),
    /// Structure, variadic tuple (signature: `(XYZ)`).
    Structure(Structure),
    /// Dictionary map (signature: `{XY}`).
    Dictionary(Dictionary),
}

impl Variant {
    /// Return inner type signature.
    pub fn signature(&self) -> String {
        match *self {
            Variant::Bool(..) => "b".to_string(),
            Variant::U8(..) => "y".to_string(),
            Variant::U16(..) => "q".to_string(),
            Variant::U32(..) => "u".to_string(),
            Variant::U64(..) => "t".to_string(),
            Variant::I16(..) => "n".to_string(),
            Variant::I32(..) => "i".to_string(),
            Variant::I64(..) => "x".to_string(),
            Variant::F64(..) => "d".to_string(),
            Variant::String(..) => "s".to_string(),
            Variant::ObjectPath(..) => "o".to_string(),
            Variant::Signature(..) => "g".to_string(),
            Variant::Variant(..) => "v".to_string(),
            Variant::Option(..) => "m".to_string(),
            Variant::Vec(ref v) => v.signature(),
            Variant::Structure(ref v) => v.signature(),
            Variant::Dictionary(ref v) => v.signature(),
        }
    }

    pub(crate) fn discriminant(&self) -> u64 {
        match *self {
            Variant::Bool(..) => 0,
            Variant::U8(..) => 1,
            Variant::U16(..) => 2,
            Variant::U32(..) => 3,
            Variant::U64(..) => 4,
            Variant::I16(..) => 5,
            Variant::I32(..) => 6,
            Variant::I64(..) => 7,
            Variant::F64(..) => 8,
            Variant::String(..) => 9,
            Variant::ObjectPath(..) => 10,
            Variant::Signature(..) => 11,
            Variant::Variant(..) => 12,
            Variant::Option(..) => 13,
            Variant::Vec(..) => 14,
            Variant::Structure(..) => 15,
            Variant::Dictionary(..) => 16,
        }
    }
}

impl hash::Hash for Variant {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: hash::Hasher,
    {
        self.discriminant().hash(hasher);
        match *self {
            Variant::Bool(v) => v.hash(hasher),
            Variant::U8(v) => v.hash(hasher),
            Variant::U16(v) => v.hash(hasher),
            Variant::U32(v) => v.hash(hasher),
            Variant::U64(v) => v.hash(hasher),
            Variant::I16(v) => v.hash(hasher),
            Variant::I32(v) => v.hash(hasher),
            Variant::I64(v) => v.hash(hasher),
            Variant::F64(v) => OrderedFloat(v).hash(hasher),
            Variant::String(ref v) => v.hash(hasher),
            Variant::ObjectPath(ref v) => v.hash(hasher),
            Variant::Signature(ref v) => v.hash(hasher),
            Variant::Variant(ref v) => v.hash(hasher),
            Variant::Option(ref v) => v.hash(hasher),
            Variant::Vec(ref v) => v.hash(hasher),
            Variant::Structure(ref v) => v.hash(hasher),
            Variant::Dictionary(ref v) => v.hash(hasher),
        }
    }
}

impl PartialEq for Variant {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (&Variant::Bool(v0), &Variant::Bool(v1)) if v0 == v1 => true,
            (&Variant::U8(v0), &Variant::U8(v1)) if v0 == v1 => true,
            (&Variant::U16(v0), &Variant::U16(v1)) if v0 == v1 => true,
            (&Variant::U32(v0), &Variant::U32(v1)) if v0 == v1 => true,
            (&Variant::U64(v0), &Variant::U64(v1)) if v0 == v1 => true,
            (&Variant::I16(v0), &Variant::I16(v1)) if v0 == v1 => true,
            (&Variant::I32(v0), &Variant::I32(v1)) if v0 == v1 => true,
            (&Variant::I64(v0), &Variant::I64(v1)) if v0 == v1 => true,
            (&Variant::F64(v0), &Variant::F64(v1)) if v0 == v1 => true,
            (&Variant::String(ref v0), &Variant::String(ref v1)) if v0 == v1 => true,
            (&Variant::ObjectPath(ref v0), &Variant::ObjectPath(ref v1)) if v0 == v1 => true,
            (&Variant::Signature(ref v0), &Variant::Signature(ref v1)) if v0 == v1 => true,
            (&Variant::Variant(ref v0), &Variant::Variant(ref v1)) if v0 == v1 => true,
            (&Variant::Option(ref v0), &Variant::Option(ref v1)) if v0 == v1 => true,
            (&Variant::Vec(ref v0), &Variant::Vec(ref v1)) if v0 == v1 => true,
            (&Variant::Structure(ref v0), &Variant::Structure(ref v1)) if v0 == v1 => true,
            (&Variant::Dictionary(ref v0), &Variant::Dictionary(ref v1)) if v0 == v1 => true,
            _ => false,
        }
    }
}

impl Ord for Variant {
    fn cmp(&self, rhs: &Self) -> cmp::Ordering {
        match (self, rhs) {
            (&Variant::Bool(v0), &Variant::Bool(ref v1)) => v0.cmp(v1),
            (&Variant::U8(v0), &Variant::U8(ref v1)) => v0.cmp(v1),
            (&Variant::U16(v0), &Variant::U16(ref v1)) => v0.cmp(v1),
            (&Variant::U32(v0), &Variant::U32(ref v1)) => v0.cmp(v1),
            (&Variant::U64(v0), &Variant::U64(ref v1)) => v0.cmp(v1),
            (&Variant::I16(v0), &Variant::I16(ref v1)) => v0.cmp(v1),
            (&Variant::I32(v0), &Variant::I32(ref v1)) => v0.cmp(v1),
            (&Variant::I64(v0), &Variant::I64(ref v1)) => v0.cmp(v1),
            (&Variant::F64(v0), &Variant::F64(v1)) => OrderedFloat(v0).cmp(&OrderedFloat(v1)),
            (&Variant::String(ref v0), &Variant::String(ref v1)) => v0.cmp(v1),
            (&Variant::ObjectPath(ref v0), &Variant::ObjectPath(ref v1)) => v0.cmp(v1),
            (&Variant::Signature(ref v0), &Variant::Signature(ref v1)) => v0.cmp(v1),
            (&Variant::Variant(ref v0), &Variant::Variant(ref v1)) => v0.cmp(v1),
            (&Variant::Option(ref v0), &Variant::Option(ref v1)) => v0.cmp(v1),
            (&Variant::Vec(ref v0), &Variant::Vec(ref v1)) => v0.cmp(v1),
            (&Variant::Structure(ref v0), &Variant::Structure(ref v1)) => v0.cmp(v1),
            (&Variant::Dictionary(ref v0), &Variant::Dictionary(ref v1)) => v0.cmp(v1),
            (ref v0, ref v1) => v0.discriminant().cmp(&v1.discriminant()),
        }
    }
}

impl Eq for Variant {}
impl PartialOrd for Variant {
    fn partial_cmp(&self, rhs: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
