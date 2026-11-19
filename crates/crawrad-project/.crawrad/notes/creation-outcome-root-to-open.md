---
id: creation-outcome-root-to-open
kind: doc
target_file: src/creation.rs
line_hint: 174
author: 
created: 
title: "Outcome::root_to_open"
tags: []
signature: "pub fn root_to_open (& self) -> Option < & Path >"
---

The directory to open now, if any.

**Both arms that say "opened" open**, and the revision of 08/09/2026
measured the cost of forgetting the second: `root_to_open() == None` with
the full, compilable project on disk.

# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_project::creation::Outcome;

let out = Outcome::Opened(PathBuf::from("/tmp"));
assert_eq!(out.root_to_open(), Some(Path::new("/tmp")));
```
