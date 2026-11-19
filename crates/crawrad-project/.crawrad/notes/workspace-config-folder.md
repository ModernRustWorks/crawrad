---
id: workspace-config-folder
kind: doc
target_file: src/workspace.rs
line_hint: 237
author: 
created: 
title: "config_folder"
tags: []
signature: "pub fn config_folder () -> Option < PathBuf >"
---

The user's configuration directory, created if it does not exist.

`%APPDATA%\CrawRAD` on Windows, `$XDG_CONFIG_HOME/crawrad` (or
`~/.config/crawrad`) on others. No crate for this: three environment variables
and one join, and an extra dependency in the shell costs more than these ten lines.

`None` when neither `APPDATA` nor `HOME` exist — happens in services and
containers. The IDE continues working without recents; what it does not do is
invent a write location on someone's disk.

# Examples

```rust
use crawrad_project::workspace::config_folder;

if let Some(path) = config_folder() {
    assert!(path.is_dir());
}
```
