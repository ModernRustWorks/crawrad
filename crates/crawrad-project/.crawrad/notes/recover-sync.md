---
id: recover-sync
kind: doc
target_file: src/recover.rs
line_hint: 112
author: 
created: 
title: "sync"
tags: []
signature: "pub fn sync < 'a > (root : & Path , edits : impl IntoIterator < Item = (& 'a Path , & 'a str) > , keep : & [PathBuf] ,) -> std :: io :: Result < usize >"
---

Synchronizes unsaved edits into recovery copies inside `root`.

Makes the folder hold exactly a copy of each unsaved edit inside `root`:
new and changed ones are written, the ones no longer unsaved are deleted
— but those in `keep`. The answer is how many copies there are now.

# Errors

Returns an I/O error if directory creation or file writing fails.

# Examples

```rust,no_run
use std::path::{Path, PathBuf};
use crawrad_project::recover::sync;

// Avoid writing files during doctests.
let res = sync(Path::new("."), [], &[]);
assert_eq!(res.unwrap(), 0);
```
