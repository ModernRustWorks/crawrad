---
id: breakpoints-viewsync
kind: doc
target_file: src/breakpoints.rs
line_hint: 203
author: 
created: 
title: "ViewSync"
tags: []
signature: "pub enum ViewSync"
---

What [`Breakpoints::sync_view`] decided.

# Examples

```rust
use crawrad_host::ViewSync;

let sync = ViewSync::Same;
assert_eq!(sync, ViewSync::Same);
```
