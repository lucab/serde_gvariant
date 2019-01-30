extern crate env_logger;
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate serde_gvariant;

use serde_gvariant::{errors, Variant};
use std::{fs, io};

/// An ostree commit object.
///
/// Signature: "(a{sv}aya(say)sstayay)"
/// Original definition at:
/// https://github.com/ostreedev/ostree/blob/v2018.5/src/libostree/ostree-core.h#L137-L150
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

// An ostree related object, as embedded into commit.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct OstreeCommitRelated {
    first: String,
    value: Vec<u8>,
}

fn main() -> errors::Result<()> {
    // Setup logging
    env_logger::Builder::new()
        .default_format_timestamp(false)
        //      .filter(Some("serde_gvariant"), log::LevelFilter::Trace)
        .init();

    // First parameter is target commit file (optional, default: fixtures sample)
    let sample = "tests/fixtures/ostree/basic-01.commit".to_string();
    let input = std::env::args().nth(1).unwrap_or(sample);

    // Open dirmeta file, and wrap it for buffered read
    let fp = fs::File::open(input)?;
    let bufrd = io::BufReader::new(fp);

    // This requires custom configuration, as fields are big-endian
    let de_cfg = serde_gvariant::Config::new().network_endian(true);

    // Deserialize from the reader and print it
    let dirmeta: OstreeCommit = de_cfg.deserialize_reader(bufrd)?;
    println!("Deserialized commit object:\n{:#?}", dirmeta);
    Ok(())
}
