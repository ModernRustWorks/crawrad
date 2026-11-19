---
id: installed-from-folder
kind: doc
target_file: src/installed.rs
line_hint: 154
author: 
created: 
title: "from_folder"
tags: []
signature: "pub fn from_folder (folder : & Path) -> InstalledPackages"
---

Loads the builtin catalogue plus packages found in the specified folder.

**Builtin catalogue PLUS whatever is in a folder.**

This is how the installation gate checks the outcome: installing and having the
IDE not find it is the worst these two gates can do together, and the only way
to measure that is to actually load what the installation left behind.

# Examples

```rust
use std::path::Path;
use crawrad_project::installed::from_folder;

let cat = from_folder(Path::new("missing_folder"));
assert!(cat.catalog.all().count() > 0);
```
