// Test cases lifted from gvariant.js
// https://github.com/larskarlitski/gvariant.js/blob/8f140d1280664330d7916546247137a627950897/test/gvariant.js

extern crate serde_gvariant;

#[test]
fn test_js_bool() {
    // 'should map gvariant booleans to javascript booleans'
    {
        // test('b', [ 0x0 ], false);
        let encoded: Vec<u8> = vec![0x0];
        let decoded: bool = false;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("bool ser");
        let de: bool = serde_gvariant::from_slice(&encoded[..]).expect("bool de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        // test('b', [ 0x1 ], true);
        let encoded: Vec<u8> = vec![0x1];
        let decoded: bool = true;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("bool ser");
        let de: bool = serde_gvariant::from_slice(&encoded[..]).expect("bool de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
}

#[test]
fn test_js_integer() {
    // 'should map gvariant integer types to javascript numbers'
    {
        // test('u', [ 0x2a, 0x0, 0x0, 0x0 ], 42);
        let encoded: Vec<u8> = vec![0x2a, 0x0, 0x0, 0x0];
        let decoded: u32 = 42;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("u32 ser");
        let de: u32 = serde_gvariant::from_slice(&encoded[..]).expect("u32 de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        // test('y', [ 0x2a ], 42);
        let encoded: Vec<u8> = vec![0x2a];
        let decoded: u8 = 42;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("u8 ser");
        let de: u8 = serde_gvariant::from_slice(&encoded[..]).expect("u8 de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        // test('i', [ 0xd6, 0xff, 0xff, 0xff ], -42);
        let encoded: Vec<u8> = vec![0xd6, 0xff, 0xff, 0xff];
        let decoded: i32 = -42;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("i32 ser");
        let de: i32 = serde_gvariant::from_slice(&encoded[..]).expect("i32 de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        // test('n', [ 0xd6, 0xff ], -42);
        let encoded: Vec<u8> = vec![0xd6, 0xff];
        let decoded: i16 = -42;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("i16 ser");
        let de: i16 = serde_gvariant::from_slice(&encoded[..]).expect("i16 de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        // test('q', [ 0x2a, 0x0 ], 42);
        let encoded: Vec<u8> = vec![0x2a, 0x00];
        let decoded: u16 = 42;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("u16 ser");
        let de: u16 = serde_gvariant::from_slice(&encoded[..]).expect("u16 de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        // test('x', [ 0xd6, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff ], -42);
        let encoded: Vec<u8> = vec![0xd6, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
        let decoded: i64 = -42;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("i64 ser");
        let de: i64 = serde_gvariant::from_slice(&encoded[..]).expect("i64 de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        // test('t', [ 0x2a, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0 ], 42);
        let encoded: Vec<u8> = vec![0x2a, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        let decoded: u64 = 42;
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("u64 ser");
        let de: u64 = serde_gvariant::from_slice(&encoded[..]).expect("u64 de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
}

#[test]
fn test_js_large_integer() {
    // 'should be able to read large integers'
    {
        let pos_cases = &[
            (vec![0x60, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], 96u64),
            (vec![0x0, 0xc, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0], 3072),
            (vec![0x0, 0x80, 0x1, 0x0, 0x0, 0x0, 0x0, 0x0], 98304),
            (vec![0x0, 0x0, 0x30, 0x0, 0x0, 0x0, 0x0, 0x0], 3145728),
            (vec![0x0, 0x0, 0x0, 0x6, 0x0, 0x0, 0x0, 0x0], 100663296),
            (vec![0x0, 0x0, 0x0, 0xc0, 0x0, 0x0, 0x0, 0x0], 3221225472),
            (vec![0x0, 0x0, 0x0, 0x0, 0x18, 0x0, 0x0, 0x0], 103079215104),
            (vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x3, 0x0, 0x0], 3298534883328),
            (
                vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x60, 0x0, 0x0],
                105553116266496,
            ),
            (
                vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xc, 0x0],
                3377699720527872,
            ),
        ];

        for t in pos_cases {
            let de_u64: u64 = serde_gvariant::from_slice(&t.0[..]).expect("u64 de");
            let ser_u64: Vec<u8> = serde_gvariant::to_vec(&t.1).expect("u64 ser");
            assert_eq!(ser_u64, t.0);
            assert_eq!(de_u64, t.1);

            let de_i64: i64 = serde_gvariant::from_slice(&t.0[..]).expect("i64 de");
            let ser_i64: Vec<u8> = serde_gvariant::to_vec(&t.1).expect("i64 ser");
            assert_eq!(ser_i64, t.0);
            assert_eq!(de_i64, t.1 as i64);
        }
    }
    {
        let neg_cases = &[
            (vec![0xa0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], -96i64),
            (vec![0x0, 0xf4, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff], -3072),
            (vec![0x0, 0x80, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff], -98304),
            (vec![0x0, 0x0, 0xd0, 0xff, 0xff, 0xff, 0xff, 0xff], -3145728),
            (
                vec![0x0, 0x0, 0x0, 0xfa, 0xff, 0xff, 0xff, 0xff],
                -100663296,
            ),
            (
                vec![0x0, 0x0, 0x0, 0x40, 0xff, 0xff, 0xff, 0xff],
                -3221225472,
            ),
            (
                vec![0x0, 0x0, 0x0, 0x0, 0xe8, 0xff, 0xff, 0xff],
                -103079215104,
            ),
            (
                vec![0x0, 0x0, 0x0, 0x0, 0x0, 0xfd, 0xff, 0xff],
                -3298534883328,
            ),
            (
                vec![0x0, 0x0, 0x0, 0x0, 0x0, 0xa0, 0xff, 0xff],
                -105553116266496,
            ),
            (
                vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0xf4, 0xff],
                -3377699720527872,
            ),
        ];

        for t in neg_cases {
            let de_i64: i64 = serde_gvariant::from_slice(&t.0[..]).expect("i64 de");
            let ser_i64: Vec<u8> = serde_gvariant::to_vec(&t.1).expect("i64 ser");
            assert_eq!(ser_i64, t.0);
            assert_eq!(de_i64, t.1 as i64);
        }
    }
}

#[test]
fn test_js_float() {
    // 'should map gvariant doubles to javascript numbers'
    // test('d', [ 0x0, 0x0, 0x0, 0x0, 0x0, 0x40, 0x45, 0x40 ], 42.5);
    let encoded: Vec<u8> = vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x40, 0x45, 0x40];
    let decoded: f64 = 42.5;
    let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("f64 ser");
    let de: f64 = serde_gvariant::from_slice(&encoded[..]).expect("f64 de");
    assert_eq!(ser, encoded);
    assert_eq!(de, decoded);
}

#[test]
fn test_js_string() {
    // 'should map gvariant strings to javascript strings'
    {
        // test('s', [ 0x0 ], '');
        let encoded: Vec<u8> = vec![0x0];
        let decoded: String = "".to_string();
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("string ser");
        let de: String = serde_gvariant::from_slice(&encoded[..]).expect("string de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
    {
        // test('s', [ 0x61, 0x62, 0x63, 0x0 ], 'abc');
        let encoded: Vec<u8> = vec![0x61, 0x62, 0x63, 0x0];
        let decoded: String = "abc".to_string();
        let ser: Vec<u8> = serde_gvariant::to_vec(&decoded).expect("string ser");
        let de: String = serde_gvariant::from_slice(&encoded[..]).expect("string de");
        assert_eq!(ser, encoded);
        assert_eq!(de, decoded);
    }
}
