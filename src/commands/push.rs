/*
* 1. trait
* trait Command
* struct CommandExecutor
* => simple for user,
* => complex for developer
* 
* 2. struct 
* struct Command
* => complex for user
* => simple for developer
*/

pub trait Push {
    fn push(&self /*repository info, user info*/) -> Result<()> {
        let command = PushExecutor::build()?;
        command.execute()
    }
}

struct PushExecutor {
    ws: Workspace,
    user_info: UserInfo,
    repo_info: RepoInfo,
}
