use rit::prelude::*;
use std::path::Path;

pub trait Driver {
    fn workdir(&self) -> &Path;
    fn workspace(&self) -> Result<Workspace>;
    fn repository(&self) -> Result<Repository>;
}
