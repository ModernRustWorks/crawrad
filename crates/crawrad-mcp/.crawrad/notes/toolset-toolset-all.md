---
id: toolset-toolset-all
kind: doc
target_file: src/toolset.rs
line_hint: 71
author: 
created: 
title: "Toolset::all"
tags: []
signature: "pub fn all () -> Self"
---

Every tool: the bridge without `--groups`.

# Examples

```rust
use crawrad_mcp::toolset::Toolset;

let ts = Toolset::all();
assert_eq!(ts.describe(), "all");
```
