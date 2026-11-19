---
id: manifest-file
kind: doc
target_file: src/lib.rs
line_hint: 36
author: 
created: 
title: "MANIFEST_FILE"
tags: []
signature: "pub const MANIFEST_FILE : & str = \"manifest.json\""
---

The file name, and it is the same for every component: see ADR 0039.

# Examples

```rust
use crawrad_cli::MANIFEST_FILE;

assert_eq!(MANIFEST_FILE, "manifest.json");
```
