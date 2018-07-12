extern crate env_logger;
extern crate log;
extern crate memmap;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;
extern crate serde_gvariant;

use serde_gvariant::errors;
use std::fs;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct InMemoryData<'a> {
    label: String,
    bytes: &'a[u8],
}

fn main() -> errors::Result<()> {
    // Setup logging
    env_logger::Builder::new()
        .default_format_timestamp(false)
      .filter(Some("serde_gvariant"), log::LevelFilter::Trace)
        .init();

    // First parameter is target commit file (optional, default: fixtures sample)
    let sample = "tests/fixtures/misc/ex-mmap.01".to_string();
    let input = std::env::args().nth(1).unwrap_or(sample);

    // Open dirmeta file, and wrap it for buffered read
    let fp = fs::File::open(input)?;

    let mmap = unsafe { memmap::MmapOptions::new().map(&fp)? };

    // This requires custom configuration, as fields are big-endian
    let de_cfg = serde_gvariant::Config::new().network_endian(true);

    // Deserialize from mmap and print it
    //let data: InMemoryData = de_cfg.deserialize_slice(&mmap)?;
    let data: serde_bytes::Bytes = de_cfg.deserialize_slice(&mmap)?;
    println!("Deserialized data:\n{:#?}", data);
    Ok(())
}
