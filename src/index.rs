use crate::prelude::*;
use std::io::{Cursor, Read};
use std::os::unix::fs::MetadataExt;
use std::{fs, io};

/// # Index
#[derive(Default)]
pub struct Index {
    header: IndexHeader,
    //todo: use stage
    pub entries: Vec<IndexEntry>,
    extensions: Vec<IndexExtension>,
    checksum: [u8; 20],
}
impl Index {
    pub fn entries(&self) -> &Vec<IndexEntry> {
        &self.entries
    }
    pub fn entries_inner(self) -> Vec<IndexEntry> {
        self.entries
    }
    pub fn parse(bytes: &[u8]) -> io::Result<Self> {
        // 12-byte header + 20-byte checksum
        if bytes.len() < 32 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Too short to be Index",
            ));
        }

        // checksum
        let content_len = bytes.len() - 20;
        let expected_checksum = &bytes[content_len..];
        let calculated_checksum = hash(&bytes[..content_len]);
        if expected_checksum != &calculated_checksum {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "checksum is invalid",
            ));
        }

        let mut cursor = Cursor::new(&bytes[..content_len]);

        // Signature
        let mut signature = [0u8; 4];
        cursor.read_exact(&mut signature)?;
        if signature != DIRC_SIGNATURE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid DIRC signature",
            ));
        }

        // Header
        let version = read_u32(&mut cursor)?;
        if !(2..=4).contains(&version) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid index version",
            ));
        }

        // number of entries
        let number_of_entries = read_u32(&mut cursor)?;
        // entries
        let mut entries = Vec::with_capacity(number_of_entries as usize);
        for _ in 0..number_of_entries {
            let entry_start = cursor.position();

            let ctime_sec = read_u32(&mut cursor)?;
            let ctime_nsec = read_u32(&mut cursor)?;
            let mtime_sec = read_u32(&mut cursor)?;
            let mtime_nsec = read_u32(&mut cursor)?;
            let dev = read_u32(&mut cursor)?;
            let ino = read_u32(&mut cursor)?;
            let mode = read_u32(&mut cursor)?;
            let uid = read_u32(&mut cursor)?;
            let gid = read_u32(&mut cursor)?;
            let file_size = read_u32(&mut cursor)?;

            let mut checksum = [0u8; 20];
            cursor.read_exact(&mut checksum)?;

            let flags = read_u16(&mut cursor)?;
            let is_extended = (flags & 0x4000) != 0;
            let extended_flags = if version >= 3 && is_extended {
                Some(read_u16(&mut cursor)?)
            } else {
                None
            };

            let mut name_bytes = Vec::new();
            let mut b = [0u8; 1];
            loop {
                cursor.read_exact(&mut b)?;
                if b[0] == 0 {
                    // until b != '\0\
                    break;
                }
                name_bytes.push(b[0]);
            }
            let name = String::from_utf8(name_bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;
            //let relative_path = path::PathBuf::from(relative_path).try_into()?;
            let relative_path = RelativePath::build(name)?;

            let entry_len = cursor.position() - entry_start;
            let pad_len = (8 - (entry_len % 8)) % 8;
            if pad_len > 0 {
                let mut padding = vec![0u8; pad_len as usize];
                cursor.read_exact(&mut padding)?;
            }

            let index_entry = IndexEntry::new(
                ctime_sec,
                ctime_nsec,
                mtime_sec,
                mtime_nsec,
                dev,
                ino,
                mode,
                uid,
                gid,
                file_size,
                checksum,
                flags,
                extended_flags,
                relative_path,
            );

            entries.push(index_entry);
        }

        //optional extensions
        let mut extensions = Vec::new();
        while (cursor.position() as usize) < content_len {
            let mut ext_sig = [0u8; 4];
            cursor.read_exact(&mut ext_sig)?;
            let ext_size = read_u32(&mut cursor)? as usize;

            let mut ext_data = vec![0u8; ext_size];
            cursor.read_exact(&mut ext_data)?;

            extensions.push(IndexExtension {
                signature: ext_sig,
                data: ext_data,
            });
        }

        let mut checksum = [0u8; 20];
        checksum.copy_from_slice(expected_checksum);

        Ok(Index {
            header: IndexHeader {
                signature: Default::default(),
                version,
                number_of_entries,
            },
            entries,
            extensions,
            checksum,
        })
    }
}

fn read_u32<R: Read>(r: &mut R) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}
fn read_u16<R: Read>(r: &mut R) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
}

impl From<Vec<IndexEntry>> for Index {
    fn from(entries: Vec<IndexEntry>) -> Self {
        //todo: optimize how to hash.
        let entries_raw = entries
            .iter()
            .map(|entry| <Object as From<IndexEntry>>::from(entry.clone()))
            .flatten()
            .collect::<Vec<u8>>();
        let checksum = hash(entries_raw);

        Self {
            entries,
            checksum,
            ..Default::default()
        }
    }
}

const DIRC_SIGNATURE: [u8; 4] = ['D' as u8, 'I' as u8, 'R' as u8, 'C' as u8];
pub struct IndexHeader {
    signature: [u8; 4],
    version: u32,
    number_of_entries: u32,
}
impl Default for IndexHeader {
    fn default() -> Self {
        Self {
            signature: DIRC_SIGNATURE,
            version: 2,
            number_of_entries: 0,
        }
    }
}
impl From<IndexHeader> for Object {
    fn from(header: IndexHeader) -> Self {
        [
            header.signature.to_vec(),
            header.version.to_be_bytes().to_vec(),
            header.number_of_entries.to_be_bytes().to_vec(),
        ]
        .concat()
    }
}

pub struct IndexExtension {
    pub signature: [u8; 4],
    pub data: Vec<u8>,
}
impl From<IndexExtension> for Object {
    fn from(ext: IndexExtension) -> Self {
        [ext.signature.to_vec(), ext.data].concat()
    }
}

#[derive(Clone)]
pub struct IndexEntry {
    ctime: u32,
    ctime_nano: u32,
    mtime: u32,
    mtime_nano: u32,
    dev: u32,
    ino: u32,
    pub mode: u32,
    uid: u32,
    gid: u32,
    file_size: u32,
    pub oid: [u8; 20],

    /// 1-bit assume-valid flag
    /// 1-bit extended flag (must be zero in version 2)
    /// 2-bit stage (used during merge)
    /// 12-bit length of file name, max(0xfff, len(file name))
    flags: u16,

    extended_flags: Option<u16>,
    pub relative_path: RelativePath,
}
impl IndexEntry {
    fn new(
        ctime: u32,
        ctime_nano: u32,
        mtime: u32,
        mtime_nano: u32,
        dev: u32,
        ino: u32,
        mode: u32,
        uid: u32,
        gid: u32,
        file_size: u32,
        oid: [u8; 20],
        flags: u16,
        extended_flags: Option<u16>,
        relative_path: RelativePath,
    ) -> Self {
        Self {
            ctime,
            ctime_nano,
            mtime,
            mtime_nano,
            dev,
            ino,
            mode,
            uid,
            gid,
            file_size,
            oid,
            flags,
            extended_flags,
            relative_path,
        }
    }
    pub fn mode(&self) -> &u32 {
        &self.mode
    }
    pub fn oid(&self) -> &[u8; 20] {
        &self.oid
    }
    pub fn relative_path(&self) -> &RelativePath {
        &self.relative_path
    }
}

impl From<(fs::Metadata, ObjectId, RelativePath)> for IndexEntry {
    fn from((stat, oid, relative_path): (fs::Metadata, ObjectId, RelativePath)) -> Self {
        Self {
            ctime: stat.ctime() as u32,
            ctime_nano: stat.ctime_nsec() as u32,
            mtime: stat.mtime() as u32,
            mtime_nano: stat.mtime_nsec() as u32,
            dev: stat.dev() as u32,
            ino: stat.ino() as u32,
            mode: stat.mode() as u32,
            uid: stat.uid() as u32,
            gid: stat.gid() as u32,
            file_size: stat.size() as u32,
            oid,
            flags: Default::default(),
            extended_flags: None,
            relative_path,
        }
    }
}

impl From<IndexEntry> for Object {
    fn from(entry: IndexEntry) -> Self {
        let ten_bytes = [
            entry.ctime,
            entry.ctime_nano,
            entry.mtime,
            entry.mtime_nano,
            entry.dev,
            entry.ino,
            entry.mode,
            entry.uid,
            entry.gid,
            entry.file_size,
        ]
        .into_iter()
        .map(|u| u.to_be_bytes())
        .collect::<Vec<[u8; 4]>>()
        .concat();
        //todo: consider extended flags,
        [
            ten_bytes,
            entry.flags.to_be_bytes().to_vec(),
            entry
                .relative_path
                .to_string_lossy()
                .into_owned()
                .into_bytes(),
        ]
        .concat()
    }
}

impl From<Index> for Object {
    fn from(index: Index) -> Object {
        [
            //<IndexHeader as Into<Object>>::into(index.header),
            <Object as From<IndexHeader>>::from(index.header),
            index
                .entries
                .into_iter()
                .map(|entry| <Object as From<IndexEntry>>::from(entry))
                .flatten()
                .collect::<Vec<u8>>(),
            index
                .extensions
                .into_iter()
                .map(|ext| <Object as From<IndexExtension>>::from(ext))
                .flatten()
                .collect::<Vec<u8>>(),
            index.checksum.to_vec(),
        ]
        .concat()
    }
}
