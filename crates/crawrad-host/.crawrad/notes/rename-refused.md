---
id: rename-refused
kind: doc
target_file: src/rename.rs
line_hint: 79
author: 
created: 
title: "refused"
tags: []
signature: "pub fn refused (files : & [PathBuf] , project : & Path , foreign : & [PathBuf] , generated_suffix : & str ,) -> Option < String >"
---

[`forbidden`]'s files named for a sentence, or `None` when none is.

# Examples

```rust
use crawrad_host::rename::refused;
use std::path::{Path, PathBuf};

let files = vec![PathBuf::from("form.designer.rs")];
assert!(refused(&files, Path::new("."), &[], ".designer.rs").is_some());
```
