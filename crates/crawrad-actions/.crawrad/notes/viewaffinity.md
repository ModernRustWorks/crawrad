---
id: viewaffinity
kind: doc
target_file: src/lib.rs
line_hint: 1151
author: 
created: 
title: "ViewAffinity"
tags: []
signature: "pub enum ViewAffinity"
---

The view affinity of an action in the catalogue (ADR 0029 / Issue #178).

# Examples

```rust
use crawrad_actions::ViewAffinity;

let affinity = ViewAffinity::Any;
assert_eq!(affinity, ViewAffinity::Any);
```
