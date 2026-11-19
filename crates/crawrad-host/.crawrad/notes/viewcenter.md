---
id: viewcenter
kind: doc
target_file: src/lib.rs
line_hint: 47
author: 
created: 
title: "ViewCenter"
tags: []
signature: "pub enum ViewCenter"
---

Which panel occupies the centre of the IDE window.

This is the canonical type — `crawrad-ide` uses it as its `Center` so that
the rest of the codebase needs no conversion.

# Examples

```rust
use crawrad_host::ViewCenter;

let center = ViewCenter::Design;
assert_eq!(center.as_str(), "design");
```
