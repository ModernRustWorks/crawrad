---
id: toolset-toolset-allows
kind: doc
target_file: src/toolset.rs
line_hint: 109
author: 
created: 
title: "Toolset::allows"
tags: []
signature: "pub fn allows (& self , name : & str) -> bool"
---

Whether the tool `name` is in this toolset.

# Examples

```rust
use crawrad_mcp::toolset::Toolset;

let ts = Toolset::parse("design").unwrap();
assert!(ts.allows("designer.add"));
assert!(!ts.allows("editor.cut"));
```
