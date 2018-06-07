extern crate serde_bytes;
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
