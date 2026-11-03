use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::io;
use std::io::Write;

pub fn compress(before: &[u8]) -> io::Result<Vec<u8>> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(before)?;
    let compressed = e.finish()?;
    Ok(compressed)
}
