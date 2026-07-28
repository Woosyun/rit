* Repository (local)
* RepositoryManger (local, remote)
* Workspace (??)

## 형상 변경 통제 예시
* 기술문서: 작성 - 승인 - 기준선설정
* 도면/BOM: 작성 - 승인 - iPCA - 기준선설정
* software: 통합영역 - code inspection - 빌드 - 입고

## 생각해볼 점
형상 관리를 사용할 수 있는 곳은 이미 사용했다.
사용하기 힘든 곳에서는 어떻게 Migration이 이뤄져야 하는지?

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
