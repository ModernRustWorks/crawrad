---
id: creation-outcome-closes-automatically
kind: doc
target_file: src/creation.rs
line_hint: 183
author: 
created: 
title: "Outcome::closes_automatically"
tags: []
signature: "pub fn closes_automatically (& self) -> bool"
---

Returns whether the dialog can close automatically.

Can the dialog close automatically? Only creation that has **nothing to say**.

# Examples

```rust
use std::path::PathBuf;
use crawrad_project::creation::Outcome;

let out = Outcome::Opened(PathBuf::from("."));
assert!(out.closes_automatically());
```
