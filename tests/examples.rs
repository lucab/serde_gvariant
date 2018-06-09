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

/*
#[test]
fn test_array_booleans_example() {
    let encoded: Vec<u8> = vec![0x01, 0x00, 0x00, 0x01, 0x01];
    let decoded: Vec<bool> = vec![true, false, false, true, true];
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("ab ser");
    let de: Vec<bool> = serde_gvariant::from_slice(&encoded[..]).expect("ab de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}
*/
