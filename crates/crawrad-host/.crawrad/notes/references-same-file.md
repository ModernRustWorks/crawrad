---
id: references-same-file
kind: doc
target_file: src/references.rs
line_hint: 236
author: 
created: 
title: "same_file"
tags: []
signature: "pub fn same_file (a : & Path , b : & Path) -> bool"
---

Compares two file paths for equivalence regardless of slash or case.

The same file? A path from the server and one from the editor rarely
agree on spelling (drive letter case, slashes), and Windows does not care.

# Examples

```rust
use crawrad_host::references::same_file;
use std::path::Path;

assert!(same_file(Path::new("src/main.rs"), Path::new("src\\MAIN.rs")));
```
