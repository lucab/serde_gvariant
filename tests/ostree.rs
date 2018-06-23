extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_gvariant;

use serde_gvariant::Variant;
use std::io::{Read, Seek};
use std::{fs, io};

/* DirMeta */

/// An ostree dirmeta object.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct OstreeDirmeta {
    uid: u32,
    gid: u32,
    mode: u32,
    xattrs: Vec<OstreeXattr>,
}

/// An ostree xattr object, as embedded into dirmeta.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct OstreeXattr {
    key: Vec<u8>,
    value: Vec<u8>,
}

/* DirTree */

/// An ostree dirtree object.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct OstreeDirtree {
    files: Vec<OstreeDirtreeFile>,
    dirs: Vec<OstreeDirtreeDir>,
}

/// An ostree file object, as embedded into dirtree.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct OstreeDirtreeFile {
    filename: String,
    checksum: Vec<u8>,
}

/// An ostree directory object, as embedded into dirtree.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct OstreeDirtreeDir {
    dirname: String,
    tree_checksum: Vec<u8>,
    meta_checksum: Vec<u8>,
}

/* Commit */

/// An ostree commit object.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct OstreeCommit {
    metadata: Vec<(String, Variant)>,
    parent_checksum: Vec<u8>,
    related_objs: Vec<OstreeCommitRelated>,
    subject: String,
    body: String,
    timestamp: u64,
    root_contents: Vec<u8>,
    root_metadata: Vec<u8>,
}

/// An ostree related object, as embedded into commit.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct OstreeCommitRelated {
    first: String,
    value: Vec<u8>,
}

/* Tests */

#[test]
fn test_basic_01_dirmeta() {
    let sample = "tests/fixtures/ostree/basic-01.dirmeta";
    let fp = fs::File::open(sample).unwrap();
    let mut bufrd = io::BufReader::new(fp);
    let mut content = Vec::new();
    bufrd.read_to_end(&mut content).unwrap();
    bufrd.seek(io::SeekFrom::Start(0)).unwrap();

    let cfg = serde_gvariant::Config::new().network_endian(true);
    let _de: OstreeDirmeta = cfg.deserialize_reader(bufrd).unwrap();
    //let ser = cfg.serialize(&_de).unwrap();
    //assert_eq!(content, ser);

    let exp = OstreeDirmeta {
        uid: 1000,
        gid: 1000,
        mode: 0o40755,
        xattrs: vec![],
    };
    assert_eq!(exp, _de);
}

#[test]
fn test_basic_01_dirtree() {
    let sample = "tests/fixtures/ostree/basic-01.dirtree";
    let fp = fs::File::open(sample).unwrap();
    let mut bufrd = io::BufReader::new(fp);
    let mut content = Vec::new();
    bufrd.read_to_end(&mut content).unwrap();
    bufrd.seek(io::SeekFrom::Start(0)).unwrap();

    let cfg = serde_gvariant::Config::new().network_endian(true);
    let _de: OstreeDirtree = cfg.deserialize_reader(bufrd).unwrap();
    //let ser = cfg.serialize(&_de).unwrap();
    //assert_eq!(content, ser);

    let exp = OstreeDirtree {
        files: vec![OstreeDirtreeFile {
            filename: "foo.txt".to_string(),
            checksum: vec![
                194, 133, 83, 229, 238, 209, 11, 240, 114, 6, 157, 68, 104, 255, 239, 80, 35, 111,
                214, 232, 48, 112, 164, 46, 48, 178, 153, 120, 83, 140, 55, 120,
            ],
        }],
        dirs: vec![],
    };
    assert_eq!(exp, _de);
}

#[test]
fn test_basic_01_commit() {
    let sample = "tests/fixtures/ostree/basic-01.commit";
    let fp = fs::File::open(sample).unwrap();
    let mut bufrd = io::BufReader::new(fp);
    let mut content = Vec::new();
    bufrd.read_to_end(&mut content).unwrap();
    bufrd.seek(io::SeekFrom::Start(0)).unwrap();

    let cfg = serde_gvariant::Config::new().network_endian(true);
    let _de: OstreeCommit = cfg.deserialize_reader(bufrd).unwrap();
    //let ser = cfg.serialize(&_de).unwrap();
    //assert_eq!(content, ser);

    let exp = OstreeCommit {
        metadata: vec![(
            "ostree.ref-binding".to_string(),
            Variant::Vec(vec![Variant::String("master".to_string())]),
        )],
        parent_checksum: vec![],
        related_objs: vec![],
        subject: "".to_string(),
        body: "".to_string(),
        timestamp: 1527716235,
        root_contents: vec![
            237, 13, 152, 4, 14, 133, 146, 115, 31, 37, 255, 200, 120, 238, 237, 220, 119, 238, 81,
            217, 240, 19, 225, 144, 201, 186, 29, 41, 182, 52, 14, 233,
        ],
        root_metadata: vec![
            121, 79, 155, 139, 49, 77, 77, 87, 25, 63, 202, 55, 218, 185, 20, 155, 7, 91, 89, 141,
            173, 179, 157, 51, 247, 125, 106, 129, 95, 202, 88, 76,
        ],
    };
    assert_eq!(exp, _de);
}
