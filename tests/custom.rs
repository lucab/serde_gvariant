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
    {
        let encoded: Vec<u8> = vec![0x01];
        let decoded: Option<bool> = Some(true);
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Option ser");
        let de: Option<bool> = serde_gvariant::from_slice(&encoded[..]).expect("Option de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        let encoded: Vec<u8> = vec![b'a', 0x00, 0x00];
        let decoded: Option<String> = Some("a".to_string());
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Option ser");
        let de: Option<String> = serde_gvariant::from_slice(&encoded[..]).expect("Option de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
}

#[test]
fn test_fixed_struct() {
    #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
    struct TestType {
        len: u8,
    };
    let encoded: Vec<u8> = vec![0x00];
    let decoded = TestType::default();
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("fixed struct ser");
    let de: TestType = serde_gvariant::from_slice(&encoded[..]).expect("fixed struct de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_variable_struct() {
    {
        #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
        struct TestType {
            len: u8,
            value: String,
        };
        let encoded: Vec<u8> = vec![0x00, 0x00];
        let decoded = TestType::default();
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("fixed struct ser");
        let de: TestType = serde_gvariant::from_slice(&encoded[..]).expect("fixed struct de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
        struct TestType {
            len: u8,
            value: String,
            meta: String,
        };
        let encoded: Vec<u8> = vec![0x00, 0x00, 0x00, 0x02];
        let decoded = TestType::default();
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("fixed struct ser");
        let de: TestType = serde_gvariant::from_slice(&encoded[..]).expect("fixed struct de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        #[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
        struct TestType {
            len: u16,
            value: String,
            metalen: u32,
            meta: String,
        };
        let encoded: Vec<u8> = vec![
            0x03, 0x00, b'f', b'o', b'o', 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, b'f', b'o',
            b'o', b'b', b'a', b'r', 0x00, 0x06,
        ];
        let decoded = TestType {
            len: 3,
            value: "foo".to_string(),
            metalen: 6,
            meta: "foobar".to_string(),
        };
        // TODO(lucab): fix ser alignment
        let _ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("fixed struct ser");
        let de: TestType = serde_gvariant::from_slice(&encoded[..]).expect("fixed struct de");
        //assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
}

#[test]
fn test_array() {
    let encoded: Vec<u8> = vec![b'a', 0x00, b'b', 0x00, 0x02, 0x04];
    let decoded: Vec<String> = vec!["a".into(), "b".into()];
    //let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Vector ser");
    let de: Vec<String> = serde_gvariant::from_slice(&encoded[..]).expect("Vector de");
    //assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_variant() {
    use serde_gvariant::Variant;

    {
        let encoded: Vec<u8> = vec![0x01, 0x00, b'b'];
        let decoded: Variant = Variant::Bool(true);
        //let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Variant ser");
        let de: Variant = serde_gvariant::from_slice(&encoded[..]).expect("Variant de");
        //assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        let encoded: Vec<u8> = vec![b'f', b'o', b'o', 0x00, 0x00, b's'];
        let decoded: Variant = Variant::String("foo".to_string());
        //let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Variant ser");
        let de: Variant = serde_gvariant::from_slice(&encoded[..]).expect("Variant de");
        //assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        let encoded: Vec<u8> = vec![0x01, 0x02, 0x00, b'a', b'y'];
        let decoded: Variant = Variant::Vec(vec![Variant::U8(1), Variant::U8(2)]);
        //let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Variant ser");
        let de: Variant = serde_gvariant::from_slice(&encoded[..]).expect("Variant de");
        //assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        let encoded: Vec<u8> = vec![b'a', 0x00, 0x02, 0x00, b'a', b's'];
        let decoded: Variant = Variant::Vec(vec![Variant::String("a".into())]);
        //let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Variant ser");
        let de: Variant = serde_gvariant::from_slice(&encoded[..]).expect("Variant de");
        //assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        let encoded: Vec<u8> = vec![b'a', 0x00, b'b', 0x00, 0x02, 0x04, 0x00, b'a', b's'];
        let decoded: Variant = Variant::Vec(vec![
            Variant::String("a".into()),
            Variant::String("b".into()),
        ]);
        //let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("Variant ser");
        let de: Variant = serde_gvariant::from_slice(&encoded[..]).expect("Variant de");
        //assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
}
