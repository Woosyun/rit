mod itest;
use itest::*;

#[test]
#[should_panic]
pub fn checkout_when_workspace_is_not_clean() -> () {
    let mut client = Client::build("checkout-when-workspace-is-not-clean").unwrap();
    client.init().unwrap();

    client.work().unwrap();
    client.try_commit().unwrap();
    client.try_branch_create("new_branch").unwrap();
    
    client.work().unwrap();
    client.try_checkout("new_branch").unwrap();
}

#[test]
pub fn checkout_from_clean_workspace() -> rit::Result<()> {
    let mut client = Client::build("checkout-from-clean-workspace")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;
    
    client.try_branch_create("new_branch")?;
    client.try_checkout("new_branch")?;

    client.work()?;
    client.try_commit()?;
    client.try_checkout("main")
}
