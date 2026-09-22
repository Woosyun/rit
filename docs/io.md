## test method
* Testing parts Interacting with operating system seperately is prefered.

```Rust
/// example

pub trait Write {
    pub fn write_all(&self, content: &[u8]) -> std::io::Result<()>;
}

pub fn command1(
    writer: impl Write,
    content: &[u8],
) {
    //...
}

pub struct FakeWrite {
    fail: bool,
    content: Vec<u8>,
};
impl Write for FakeWrite {
    pub fn write_all(&self, content: &[u8]) -> std::io::Result<()> {
        if self.fail {
            return Err("Something went wrong");
        }
        self.content.??
    }
}
```

* unit test coverage should be 100%
