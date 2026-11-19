---
id: profile-folder
kind: doc
target_file: src/profile.rs
line_hint: 41
author: 
created: 
title: "folder"
tags: []
signature: "pub fn folder () -> Option < PathBuf >"
---

Where the profiles live; `None` without a configuration folder.

# Examples

```rust
use crawrad_project::profile::folder;

if let Some(path) = folder() {
    assert!(path.ends_with("profiles"));
}
```
