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

//merge
#[test]
pub fn merge_compatible_branches() -> rit::Result<()> {
    let mut client = Client::build("merge-compatible-branches")?;
    client.try_init()?;

    client.try_work()?;
    println!("initial commit");
    client.try_commit()?;
    
    //select files
    let files = client.shuffle_files()?;
    let number_to_touch = files.len()/4;
    let mut files_to_remove = Vec::with_capacity(number_to_touch);
    let mut files_to_modify1 = Vec::with_capacity(number_to_touch);
    let mut files_to_modify2 = Vec::with_capacity(number_to_touch);
    for (i, file) in files.iter().enumerate() {
        if i < number_to_touch {
            files_to_remove.push(file);
        } else if i < 2 * number_to_touch {
            files_to_modify1.push(file);
        } else if i < 3 * number_to_touch {
            files_to_modify2.push(file);
        } else {
            break;
        }
    }

    client.try_branch_create("branch")?;
    println!("checkout to 'branch' branch");
    client.try_checkout("branch")?;
    client.modify(&files_to_modify1)?;
    client.remove(&files_to_remove)?;
    client.add(number_to_touch)?;
    println!("commit on 'branch' branch");
    client.try_commit()?;

    println!("checkout to 'main' branch");
    client.try_checkout("main")?;
    client.modify(&files_to_modify2)?;
    client.remove(&files_to_remove)?;
    client.add(number_to_touch)?;
    println!("commit on 'main' branch");
    client.try_commit()?;

    client.try_merge_branch("branch")?;
    Ok(())
}
#[test]
#[should_panic]
pub fn merge_conflict_occurred() {
    let mut client = Client::build("merge-conflict-occurred").unwrap();
    client.try_init().unwrap();

    client.try_work().unwrap();
    client.try_commit().unwrap();

    let files = client.shuffle_files().unwrap();
    let number_to_touch = files.len()/4;
    let mut files_to_remove = Vec::with_capacity(number_to_touch);
    let mut files_to_modify = Vec::with_capacity(number_to_touch);
    for (i, file) in files.iter().enumerate() {
        if i < number_to_touch {
            files_to_remove.push(file);
        } else if i < 2 * number_to_touch {
            files_to_modify.push(file);
        } else {
            break;
        }
    }
    client.try_branch_create("branch").unwrap();
    client.try_checkout("branch").unwrap();
    client.modify(&files_to_modify).unwrap();
    client.remove(&files_to_remove).unwrap();
    client.add(number_to_touch).unwrap();
    client.try_commit().unwrap();

    client.try_checkout("main").unwrap();
    client.modify(&files_to_remove).unwrap();
    client.remove(&files_to_modify).unwrap();
    client.add(number_to_touch).unwrap();
    client.try_commit().unwrap();

    client.try_merge_branch("branch").unwrap();
}
