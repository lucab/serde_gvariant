#[macro_use]
extern crate serde_derive;
extern crate serde_gvariant;

#[test]
fn test_string_example() {
    let encoded: Vec<u8> = vec![
        b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'\0',
    ];
    let decoded: String = "hello world".to_string();
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("String ser");
    let de: String = serde_gvariant::from_slice(&encoded[..]).expect("String de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_maybe_string_example() {
    let encoded: Vec<u8> = vec![
        b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'\0', b'\0',
    ];
    let decoded: Option<String> = Some("hello world".to_string());
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Option ser");
    let de: Option<String> = serde_gvariant::from_slice(&encoded[..]).expect("Option de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_array_booleans_example() {
    let encoded: Vec<u8> = vec![0x01, 0x00, 0x00, 0x01, 0x01];
    let decoded: Vec<bool> = vec![true, false, false, true, true];
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("ab ser");
    let de: Vec<bool> = serde_gvariant::from_slice(&encoded[..]).expect("ab de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_struct() {
    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct TestType {
        first: String,
        second: i32,
    };
    let encoded: Vec<u8> = vec![b'f', b'o', b'o', 0x00, 0xff, 0xff, 0xff, 0xff, 0x04];
    let decoded = TestType {
        first: "foo".to_string(),
        second: -1,
    };
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("fixed struct ser");
    let de: TestType = serde_gvariant::from_slice(&encoded[..]).expect("fixed struct de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_simple_struct() {
    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct TestType {
        first: u8,
        second: u8,
    };
    let encoded: Vec<u8> = vec![0x70, 0x80];
    let decoded = TestType {
        first: 0x70,
        second: 0x80,
    };
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("fixed struct ser");
    let de: TestType = serde_gvariant::from_slice(&encoded[..]).expect("fixed struct de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_array_bytes_example() {
    let encoded: Vec<u8> = vec![0x04, 0x05, 0x06, 0x07];
    let decoded: Vec<u8> = vec![0x04, 0x05, 0x06, 0x07];
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("ab ser");
    let de: Vec<u8> = serde_gvariant::from_slice(&encoded[..]).expect("ab de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_array_integers_example() {
    let encoded: Vec<u8> = vec![0x04, 0x00, 0x00, 0x00, 0x02, 0x01, 0x00, 0x00];
    let decoded: Vec<u32> = vec![4, 258];
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("ab ser");
    let de: Vec<u32> = serde_gvariant::from_slice(&encoded[..]).expect("ab de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}
