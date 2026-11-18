mod test_harness;
use test_harness::TestHarness;

#[test]
fn init_succeed_on_empty_directory() {
    let th = TestHarness::new();
    th.try_init();
}

#[test]
fn init_pass_on_existing_repository() {
    let th = TestHarness::new();
    th.try_init();
}
