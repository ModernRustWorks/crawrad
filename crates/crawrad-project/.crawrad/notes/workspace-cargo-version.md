---
id: workspace-cargo-version
kind: doc
target_file: src/workspace.rs
line_hint: 1658
author: 
created: 
title: "cargo_version"
tags: []
signature: "pub fn cargo_version (output : & str) -> Option < (u32 , u32) >"
---

Parses `(major, minor)` version from `cargo --version` output.

`(major, minor)` from what `cargo --version` prints: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`.
`None` for anything else — the check is then skipped and `cargo new` speaks for itself.

# Examples

```rust
use crawrad_project::workspace::cargo_version;

assert_eq!(cargo_version("cargo 1.95.0 (f2d3ce0bd 2026-03-21)"), Some((1, 95)));
assert_eq!(cargo_version("not cargo"), None);
```
