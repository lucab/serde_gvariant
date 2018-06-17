use ordered_float::OrderedFloat;
use std::{cmp, hash};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Variant {
    Bool(bool),

    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    I16(i16),
    I32(i32),
    I64(i64),

    F64(f64),

    String(String),

    Option(Option<Box<Variant>>),
    Vec(Vec<Variant>),
}

impl Variant {
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
            Variant::Option(..) => 10,
            Variant::Vec(..) => 11,
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
            Variant::Option(ref v) => v.hash(hasher),
            Variant::Vec(ref v) => v.hash(hasher),
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
            (&Variant::Option(ref v0), &Variant::Option(ref v1)) if v0 == v1 => true,
            (&Variant::Vec(ref v0), &Variant::Vec(ref v1)) if v0 == v1 => true,
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
            (&Variant::Option(ref v0), &Variant::Option(ref v1)) => v0.cmp(v1),
            (&Variant::Vec(ref v0), &Variant::Vec(ref v1)) => v0.cmp(v1),
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
