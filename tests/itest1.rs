/*
* test cases for various task scenarios
*
* since change can be discovered only when mtime is different,
* calling sleep_1_sec() between works is neccessary
*/

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

    client.sleep_1_sec();

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

    client.sleep_1_sec();
    
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

    client.sleep_1_sec();

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

    client.sleep_1_sec();

    client.try_branch_create("branch_0")?;
    client.try_checkout("branch_0")?;
    client.try_work()?;
    client.try_commit()?;

    client.sleep_1_sec();

    client.try_branch_create("branch_0_0")?;
    client.try_checkout("branch_0_0")?;
    client.try_work()?;
    client.try_commit()?;

    client.try_checkout("main")?;
    client.try_checkout("branch_0")
}

#[test]
#[should_panic]
pub fn merge_conflict_occurred() {
    let mut client = Client::build("merge-conflict-occurred").unwrap();
    client.try_init().unwrap();

    client.try_work().unwrap();
    client.try_commit().unwrap();

    let indices = client.shuffle_files().unwrap();
    let number_to_touch = indices.len()/4;
    let mut indices_to_remove = Vec::with_capacity(number_to_touch);
    let mut indices_to_modify = Vec::with_capacity(number_to_touch);
    for (i, index) in indices.into_iter().enumerate() {
        if i < number_to_touch {
            indices_to_remove.push(index);
        } else if i < 2 * number_to_touch {
            indices_to_modify.push(index);
        } else {
            break;
        }
    }

    client.sleep_1_sec();

    client.try_branch_create("branch").unwrap();
    client.try_checkout("branch").unwrap();
    for index in indices_to_modify.iter() {
        client.modify(index).unwrap();
    }
    for index in indices_to_remove.iter() {
        client.remove(index).unwrap();
    }
    client.try_commit().unwrap();

    client.sleep_1_sec();

    client.try_checkout("main").unwrap();
    for index in indices_to_modify.iter() {
        client.remove(index).unwrap();
    }
    for index in indices_to_remove.iter() {
        client.modify(index).unwrap();
    }
    client.try_commit().unwrap();

    client.try_merge_branch("branch").unwrap();
}

#[test]
fn merge_compatible_branches() -> rit::Result<()> {
    let mut client = Client::build("merge-compatible-branches")?;
    client.try_init()?;

    client.try_work()?;
    client.try_commit()?;
    
    //select files
    let indices = client.shuffle_files()?;
    let number_to_touch = indices.len()/4;
    let mut indices_to_remove = Vec::with_capacity(number_to_touch);
    let mut indices_to_modify1 = Vec::with_capacity(number_to_touch);
    let mut indices_to_modify2 = Vec::with_capacity(number_to_touch);
    for (i, index) in indices.iter().enumerate() {
        if i < number_to_touch {
            indices_to_remove.push(index);
        } else if i < 2 * number_to_touch {
            indices_to_modify1.push(index);
        } else if i < 3 * number_to_touch {
            indices_to_modify2.push(index);
        } else {
            break;
        }
    }

    client.sleep_1_sec();

    client.try_branch_create("branch")?;
    client.try_checkout("branch")?;
    for index in indices_to_modify1 {
        client.modify(index)?;
    }
    for index in indices_to_remove {
        client.remove(index)?;
    }

    client.try_commit()?; // errorneous!!
    
    client.sleep_1_sec();

    client.try_checkout("main")?;
    for index in indices_to_modify2 {
        client.modify(index)?;
    }
    client.try_commit()?;

    client.try_merge_branch("branch")?;
    Ok(())
}
