---
id: desktop-desktop-with-tabs
kind: doc
target_file: src/desktop.rs
line_hint: 73
author: 
created: 
title: "Desktop::with_tabs"
tags: []
signature: "pub fn with_tabs (mut self , root : & Path , files : & [PathBuf] , active : Option < usize > , caret : impl Fn (& Path) -> Option < (usize , usize) > ,) -> Self"
---

Populates the desktop with tab places relative to the project root.

The tabs of `files` that live in `root`, relative to it, each with the
caret `caret` knows (else the top), and `active` mapped onto them.

# Examples

```rust
use std::path::{Path, PathBuf};
use crawrad_project::desktop::Desktop;

let d = Desktop::default().with_tabs(Path::new("."), &[], None, |_| None);
assert!(d.tabs.is_empty());
```
