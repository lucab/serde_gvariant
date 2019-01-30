extern crate env_logger;
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_gvariant;

use serde_gvariant::errors;
use std::{fs, io};

/// An ostree dirmeta object.
///
/// Signature: "(uuua(ayay))"
/// Original definition at:
/// https://github.com/ostreedev/ostree/blob/v2018.5/src/libostree/ostree-core.h#L112-L126
#[derive(Debug, Deserialize)]
struct OstreeDirmeta {
    uid: u32,                 // u - uid (big-endian)
    gid: u32,                 // u - gid (big-endian)
    mode: u32,                // u - mode (big-endian)
    xattrs: Vec<OstreeXattr>, // a(ayay) - xattrs
}

// An ostree xattr object, as embedded into dirmeta.
#[derive(Debug, Deserialize)]
struct OstreeXattr {
    key: Vec<u8>,
    value: Vec<u8>,
}

fn main() -> errors::Result<()> {
    // Setup logging
    env_logger::Builder::new()
        .default_format_timestamp(false)
        // .filter(Some("serde_gvariant"), log::LevelFilter::Trace)
        .init();

    // First parameter is target dirmeta file (optional, default: fixtures sample)
    let sample = "tests/fixtures/ostree/basic-01.dirmeta".to_string();
    let input = std::env::args().nth(1).unwrap_or(sample);

    // Open dirmeta file, and wrap it for buffered read
    let fp = fs::File::open(input)?;
    let bufrd = io::BufReader::new(fp);

    // This requires custom configuration, as fields are big-endian
    let de_cfg = serde_gvariant::Config::new().network_endian(true);

    // Deserialize from the reader and print it
    let dirmeta: OstreeDirmeta = de_cfg.deserialize_reader(bufrd)?;
    println!("Deserialized dirmeta object:\n{:#?}", dirmeta);
    Ok(())
}
