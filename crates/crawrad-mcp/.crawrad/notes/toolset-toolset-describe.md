---
id: toolset-toolset-describe
kind: doc
target_file: src/toolset.rs
line_hint: 100
author: 
created: 
title: "Toolset::describe"
tags: []
signature: "pub fn describe (& self) -> String"
---

The group names, comma-joined; `all` when unfiltered.

# Examples

```rust
use crawrad_mcp::toolset::Toolset;

let ts = Toolset::parse("code").unwrap();
assert_eq!(ts.describe(), "code");
```
