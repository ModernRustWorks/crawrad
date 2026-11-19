---
id: navigation-foreign-roots
kind: doc
target_file: src/navigation.rs
line_hint: 245
author: 
created: 
title: "foreign_roots"
tags: []
signature: "pub fn foreign_roots () -> Vec < PathBuf >"
---

Returns directories containing external toolchain and dependency sources.

The folders whose files are not the user's to edit: the toolchain and
cargo's downloads. `RUSTUP_HOME` / `CARGO_HOME` when set, else the
defaults under the user's home.

# Examples

```rust
use crawrad_host::navigation::foreign_roots;

let roots = foreign_roots();
assert!(!roots.is_empty());
```
