extern crate env_logger;
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_gvariant;

use serde_gvariant::errors;
use std::{fs, io};

/// An ostree dirtree object.
///
/// Signature: "(a(say)a(sayay))"
/// Original definition at:
/// https://github.com/ostreedev/ostree/blob/v2018.5/src/libostree/ostree-core.h#L128-L135
#[derive(Debug, Deserialize)]
struct OstreeDirtree {
    files: Vec<OstreeDirtreeFile>,
    dirs: Vec<OstreeDirtreeDir>,
}

// An ostree file object, as embedded into dirtree.
#[derive(Debug, Deserialize)]
struct OstreeDirtreeFile {
    filename: String,
    checksum: Vec<u8>,
}

// An ostree directory object, as embedded into dirtree.
#[derive(Debug, Deserialize)]
struct OstreeDirtreeDir {
    dirname: String,
    tree_checksum: Vec<u8>,
    meta_checksum: Vec<u8>,
}

fn main() -> errors::Result<()> {
    // Setup logging
    env_logger::Builder::new()
        .default_format_timestamp(false)
        // .filter(Some("serde_gvariant"), log::LevelFilter::Trace)
        .init();

    // First parameter is target dirtree file (optional, default: fixtures sample)
    let sample = "tests/fixtures/ostree/basic-01.dirtree".to_string();
    let input = std::env::args().nth(1).unwrap_or(sample);

    // Open dirtree file, and wrap it for buffered read
    let fp = fs::File::open(input)?;
    let bufrd = io::BufReader::new(fp);

    // This requires custom configuration, as fields are big-endian
    let de_cfg = serde_gvariant::Config::new().network_endian(true);

    // Deserialize from the reader and print it
    let dirtree: OstreeDirtree = de_cfg.deserialize_reader(bufrd)?;
    println!("Deserialized dirtree object:\n{:#?}", dirtree);
    Ok(())
}
