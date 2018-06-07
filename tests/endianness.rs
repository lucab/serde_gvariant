extern crate serde_gvariant;

#[test]
fn test_network_endian() {
    let encoded: Vec<u8> = vec![0x2a, 0x00];
    {
        let le_cfg = serde_gvariant::config().network_endian(false);
        let le_decoded: u16 = 42;
        let le_ser: Vec<u8> = le_cfg.serialize(&le_decoded).expect("u16 LE ser");
        let le_de: u16 = le_cfg.deserialize_slice(&encoded[..]).expect("u16 LE de");
        assert_eq!(le_de, le_decoded);
        assert_eq!(le_ser, encoded);
    }
    {
        let be_cfg = serde_gvariant::config().network_endian(true);
        let be_decoded: u16 = 10752;
        let be_ser: Vec<u8> = be_cfg.serialize(&be_decoded).expect("u16 BE ser");
        let be_de: u16 = be_cfg.deserialize_slice(&encoded[..]).expect("u16 BE de");
        assert_eq!(be_de, be_decoded);
        assert_eq!(be_ser, encoded);
    }
}
