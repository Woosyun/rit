use crate::{
    prelude::*,
    fs,
};
use std::{
    path::{PathBuf, Path},
    collections::{VecDeque, HashSet},
};

pub struct Merge {
    ws: Workspace,
    repo: Repository,
    target_branch: Result<String>,
}
impl Merge {
    pub fn build(workdir: PathBuf) -> Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        let target_branch = Err(Error::Unknown("you have to set target branch".to_string()));

        if !ws.into_rev()?
            .diff(&repo.into_rev()?)?
            .is_clean() {
            return Err(Error::Workspace("workspace is not clean".to_string()));
        }

        let merge = Self {
            ws,
            repo,
            target_branch,
        };
        Ok(merge)
    }

    pub fn set_target_branch(&mut self, branch: String) {
        self.target_branch = Ok(branch);
    }

    fn find_base(&self) -> Result<FindBase> {
        let from = self.repo.refs.get(self.repo.local_head.get()?.branch()?)?;
        let from_str = from.to_string();
        let to = self.repo.refs.get(&self.target_branch.clone()?)?;
        let to_str = to.to_string();

        let mut from_seen = HashSet::new();
        let mut from_que = VecDeque::from([from]);
        let mut to_seen = HashSet::new();
        let mut to_que = VecDeque::from([to]);

        while !from_que.is_empty() || !to_que.is_empty() {
            if let Some(oid) = from_que.pop_front() {
                let oid_str = oid.to_string();
                if from_seen.insert(oid_str.clone()) {
                    continue;
                }
                if from_seen.contains(&to_str) {
                    return Ok(FindBase::AlreadyUpToDate);
                }

                if to_seen.contains(&oid_str) {
                    return Ok(FindBase::Base(oid));
                }

                let commit: repository::Commit = self.repo.db.retrieve(&oid)?;
                for parent_oid in commit.parents() {
                    from_que.push_back(parent_oid.to_owned());
                }
            }
            if let Some(oid) = to_que.pop_front() {
                let oid_str = oid.to_string();
                if to_seen.insert(oid_str.clone()) {
                    continue;
                }
                if to_seen.contains(&from_str) {
                    return Ok(FindBase::FastForward);
                }

                if from_seen.contains(&oid_str) {
                    return Ok(FindBase::Base(oid));
                }

                let commit: repository::Commit = self.repo.db.retrieve(&oid)?;
                for parent_oid in commit.parents() {
                    to_que.push_back(parent_oid.to_owned());
                }
            }
        }

        Err(Error::Revision("cannot find base revision".to_string()))
    }
    fn branch_diff(&self, from: &Rev, from_diff: &RevDiff, to: &Rev, to_diff: &RevDiff) -> Result<RevDiff> {
        let mut rev_diff = RevDiff::new();
        let conflict_error = Error::Revision("conflict detect".to_string());
        for a in to_diff.added.iter() {
            if from_diff.removed.contains(a) {
                return Err(conflict_error);
            } else if from_diff.added.contains(a) || from_diff.modified.contains(a) {
                if from.0.get(a).unwrap().oid()? != to.0.get(a).unwrap().oid()? {
                    return Err(conflict_error);
                }
            }
            rev_diff.added.insert(a.to_path_buf());
        }
        for r in to_diff.removed.iter() {
            if from_diff.added.contains(r) || from_diff.added.contains(r) {
                return Err(conflict_error);
            }
            rev_diff.removed.insert(r.to_path_buf());
        }
        for m in to_diff.modified.iter() {
            if from_diff.added.contains(m) || from_diff.modified.contains(m) {
                if from.0.get(m).unwrap().oid()? != to.0.get(m).unwrap().oid()? {
                    return Err(conflict_error);
                }
            } else if from_diff.removed.contains(m) {
                return Err(conflict_error);
            }
            rev_diff.modified.insert(m.to_path_buf());
        }

        Ok(rev_diff)
    }

    fn upsert_workspace(&self, index: &Path, entry: &Box<dyn Stat>) -> Result<()> {
        let mtime = entry.mtime();
        let oid = entry.oid()?;
        let blob: Blob = self.repo.db.retrieve(oid)?;
        let path = self.ws.path().join(index);
        fs::write(&path, blob.content())?;
        fs::set_file_mtime(&path, mtime)
    }

    pub fn execute(&self) -> Result<()> {
        let target_oid = self.repo.refs.get(&self.target_branch.clone()?)?;
        let original_branch = self.repo.local_head.get()?.branch()?.to_string();

        match self.find_base()? {
            FindBase::FastForward => {
                let cmd = crate::commands::Checkout::build(self.ws.path().to_path_buf())?;
                cmd.execute(&self.target_branch.clone()?)?;
                self.repo.refs.set(&original_branch, &target_oid)?;
                let cmd = crate::commands::Checkout::build(self.ws.path().to_path_buf())?;
                cmd.execute(&original_branch)?;
            },
            FindBase::Base(oid) => {
                let from = self.repo.into_rev()?;
                let base = Revision::build(self.repo.clone(), &oid)?.into_rev()?;
                let to = Revision::build(self.repo.clone(), &target_oid)?.into_rev()?;
                let from_diff = base.diff(&from)?;
                let to_diff = base.diff(&to)?;
                let branch_diff = self.branch_diff(&from, &from_diff, &to, &to_diff)?;

                for a in branch_diff.added.iter() {
                    self.upsert_workspace(a, to.0.get(a).unwrap())?;
                }
                for m in branch_diff.modified.iter() {
                    self.upsert_workspace(m, to.0.get(m).unwrap())?;
                }
                for r in branch_diff.removed.iter() {
                    fs::remove_file(r)?;
                }

                let mut commit = commands::Commit::build(self.ws.path().to_path_buf())?;
                commit.add_parent(target_oid);
                commit.set_message(format!("{} merged {}", original_branch, self.target_branch.clone()?));
                commit.execute()?;
            },
            _ => ()
        }

        Ok(())
    }
}

enum FindBase {
    Base(Oid),
    FastForward,
    AlreadyUpToDate,
}
