From

```mermaid
gitGraph
    commit id: "a"
    commit id: "b"
    commit id: "c" tag: "HEAD"
```

stash will create

```mermaid
gitGraph
    commit id: "a"
    commit id: "b"
    commit id: "c" tag: "HEAD"
    branch s1
    commit id: "s1"
    checkout main
    merge s1 id: "s2"
```

s1 stores current state of Index.

s2 stores unstaged changes. Cannot use "add ." because that would include all untracked files, not modified files only.

To go back c from s2, run "reset s1", "reset --soft c"