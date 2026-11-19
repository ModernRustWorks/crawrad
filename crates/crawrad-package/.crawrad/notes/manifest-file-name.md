---
id: manifest-file-name
kind: doc
target_file: src/manifest.rs
line_hint: 65
author: 
created: 
title: "FILE_NAME"
tags: []
signature: "pub const FILE_NAME : & str = \"manifest.json\""
---

The file name, and it is fixed.

Fixed because the IDE needs to find it inside a crate that it just
downloaded, without asking anyone anything. It is the same reason as `Cargo.toml`.

# Examples

```rust
use crawrad_package::manifest::FILE_NAME;

assert_eq!(FILE_NAME, "manifest.json");
```
