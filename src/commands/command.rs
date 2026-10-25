use std::error;

pub trait Command<C, B: CommandBuilder<C>> {
    fn builder() -> B;
    fn execute(self) -> Result<(), Box<dyn error::Error>>;
}

pub trait CommandBuilder<C> {
    fn new() -> Self;
    fn build(self) -> Result<C, Box<dyn error::Error>>;
}
