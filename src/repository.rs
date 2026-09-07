pub struct Tree {
    entries: Vec<TreeEntry>,
}
impl Tree {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, _new_entry: Entry) {
        todo!("add entry");
    }
}

pub enum TreeEntry {
    Tree(Tree),
    Entry(Entry),
}

pub struct Entry {
    //...
}
