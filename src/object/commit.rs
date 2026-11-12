use super::{Object, ObjectId, ObjectKind};

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
    fn from(commit: Commit) -> Object {
        let author = commit.author;
        let author =
            format!("{} <#{}> #{}", author.name, author.email, author.timestamp).into_bytes();
        let committer = commit.committer;
        let committer = format!(
            "{} <#{}> #{}",
            committer.name, committer.email, committer.timestamp
        )
        .into_bytes();
        let data = [
            commit.parent.to_vec(),
            author,
            committer,
            commit.message.as_bytes().to_vec(),
        ]
        .concat();
        Object::new(ObjectKind::Commit, data.len(), data)
    }
}
