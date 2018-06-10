extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_gvariant;

#[test]
fn test_bytes_buf() {
    let data = &[0x01, 0x02, 0x00, 0x03, 0x04];
    let buf = serde_bytes::ByteBuf::from(data.clone().to_vec());
    let slice = serde_bytes::Bytes::new(data);
    let de: serde_bytes::ByteBuf = serde_gvariant::from_slice(&slice).expect("bytes de");
    let ser: Vec<u8> = serde_gvariant::to_vec(&buf).expect("bytes ser");
    assert_eq!(ser, data.to_vec());
    assert_eq!(de, buf);
}

#[test]
fn test_unit() {
    let encoded: Vec<u8> = vec![];
    let decoded = ();
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("unit ser");
    let de: () = serde_gvariant::from_slice(&encoded[..]).expect("unit de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_unit_struct() {
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct TestType;
    let encoded: Vec<u8> = vec![];
    let decoded = TestType {};
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("unit struct ser");
    let de: TestType = serde_gvariant::from_slice(&encoded[..]).expect("unit struct de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_option() {
    {
        let encoded: Vec<u8> = vec![];
        let decoded: Option<u8> = None;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Option ser");
        let de: Option<u8> = serde_gvariant::from_slice(&encoded[..]).expect("Option de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        let encoded: Vec<u8> = vec![];
        let decoded: Option<String> = None;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Option ser");
        let de: Option<String> = serde_gvariant::from_slice(&encoded[..]).expect("Option de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
}
