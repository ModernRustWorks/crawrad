---
id: hostmodel-breakpoints-json
kind: doc
target_file: src/lib.rs
line_hint: 287
author: 
created: 
title: "HostModel::breakpoints_json"
tags: []
signature: "pub fn breakpoints_json (& self) -> serde_json :: Value"
---

Every breakpoint, file by file, for the MCP.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert!(host.breakpoints_json().as_object().unwrap().is_empty());
```
