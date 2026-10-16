pub struct Commit {
    tree: super::Oid,
    parent: super::Oid,
    author: Author,
    committer: Committer,
    message: String,
}
impl Commit {
    pub fn new(
        root_oid: super::Oid,
        parent: super::Oid,
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

impl super::IntoObject for Commit {
    fn into_object(self) -> Vec<u8> {
        let tree = [format!("tree ").into_bytes(), self.tree.0.to_vec()].concat();
        let author = format!(
            "{} <#{}> #{}",
            self.author.name, self.author.email, self.author.timestamp
        )
        .into_bytes();
        let committer = format!(
            "{} <#{}> #{}",
            self.committer.name, self.committer.email, self.committer.timestamp
        )
        .into_bytes();
        [
            tree,
            self.parent.0.to_vec(),
            author,
            committer,
            self.message.as_bytes().to_vec(),
        ]
        .concat()
    }
}
