---
id: package-manager-searching
kind: doc
target_file: src/package_manager.rs
line_hint: 585
author: 
created: 
title: "SEARCHING"
tags: []
signature: "pub const SEARCHING : & str = \"Querying crates.io registry...\""
---

The status line while the search thread is out.

# Examples

```rust
use crawrad_project::package_manager::SEARCHING;

assert_eq!(SEARCHING, "Querying crates.io registry...");
```
