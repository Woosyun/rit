mod test_harness;
use test_harness::TestHarness;

#[test]
#[should_panic(expected = "cannot run commit without repository initialized")]
fn commit_without_repository_initialized() {
    let th = TestHarness::new();
    th.try_commit("errorneous commit", "me", "me1234@qwer.com");
}

#[test]
fn commit_once() {
    let th = TestHarness::new();
    th.try_init();
    th.try_commit("errorneous commit", "me", "me1234@qwer.com");
}
