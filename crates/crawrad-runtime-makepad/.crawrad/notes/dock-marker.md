---
id: dock-marker
kind: doc
target_file: src/dock.rs
line_hint: 18
author: 
created: 
title: "marker"
tags: []
signature: "pub fn marker () -> String"
---

The first line of the file — the version marker that gates the rest.

# Examples

```rust
use crawrad_runtime_makepad::dock::marker;

assert!(marker().starts_with("crawrad-dock-layout"));
```
