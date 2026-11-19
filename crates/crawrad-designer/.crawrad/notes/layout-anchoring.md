---
id: layout-anchoring
kind: doc
target_file: src/layout.rs
line_hint: 318
author: 
created: 
title: "Anchoring"
tags: []
signature: "pub trait Anchoring : Sized"
---

Adds read, write, and readjust operations to anchors.

**What the IDE adds to the box part** — read, write and readjust.

A trait and not methods: the type is from another crate, and Rust's orphan rule
only lets the definition have inherent `impl`. The cost is an extra `use` in callers;
the return is the type being the SAME that goes to the user project.

# Examples

```rust
use crawrad_designer::layout::Anchoring;
use crawrad_component::Anchors;

let anchors = Anchors::read(None);
assert!(anchors.left);
```
