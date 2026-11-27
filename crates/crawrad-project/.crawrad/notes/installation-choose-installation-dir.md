---
id: installation-choose-installation-dir
kind: doc
target_file: src/installation.rs
line_hint: 262
author: 
created: 
title: "choose_installation_dir"
tags: []
signature: "pub fn choose_installation_dir (exe_dir : & Path , config : Option < & Path >) -> Option < PathBuf >"
---

Picks the folder components are installed into.

**NEXT TO THE EXE, UNLESS THE STORE OWNS IT.**

A normal install keeps components beside the executable. A Store install
cannot write there, so components go to `<config folder>/components`, which
[`crate::installed`] also scans. Without a config folder a Store install has
nowhere to write and the answer is `None`.

# Examples

```rust
use std::path::Path;
use crawrad_project::installation::choose_installation_dir;

// Forward slash on purpose: the backslash only separates a path on Windows,
// and this example runs on every platform.
let store = Path::new("/Program Files/WindowsApps/CrawRAD");
let config = Path::new("/Users/me/AppData/Roaming/CrawRAD");
assert_eq!(choose_installation_dir(store, Some(config)), Some(config.join("components")));
```
