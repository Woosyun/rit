pub mod blob;
pub use blob::*;

pub mod tree;
pub use tree::*;

pub mod commit;
pub use commit::*;

use std::io;
use std::io::Write;
use std::ops::Deref;

/// ObjectId is equal as \[u8; 20],
/// pointing Object(Tree, Blob, Commit).
#[derive(Ord, PartialOrd, Clone, PartialEq, Eq, Debug)]
pub struct ObjectId([u8; 20]);
impl ObjectId {
    pub fn new(oid: [u8; 20]) -> Self {
        Self(oid)
    }
}
impl Deref for ObjectId {
    type Target = [u8; 20];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<U> AsRef<U> for ObjectId
where
    U: ?Sized,
    <ObjectId as Deref>::Target: AsRef<U>,
{
    fn as_ref(&self) -> &U {
        self.deref().as_ref()
    }
}

impl TryFrom<&str> for ObjectId {
    type Error = io::Error;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
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
            .map(|vals| Ok(decode_hex(vals[0])? << 4 | decode_hex(vals[1])?))
            .collect::<io::Result<Vec<u8>>>()?;
        let mut re = [0_u8; 20];
        re.copy_from_slice(&v[..]);

        Ok(Self::new(re))
    }
}
impl From<[u8; 20]> for ObjectId {
    fn from(oid: [u8; 20]) -> Self {
        Self::new(oid)
    }
}
impl TryFrom<&[u8]> for ObjectId {
    type Error = io::Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 20 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Valid object id of sha1 is 20 bytes",
            ));
        }
        let mut oid = [0; 20];
        oid.copy_from_slice(bytes);
        Ok(Self::new(oid))
    }
}
impl From<&ObjectId> for String {
    fn from(oid: &ObjectId) -> String {
        oid.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}

/// Changes u8 of char to hex of u4 representation
/// treating upper/lower cases same.
fn decode_hex(c: char) -> io::Result<u8> {
    let i = match c {
        'a'..='f' => c as u8 - b'a' + 10,
        'A'..='F' => c as u8 - b'A' + 10,
        '0'..='9' => c as u8 - b'0',
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid character",
            ));
        }
    };
    Ok(i)
}

/// Represents Object(Commit, Tree, Blob),
/// with byte-content, type, length of the byte-content.
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
impl TryFrom<ObjectRaw> for Object {
    type Error = io::Error;
    fn try_from(raw: ObjectRaw) -> Result<Self, Self::Error> {
        // 1. Get seperator indices.
        let space_pointer =
            raw.data.iter().position(|&b| b == b' ').ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "cannot parse raw object")
            })?;
        let null_pointer =
            raw.data.iter().position(|&b| b == b'\0').ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidData, "cannot parse raw object")
            })?;

        // 2. Seperate raw data
        let kind_raw = raw.data.get(..space_pointer).unwrap();
        let size_raw = raw.data.get(space_pointer + 1..null_pointer).unwrap();
        let data = raw.data.get(null_pointer + 1..).unwrap();

        // 3. Parse them
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
        let size = decode_u32_from_bytes(size_raw)?;

        // 4. Create new object
        let obj = Object::new(kind, size as usize, data.to_vec());
        // let obj = Object::new(kind, 0, data.to_vec());
        Ok(obj)
    }
}

fn decode_u32_from_bytes(bytes: &[u8]) -> io::Result<u32> {
    let mut t = 0_u64;
    for &b in bytes {
        let n = (b as char)
            .to_digit(10)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "something went wrong"))?;
        t *= 10;
        t += n as u64;

        if t > u32::MAX as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "size cannot be larger than u32::MAX",
            ));
        }
    }
    Ok(t as u32)
}

#[derive(Debug)]
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

/// Byte representation of [`Object`],
/// actual one to store/retrieve.
pub struct ObjectRaw {
    pub data: Vec<u8>,
}
impl ObjectRaw {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
impl From<Object> for ObjectRaw {
    fn from(obj: Object) -> ObjectRaw {
        let mut out = Vec::with_capacity(256);
        write!(&mut out, "{} {}\0", obj.kind.to_str(), obj.size).unwrap();
        out = [out, obj.data].concat();
        //out.copy_from_slice(&obj.data);

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
