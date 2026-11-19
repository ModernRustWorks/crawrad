---
id: navigation-locked
kind: doc
target_file: src/navigation.rs
line_hint: 275
author: 
created: 
title: "locked"
tags: []
signature: "pub fn locked (file : & Path , roots : & [PathBuf] , generated_suffix : & str) -> bool"
---

Checks whether a file should open read-only in the editor.

**Does `file` open read-only?** The toolchain's and the downloaded
crates' sources ([`is_foreign`]), and the files the IDE generates from a
form (`generated_suffix`, the generator's own `.designer.rs`): an edit
there is lost at the next generation, so the form is where it changes.

# Examples

```rust
use crawrad_host::navigation::locked;
use std::path::Path;

assert!(locked(Path::new("form.designer.rs"), &[], ".designer.rs"));
assert!(!locked(Path::new("form.rs"), &[], ".designer.rs"));
```
