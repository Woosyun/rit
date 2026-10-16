## Blob
```{BLOB_TYPE} {blob.bytes of original contents}\0{contents}```
* Oid is computed from this, before compress

## Tree
1. ```{TREE_TYPE}?{number of entries}?[entries]```

### Entry
```{mode: String} {path}\0{oid}```

## [Index](https://git-scm.com/docs/index-format)
1. 12-byte Header
2. Entries
3. 20-byte hash
* Sort by file name(String of unsigned bytes) asc, sort by stage asc
### Header
1. 4-byte, signature. {"D", "I", "R", "C"} (stands for "dircache")
2. 4-byte, version number. supported ones are 2, 3, 4.
3. 32-bit, number of entries
### Entry
1. Ten 4-byte numbers from stat()
    1. ctime seconds, last time file's metadata changed
    2. ctime nanoseconds, fractions
    3. mtime seconds, last time file's content changed
    4. mtime nanoseconds fractions
    5. dev, id of the HW device the file resides on
    6. ino, number of inode storing its attributes
    7. mode: String, permissions ### todo: ???
        1. 4-bit object type
        2. 3-bit unused
        3. 9-bit unix permissions, only 0755 and 0644 are valid for regular files.
    8. uid, user id
    9. gid, group id
    10. file size
2. 20-byte SHA-1 hash, packed bytes form
3. 2-byte set of flags: 16-bit unsigned big-endian number, pack serveral pieces of information (high to low)
    1. 1-bit assume-valid flag
    2. 1-bit extended flag (must be zero in version 2)
    3. 2-bit stage (during merge)
    4. 12-bit length of file name, max(0xfff, length of file name)
4. (Version 3 or later) A 16-bit field, only applicable if the "extended flag" above is 1, split into (high to low bits)
    1. 1-bit reserved for future
    2. 1-bit skip-worktree flag (used by sparse checkout)
    3. 1-bit intent-to-add flag (used by "git add -N")
    4. 13-bit unused, must be zero
5. File name
6. Padded with "\0" to make total length multiple of 8 bytes. At least 1 null byte


