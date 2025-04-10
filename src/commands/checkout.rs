use crate::{
    workspace::Workspace,
    repository::{Repository, Oid, Blob},
    revision::{Revision, IntoRev},
    fs,
};
use std::path::{PathBuf, Path};

pub struct Checkout {
    ws: Workspace,
    repo: Repository,
}

impl Checkout {
    pub fn build(workdir: PathBuf) -> crate::Result<Self> {
        let ws = Workspace::build(workdir)?;
        let repo = Repository::build(&ws)?;

        let re = Self {
            ws,
            repo,
        };
        Ok(re)
    }
    
    pub fn execute(&self, oid: Oid) -> crate::Result<()> {
        let head = match self.repo.get_head()? {
            Some(oid) => oid,
            None => return Err(crate::Error::Workspace("repository is empty. cannot use checkout".into())),
        };
        let prev_rev = Revision::build(self.repo.clone(), Some(&head))?;
        let prev_rev = prev_rev.into_rev()?;

        let curr_rev = self.ws.into_rev()?;

        //check whether workspace is clean or not.
        let rev_diff_for_check = prev_rev.diff(&curr_rev)?;
        if !rev_diff_for_check.is_clean() {
            return Err(crate::Error::Workspace("workspace is not clean. cannot use checkout".into()));
        }

        let target_rev = Revision::build(self.repo.clone(), Some(&oid))?;
        let target_rev = target_rev.into_rev()?;
        let rev_diff = curr_rev.diff(&target_rev)?;

        //modify workspace
        let insert_to_workspace = |index: &Path| -> crate::Result<()> {
            let oid = target_rev.0.get(index).unwrap().oid()?;
            let blob: Blob = self.repo.db.retrieve(oid)?;
            let path = self.ws.path().join(index);
            fs::write(&path, blob.content())?;

            Ok(())
        };
        for a in rev_diff.added.iter() {
            insert_to_workspace(a)?;
        }
        for m in rev_diff.modified.iter() {
            insert_to_workspace(m)?;
        }
        for r in rev_diff.removed.iter() {
            let path = self.ws.path().join(r);
            fs::remove_file(&path)?;
        }

        //clear empty directories

        //update head
        self.repo.set_head(&oid)?;

        Ok(())
    }
}
