use std::{
    path::PathBuf,
    fmt::Write,
};
use crate::{
    workspace::{Workspace},
    repository::{Repository},
    revision::*,
};

pub struct Status {
    ws: Workspace,
    repo: Repository,
}
impl Status {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;
        Ok(Self {
            ws,
            repo,
        })
    }
    
    fn scan(&self) -> crate::Result<RevDiff> {
        let parent = self.repo.get_head()?;
        let prev_rev = Revision::build(self.repo.clone(), parent.as_ref())?;
        let prev_rev = prev_rev.into_rev()?;
        let curr_rev = self.ws.into_rev()?;

        let rev_diff = prev_rev.diff(&curr_rev)?;
        Ok(rev_diff)
    }

    pub fn execute(&self) -> crate::Result<()> {
        let rev_diff = self.scan()?;
        let mut output = String::new();
        writeln!(output, "added files").unwrap();
        for entry in rev_diff.added.iter() {
            writeln!(output, "{:?}", entry).unwrap();
        }
        writeln!(output, "removed files").unwrap();
        for entry in rev_diff.removed.iter() {
            writeln!(output, "{:?}", entry).unwrap();
        }
        writeln!(output, "modified files").unwrap();
        for entry in rev_diff.modified.iter() {
            writeln!(output, "{:?}", entry).unwrap();
        }

        println!("{}", output);
        Ok(())
    }
}
