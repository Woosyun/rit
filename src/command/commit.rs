use crate::prelude::*;
use std::path::PathBuf;

pub fn commit(path: PathBuf, message: String) -> Result<(), String> {
    // todo: pass closure that store all tree and blob
    let mut db_path = path.clone();
    db_path.push(Database::folder_name());

    let database = Database::build(db_path);

    // scan files from workspace
    let files = Workspace::list_files(path.clone())
        .map_err(|e| e.to_string())?;
    
    // store files as blobs
    let entries = files
        .into_iter()
        .map(|file| {
            // read file
            // blob::build
            // store it in database
            let content = String::new();
            let blob = Blob::build(content);
            database.store(blob)
            Entry::from(blob)
        })
        .collect::<Vec<_>>();

    // store entries in tree
    let mut tree = Tree::new();
    entries
        .into_iter()
        .map(|entry| {
            // todo
            // make sure ancestors only includes 
            // path between root folder(".") and target file
            // => workdir > ... > entry.path
            let ancestors = entry.path.ancestors();
            tree.add_entry(Entry::from(blob))
        })
        .collect::<Vec<_>>();

    let block = |tree: Tree| {
        database.store(tree)
    };
    tree.traverse(block);

    Ok(())
}
