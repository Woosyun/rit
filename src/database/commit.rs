#![allow(unused)]

pub struct Commit {
    tree: super::Oid,
    parent: super::Oid,
    author: Author,
    committer: Committer,
    message: String,
}
pub struct Author {
    name: String,
    email: String,
    timestamp: u64,
}
pub struct Committer {
    name: String,
    email: String,
    timestamp: u64,
}


impl super::IntoObject for Commit {
    fn into_object(self) -> Vec<u8> {
        let tree = [format!("tree ").into_bytes(), self.tree.0.to_vec()].concat();
        let parent = [format!("parent ").into_bytes(), self.parent.0.to_vec()].concat();
        let author = format!("{} <#{}> #{}", self.author.name, self.author.email, self.author.timestamp).into_bytes();
        let committer = format!("{} <#{}> #{}", self.committer.name, self.committer.email, self.committer.timestamp).into_bytes();
        [tree, parent, author, committer, self.message.as_bytes().to_vec()].concat()
    }
}
