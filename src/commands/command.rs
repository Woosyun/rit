use std::error;

pub trait Command<C, B: CommandBuilder<C>> {
    fn builder() -> B;
    fn execute() -> Result<(), Box<dyn error::Error>>;
}

pub trait CommandBuilder<C> {
    fn new() -> Self;
    fn build(self) -> C;
}
