use rit::prelude::*;
use super::driver::Driver;
use rand::prelude::*;

pub trait Commander: Driver {
    fn init(&self) -> Result<()> {
        let cmd = commands::Init::build(self.workspace()?.workdir().to_path_buf())?;
        cmd.execute()
    }
    fn commit(&self) -> rit::Result<()> {
        let mut cmd = rit::commands::Commit::build(self.workdir().to_path_buf())?;
        let message = format!("commit-{}", rand::rng().random::<u32>());
        cmd.set_message(message);
        cmd.execute()
    }
    fn merge(&self, to: &str) -> Result<()> {
        let mut merge = commands::Merge::build(self.workdir().to_path_buf())?;
        merge.set_target_branch(to.to_string());
        merge.execute()
    }
    fn create_branch(&self, new_branch: &str) -> Result<()> {
        let branch = commands::Branch::build(self.workdir().to_path_buf())?;
        branch.create(new_branch)
    }
    fn checkout(&self, branch: &str) -> Result<()> {
        let checkout = commands::Checkout::build(self.workdir().to_path_buf())?;
        checkout.execute(branch)
    }
}
