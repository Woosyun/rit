pub mod blob;
pub use blob::*;

pub mod tree;
pub use tree::*;

pub mod commit;
pub use commit::*;

use std::io;

pub type ObjectId = [u8; 20];
pub struct Object {
    pub kind: ObjectKind,
    pub size: u32,
    pub data: Vec<u8>,
}
impl Object {
    pub fn new(kind: ObjectKind, size: usize, data: Vec<u8>) -> Self {
        Self {
            kind,
            size: size as u32,
            data,
        }
    }
    pub fn parse(obj_raw: ObjectRaw) -> io::Result<Self> {
        let partition_point = obj_raw.data.partition_point(|&b| b == b'\0');
        let (header_raw, data) =
            obj_raw
                .data
                .split_at_checked(partition_point)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Cannot parse to object: empty body",
                    )
                })?;
        let partition_point = header_raw.partition_point(|&b| b == b' ');
        let (kind_raw, size_raw) =
            header_raw
                .split_at_checked(partition_point)
                .ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "cannot parse header of the object",
                    )
                })?;
        let kind = match kind_raw {
            b"blob" => ObjectKind::Blob,
            b"tree" => ObjectKind::Tree,
            b"commit" => ObjectKind::Commit,

            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "cannot parse object",
                ));
            }
        };

        let size = size_raw
            .get(..4)
            .and_then(|v| v.try_into().ok())
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "bytes representing size shorter than 4",
                )
            })?;
        let size = u32::from_be_bytes(size);

        let obj = Self {
            kind,
            size,
            data: data.to_vec(),
        };
        Ok(obj)
    }
}
pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
}
pub struct ObjectRaw {
    data: Vec<u8>,
}
impl ObjectRaw {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl io::Write for ObjectRaw {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.data.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.data.flush()
    }
}
impl io::Read for ObjectRaw {
    fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
        self.data.as_slice().read(out)
    }
}
