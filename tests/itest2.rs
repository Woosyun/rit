mod itest;
use itest::*;

#[test]
pub fn checkout_when_workspace_is_not_clean() -> rit::Result<()> {
    let mut client = Client::build("checkout")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;
    
    client.work()?;
    client.try_checkout(/*previous revison?*/)
}

#[test]
pub fn checkout_to_previous_revision() -> rit::Result<()> {
    let mut client = Client::build("checkout-to-preivous-revision")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;

    client.work()?;
    client.try_commit()?;
    client.try_checkout()
}
#[test]
pub fn checkout_to_original_reivision() -> rit::Result<()> {
    let mut client = Client::build("checkout-to-original-revision")?;
    client.init()?;

    client.work()?;
    client.try_commit()?;

    client.work()?;
    client.try_commit()?;
    client.try_checkout()?;

    client.try_checkout()
}

#[test]
pub fn branch() -> rit::Result<()> {
    todo!("test branch");
}

#[test]
pub fn log() -> rit::Result<()> {
    todo!("test log");
}
