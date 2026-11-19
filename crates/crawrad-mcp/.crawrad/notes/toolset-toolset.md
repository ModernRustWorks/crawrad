---
id: toolset-toolset
kind: doc
target_file: src/toolset.rs
line_hint: 62
author: 
created: 
title: "Toolset"
tags: []
signature: "pub struct Toolset"
---

The tools a bridge lets through: every tool, or those of some groups.

# Examples

```rust
use crawrad_mcp::toolset::Toolset;

let ts = Toolset::all();
assert!(ts.allows("editor.apply_edit"));
```
