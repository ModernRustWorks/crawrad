---
id: render-present
kind: doc
target_file: src/render.rs
line_hint: 18
author: 
created: 
title: "present"
tags: []
signature: "pub fn present (var : & Variable , cx : & Cx < '_ > , depth : usize) -> Option < String >"
---

The readable text of `var`, or `None` when its shape is not known here.

# Examples

```rust,ignore
// private module: doctests cannot reach items in non-public modules
use crawrad_dap_rust::render::present;
```
