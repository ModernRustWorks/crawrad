---
id: installation-installation-dir
kind: doc
target_file: src/installation.rs
line_hint: 229
author: 
created: 
title: "installation_dir"
tags: []
signature: "pub fn installation_dir () -> Option < PathBuf >"
---

Returns the path where the IDE installs component packages.

**WHERE THE IDE INSTALLS.**

Next to the executable, which is the first place [`crate::installed`] scans —
in development this is `target/debug/`, which leaves the path exercisable
without installing the IDE and without dirtying the repository.

# Examples

```rust
use crawrad_project::installation::installation_dir;

let _ = installation_dir();
```
