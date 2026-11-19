---
id: rename-forbidden
kind: doc
target_file: src/rename.rs
line_hint: 91
author: 
created: 
title: "forbidden"
tags: []
signature: "pub fn forbidden (files : & [PathBuf] , project : & Path , foreign : & [PathBuf] , generated_suffix : & str ,) -> Vec < PathBuf >"
---

Identifies files involved in a rename that must not be modified.

The files of a rename it may not touch: a generated form source, a file
outside `project`, or one of the toolchain's / downloaded crates'.

# Examples

```rust
use crawrad_host::rename::forbidden;
use std::path::{Path, PathBuf};

let files = vec![PathBuf::from("form.designer.rs")];
let bad = forbidden(&files, Path::new("."), &[], ".designer.rs");
assert_eq!(bad.len(), 1);
```
