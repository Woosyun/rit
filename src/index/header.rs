pub struct Header {
    signature: [char; 4],
    version: u32,
    number_of_entries: u32,
}
impl Default for Header {
    fn default() -> Self {
        Self {
            signature: ['D', 'I', 'R', 'C'],
            version: 2,
            number_of_entries: 0,
        }
    }
}
