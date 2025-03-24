use crate::workspace::Stat;

pub struct Blob {
    name: String,
    stat: Stat,
    content: String
}
impl Blob {
    pub fn new(name: String, stat: Stat, content: String) -> Self {
        Self {
            name,
            stat,
            content,
        }
    }
}
