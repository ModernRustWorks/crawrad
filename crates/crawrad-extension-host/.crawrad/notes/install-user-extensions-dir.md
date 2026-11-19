---
id: install-user-extensions-dir
kind: doc
target_file: src/install.rs
line_hint: 61
author: 
created: 
title: "user_extensions_dir"
tags: []
signature: "pub fn user_extensions_dir () -> PathBuf"
---

Where this user's extensions are installed, once for every project.

`%APPDATA%/CrawRAD/extensions`, else `~/.crawrad/extensions`; each one lives in `<id>@<version>`.

# Examples

```rust
use crawrad_extension_host::install::user_extensions_dir;

assert!(user_extensions_dir().ends_with("extensions"));
```
