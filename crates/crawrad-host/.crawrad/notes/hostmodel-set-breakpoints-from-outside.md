---
id: hostmodel-set-breakpoints-from-outside
kind: doc
target_file: src/lib.rs
line_hint: 247
author: 
created: 
title: "HostModel::set_breakpoints_from_outside"
tags: []
signature: "pub fn set_breakpoints_from_outside (& mut self , file : & std :: path :: Path , lines : & [u32])"
---

Sets breakpoint lines from external callers like the MCP.

Breakpoints set from outside the editor (the MCP's
`debug.set_breakpoints`): the view is handed them, and a program being
debugged gets them at once.

# Examples

```rust
use std::path::Path;
use crawrad_host::HostModel;

let mut host = HostModel::new();
host.set_breakpoints_from_outside(Path::new("src/lib.rs"), &[10]);
assert_eq!(host.breakpoints().lines(Path::new("src/lib.rs")), vec![10]);
```
