#[macro_use]
extern crate proptest;
extern crate serde_gvariant;

use proptest::prelude::any;

proptest! {
    #[test]
    fn testprop_roundtrip_bool(input in any::<bool>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&input)?;
        let decoded: bool = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(input, decoded);
    }

    #[test]
    fn testprop_roundtrip_i8(num in any::<i8>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: i8 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_u8(num in any::<u8>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: u8 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_i16(num in any::<i16>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: i16 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_u16(num in any::<u16>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: u16 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_i32(num in any::<i32>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: i32 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_u32(num in any::<u32>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: u32 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_i64(num in any::<i64>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: i64 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_u64(num in any::<u64>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: u64 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_f64(num in any::<f64>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&num)?;
        let decoded: f64 = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(num, decoded);
    }

    #[test]
    fn testprop_roundtrip_string(ref input in any::<String>()){
        let encoded: Vec<u8> = serde_gvariant::to_vec(&input)?;
        let decoded: String = serde_gvariant::from_slice(&encoded[..])?;
        prop_assert_eq!(input, &decoded);
    }

    #[test]
    fn testprop_nonpanic_variant(ref bytes in any::<Vec<u8>>()){
        use serde_gvariant::errors::Result;
        use serde_gvariant::Variant;
        let _decoded: Result<Variant> = serde_gvariant::from_slice(&bytes[..]);
    }
}
