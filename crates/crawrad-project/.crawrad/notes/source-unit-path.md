---
id: source-unit-path
kind: doc
target_file: src/source.rs
line_hint: 115
author: 
created: 
title: "unit_path"
tags: []
signature: "pub fn unit_path (form : & Path) -> PathBuf"
---

The path of the unit where the handlers of this form live.

# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_project::source::unit_path;

assert_eq!(unit_path(Path::new("form.crawform")), PathBuf::from("form.rs"));
```
