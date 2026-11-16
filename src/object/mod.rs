pub mod blob;
pub use blob::*;

pub mod tree;
pub use tree::*;

pub mod commit;
pub use commit::*;

use std::io;
use std::io::Write;

pub type ObjectId = [u8; 20];
pub fn encode_to_hex(oid: &ObjectId) -> String {
    oid.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}
pub fn decode_to_oid(hex: &str) -> io::Result<ObjectId> {
    if hex.len() != 40 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Should be 40 characters to be ObjectId",
        ));
    }
    let v = hex
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|vals| Ok(char_to_word(vals[0] as u8)? << 4 | char_to_word(vals[1] as u8)?))
        .collect::<io::Result<Vec<u8>>>()?;
    let mut re = [0; 20];
    re.copy_from_slice(&v[..]);
    Ok(re)
}
fn char_to_word(c: u8) -> io::Result<u8> {
    let i = match c {
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        b'0'..=b'9' => c - b'0',
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid character",
            ));
        }
    };
    Ok(i)
}

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
}
pub enum ObjectKind {
    Blob,
    Tree,
    Commit,
}
impl ObjectKind {
    fn to_str(&self) -> &str {
        match self {
            Self::Blob => "blob",
            Self::Tree => "tree",
            Self::Commit => "commit",
        }
    }
}

pub struct ObjectRaw {
    pub data: Vec<u8>,
}
impl ObjectRaw {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn parse(&self) -> io::Result<Object> {
        let partition_point = self.data.partition_point(|&b| b == b'\0');
        let (header_raw, data) = self.data.split_at_checked(partition_point).ok_or_else(|| {
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

        let obj = Object::new(kind, size as usize, data.to_vec());
        Ok(obj)
    }
}
impl From<&Object> for ObjectRaw {
    fn from(obj: &Object) -> ObjectRaw {
        let mut out = Vec::with_capacity(256);
        write!(&mut out, "{} {}\0", obj.kind.to_str(), &obj.size,).unwrap();
        out.copy_from_slice(&obj.data);

        ObjectRaw::new(out)
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
