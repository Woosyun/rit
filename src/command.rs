use std::{error, path};

/// ### Example
/// ```
/// struct Commit {
///     commit_message: String,
/// }
/// impl Command<Commit, CommitBuilder> for Commit {
///     fn builder() -> CommitBuilder {
///         CommitBuilder::new()
///     }
///     fn execute(self) -> Result<(), Box<dyn std::error::Error>> {
///         println!("commit executed with message: {}", self.commit_message);
///         Ok(())
///     }
/// }
/// ```
pub trait Command<C, B: CommandBuilder<C>> {
    fn builder() -> B;
    fn execute(self) -> Result<(), Box<dyn error::Error>>;
}

/// ## CommandBuilder
/// * Responsible for validating given path is right.
///
/// ### Example
/// ```
/// struct CommitBuilder {
///     commit_message: Option<String>,
/// }
/// impl CommandBuilder<Commit> for CommitBuilder {
///     fn new() -> CommitBuilder {
///         Self {
///             commit_message: None,
///         }
///     }
///     fn build(self) -> Result<Commit, Box<dyn std::error::Error>> {
///         let Some(commit_message) = self.commit_message else {
///             return Err("Commit needs commit message. Message Not Found".into());
///         };
///         return Commit { commit_message };
///     }
/// }
/// ```
pub trait CommandBuilder<C> {
    fn new() -> Self;
    fn working_directory<P: AsRef<path::Path>>(self, workdir: P) -> Self;
    fn build(self) -> Result<C, Box<dyn error::Error>>;
}
