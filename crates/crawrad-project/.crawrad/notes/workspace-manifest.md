---
id: workspace-manifest
kind: doc
target_file: src/workspace.rs
line_hint: 64
author: 
created: 
title: "MANIFEST"
tags: []
signature: "pub const MANIFEST : & str = \"Cargo.toml\""
---

The Cargo manifest file name. Defined once to avoid casing variants.

# Examples

```rust
use crawrad_project::workspace::MANIFEST;

assert_eq!(MANIFEST, "Cargo.toml");
```
