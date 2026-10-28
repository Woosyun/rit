pub trait Loader {
    fn set_lockfile();
    fn load();
    fn load_mut();
}
