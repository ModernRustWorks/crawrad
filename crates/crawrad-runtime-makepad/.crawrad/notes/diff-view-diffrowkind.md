---
id: diff-view-diffrowkind
kind: doc
target_file: src/diff_view.rs
line_hint: 149
author: 
created: 
title: "DiffRowKind"
tags: []
signature: "pub enum DiffRowKind"
---

What kind of difference or match a side-by-side row represents.

# Examples

```rust
use crawrad_runtime_makepad::diff_view::DiffRowKind;

assert_ne!(DiffRowKind::Same, DiffRowKind::Changed);
```
