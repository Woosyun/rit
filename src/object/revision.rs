use super::{Object, ObjectId};

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

impl From<Commit> for Object {
    fn from(revision: Commit) -> Object {
        let tree = [format!("tree ").into_bytes(), revision.tree.to_vec()].concat();
        let author = revision.author;
        let author =
            format!("{} <#{}> #{}", author.name, author.email, author.timestamp).into_bytes();
        let committer = revision.committer;
        let committer = format!(
            "{} <#{}> #{}",
            committer.name, committer.email, committer.timestamp
        )
        .into_bytes();
        [
            tree,
            revision.parent.to_vec(),
            author,
            committer,
            revision.message.as_bytes().to_vec(),
        ]
        .concat()
    }
}
