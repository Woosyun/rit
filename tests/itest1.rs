mod itest;
use itest::*;

#[test]
pub fn initialize_repository() -> rit::Result<()> {
    let user = Client::build("initialize-repository")?;
    user.try_init()
}

#[test]
pub fn work_in_main_branch() -> rit::Result<()> {
    let mut client = Client::build("work-in-main-branch")?;
    client.try_init()?;

    client.try_work()?;
    client.try_commit()?;

    client.try_work()?;
    client.try_commit()
}

#[test]
#[should_panic]
pub fn try_to_work_with_another_branch_when_workspace_is_not_clean() -> () {
    let mut client = Client::build("try_to_work_with_another_branch_when_workspace_is_not_clean").unwrap();
    client.try_init().unwrap();

    client.try_work().unwrap();
    client.try_commit().unwrap();
    client.try_branch_create("new_branch").unwrap();
    
    client.try_work().unwrap();
    client.try_checkout("new_branch").unwrap();
}
#[test]
pub fn work_with_another_branch() -> rit::Result<()> {
    let mut client = Client::build("work-with-another-branch")?;
    client.try_init()?;

    client.try_work()?;
    client.try_commit()?;
    
    client.try_branch_create("new_branch")?;
    client.try_checkout("new_branch")?;

    client.try_work()?;
    client.try_commit()?;
    client.try_checkout("main")
}
#[test]
pub fn work_with_multiple_branches() -> rit::Result<()> {
    let mut client = Client::build("work-with-multiple-branches")?;
    client.try_init()?;

    client.try_work()?;
    client.try_commit()?;

    client.try_branch_create("branch_0")?;
    client.try_checkout("branch_0")?;
    client.try_work()?;
    client.try_commit()?;

    client.try_branch_create("branch_0_0")?;
    client.try_checkout("branch_0_0")?;
    client.try_work()?;
    client.try_commit()?;

    client.try_checkout("main")?;
    client.try_checkout("branch_0")
}
