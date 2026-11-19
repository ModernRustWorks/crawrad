---
id: assemble
kind: doc
target_file: src/lib.rs
line_hint: 39
author: 
created: 
title: "assemble"
tags: []
signature: "pub fn assemble (root : & Path) -> Result < String , String >"
---

**The whole file**, assembled from the `Cargo.toml` and the Rust source.

Nothing is compiled: `syn` reads the `.rs` as text.

# Examples

```rust
use std::path::Path;
use crawrad_cli::assemble;

let res = assemble(Path::new("non_existent_package_dir"));
assert!(res.is_err());
```

# Errors

Returns an error message string if reading `Cargo.toml`, traversing `src`, parsing, or finding components fails.
