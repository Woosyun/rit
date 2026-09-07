## test 방법
* OS와 상호작용하는 부분은 분리하여 테스트 한다.

```Rust
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
