use super::*;
use std::io::Write;

pub struct Commit {
    tree: ObjectId,
    parent: ObjectId,
    author: Author,
    committer: Committer,
    message: String,
}
impl Commit {
    pub fn new(
        root_oid: ObjectId,
        parent: ObjectId,
        author: Author,
        committer: Committer,
        message: String,
    ) -> Self {
        Self {
            tree: root_oid,
            parent,
            author,
            committer,
            message,
        }
    }
}

#[derive(Clone)]
pub struct Author {
    name: String,
    email: String,
    timestamp: u64,
}
pub type Committer = Author;
impl Author {
    pub fn new(name: String, email: String, timestamp: u64) -> Self {
        Self {
            name,
            email,
            timestamp,
        }
    }
}

impl From<&Commit> for Object {
    fn from(commit: &Commit) -> Object {
        let mut data = Vec::with_capacity(256);
        writeln!(&mut data, "tree {}", encode_to_hex(&commit.tree)).unwrap();
        writeln!(&mut data, "parent {}", encode_to_hex(&commit.parent)).unwrap();
        writeln!(
            &mut data,
            "author {} <{}> {}",
            commit.author.name, commit.author.email, commit.author.timestamp
        )
        .unwrap();
        writeln!(
            &mut data,
            "committer {} <{}> {}",
            commit.committer.name, commit.committer.email, commit.committer.timestamp
        )
        .unwrap();

        write!(&mut data, "\n{}", commit.message).unwrap();

        Object::new(ObjectKind::Commit, data.len(), data)
    }
}
