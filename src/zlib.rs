use flate2::Compression;
#[allow(unused)]
use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use std::error;
use std::io::Write;

pub fn compress(before: &[u8]) -> Result<Vec<u8>, Box<dyn error::Error>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(before)?;
    let compressed = e.finish()?;
    Ok(compressed)
}
