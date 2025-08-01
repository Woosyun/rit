use crate::prelude::*;
use std::{
    fs,
    path::{PathBuf, Path},
    collections::{VecDeque, HashSet},
};

pub struct Merge {
    ws: Workspace,
    repo: Repository,
    target_branch: Result<String>,
    current_branch: String,
}
impl Merge {
    pub fn build(workdir: PathBuf) -> Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        let target_branch = Err(Error::Commands("target branch is not set".to_string()));

        if !ws.into_rev()?
            .diff(&repo.into_rev()?)?
            .is_clean() 
        {
            return Err(Error::Workspace("workspace is not clean".to_string()));
        }

        let current_branch = match repo.local_head.get()? {
            Head::Branch(branch) => Ok(branch),
            _ => Err(Error::Commands("Cannot run merge on non-branch".into())),
        }?;

        let merge = Self {
            ws,
            repo,
            target_branch,
            current_branch,
        };
        Ok(merge)
    }

    pub fn set_target_branch(&mut self, branch: String) {
        self.target_branch = Ok(branch);
    }

    fn find_base(&self) -> Result<FindBase> {
        let from_branch = self.repo.refs.get(&self.current_branch)?;
        let from = from_branch.leaf();
        //let to = self.repo.refs.get(self.target_branch.as_ref().map_err(|e| e.clone())?)?;
        let to_branch = self.target_branch.as_ref()
            .map_err(|e| e.clone())
            .map(|branch| self.repo.refs.get(branch))??;
        let to = to_branch.leaf();

        let mut from_seen = HashSet::<Oid>::new();
        let mut from_que = VecDeque::<Oid>::from([from.clone()]);
        let mut to_seen = HashSet::<Oid>::new();
        let mut to_que = VecDeque::<Oid>::from([to.clone()]);

        while !from_que.is_empty() || !to_que.is_empty() {
            if let Some(oid) = from_que.pop_front() {
                if !from_seen.insert(oid.clone()) {
                    continue;
                } else if from_seen.contains(to) {
                    return Ok(FindBase::AlreadyUpToDate);
                } else if to_seen.contains(&oid) {
                    return Ok(FindBase::Base(oid.clone()));
                }

                let commit: repository::Commit = self.repo.db.retrieve(&oid)?;
                for parent_oid in commit.parents() {
                    from_que.push_back(parent_oid.to_owned());
                }
            }
            if let Some(oid) = to_que.pop_front() {
                if !to_seen.insert(oid.clone()) {
                    continue;
                }
                if to_seen.contains(from) {
                    return Ok(FindBase::FastForward);
                }

                if from_seen.contains(&oid) {
                    return Ok(FindBase::Base(oid));
                }

                let commit: repository::Commit = self.repo.db.retrieve(&oid)?;
                for parent_oid in commit.parents() {
                    to_que.push_back(parent_oid.to_owned());
                }
            }
        }

        Err(Error::Commands("cannot find base revision".to_string()))
    }

    fn diff_rev_diff(&self, from: &Rev, from_diff: &RevDiff, to: &Rev, to_diff: &RevDiff) -> Result<RevDiff> {
        let mut rev_diff = RevDiff::new();
        let conflict_error = Error::Commands("conflict detect".to_string());
        for a in to_diff.added.iter() {
            if from_diff.removed.contains(a) {
                return Err(conflict_error);
            } else if from_diff.added.contains(a) || from_diff.modified.contains(a) {
                if from.get(a).unwrap().oid()? != to.get(a).unwrap().oid()? {
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
                if from.get(m).unwrap().oid()? != to.get(m).unwrap().oid()? {
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
        let path = self.ws.workdir().join(index);
        fs::write(&path, blob)
            .map_err(|e| Error::Commands(e.to_string()))?;
        set_file_mtime(&path, mtime)
            .map_err(|e| Error::Commands(e.to_string()))
    }

    pub fn execute(&self) -> Result<()> {
        let target_branch = self.repo.refs.get(&self.target_branch.clone()?)?;
        let target_oid = target_branch.leaf();

        match self.find_base()? {
            FindBase::FastForward => {
                let mut cmd = super::checkout::Checkout::build(self.ws.workdir().to_path_buf())?;
                cmd.set_target_to_branch(self.target_branch.clone()?);
                cmd.execute()?;

                self.repo.refs.set(&self.current_branch, &target_oid)?;

                let mut cmd = super::checkout::Checkout::build(self.ws.workdir().to_path_buf())?;
                cmd.set_target_to_branch(self.current_branch.clone());
                cmd.execute()?;
            },
            FindBase::Base(oid) => {
                let from = self.repo.into_rev()?;
                let base = Revision::build(self.repo.clone(), &oid)?.into_rev()?;
                let to = Revision::build(self.repo.clone(), &target_oid)?.into_rev()?;

                let from_diff = base.diff(&from)?;
                let to_diff = base.diff(&to)?;
                let branch_diff = self.diff_rev_diff(&from, &from_diff, &to, &to_diff)?;

                for a in branch_diff.added.iter() {
                    self.upsert_workspace(a, to.get(a).unwrap())?;
                }
                for m in branch_diff.modified.iter() {
                    self.upsert_workspace(m, to.get(m).unwrap())?;
                }
                for r in branch_diff.removed.iter() {
                    let path = self.ws.workdir().join(r);
                    fs::remove_file(path)
                        .map_err(|e| Error::Commands(e.to_string()))?;
                }

                let mut commit = super::commit::Commit::build(self.ws.workdir().to_path_buf())?;
                commit.add_parent(target_oid.clone());
                commit.set_message(format!("{} merged {}", self.current_branch, self.target_branch.clone()?));
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
