use byteorder::{LittleEndian, ReadBytesExt};
use de::top::TopDeserializer;
use errors::{self, ResultExt};
use serde::de::Error;
use std::io;

pub(crate) fn read_len<RS: io::Read + io::Seek>(
    top: &mut TopDeserializer<RS>,
    start: u64,
    end: u64,
    len: u64,
) -> errors::Result<(u64, u64)> {
    let size = compute_size(len);
    let len_pos = end
        .checked_sub(size)
        .ok_or_else(|| errors::Error::custom("struct: length position underflow"))?;
    // Seek to end, in order to read length.
    top.reader.seek(io::SeekFrom::Start(len_pos))?;
    // Read length, which is variable in size.
    let val = match size {
        1 => {
            let len8 = top
                .reader
                .read_u8()
                .chain_err(|| "struct: reading array length (u8)")?;
            u64::from(len8)
        }
        2 => {
            let len16 = top
                .reader
                .read_u16::<LittleEndian>()
                .chain_err(|| "struct: reading array length (u16)")?;
            u64::from(len16)
        }
        4 => {
            let len32 = top
                .reader
                .read_u32::<LittleEndian>()
                .chain_err(|| "struct: reading array length (u32)")?;
            u64::from(len32)
        }
        8 => top
            .reader
            .read_u64::<LittleEndian>()
            .chain_err(|| "struct: reading array length (u64)")?,
        _ => return Err(errors::Error::custom("struct: unsupported array size")),
    };
    // Reposition to the beginning.
    top.reader.seek(io::SeekFrom::Start(start))?;
    Ok((val, size))
}

pub(crate) fn compute_size(len: u64) -> u64 {
    if len <= u64::from(::std::u8::MAX) {
        1
    } else if len <= u64::from(::std::u16::MAX) {
        2
    } else if len <= u64::from(::std::u32::MAX) {
        4
    } else {
        8
    }
}
