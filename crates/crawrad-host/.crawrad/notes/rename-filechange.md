---
id: rename-filechange
kind: doc
target_file: src/rename.rs
line_hint: 110
author: 
created: 
title: "FileChange"
tags: []
signature: "pub struct FileChange"
---

One file of a rename: how many occurrences, its text before and after.

# Examples

```rust
use crawrad_host::rename::FileChange;
use std::path::PathBuf;

let change = FileChange {
    file: PathBuf::from("src/lib.rs"),
    count: 1,
    before: "foo".into(),
    after: "bar".into(),
};
assert_eq!(change.count, 1);
```
