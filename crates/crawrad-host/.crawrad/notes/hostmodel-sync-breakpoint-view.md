---
id: hostmodel-sync-breakpoint-view
kind: doc
target_file: src/lib.rs
line_hint: 229
author: 
created: 
title: "HostModel::sync_breakpoint_view"
tags: []
signature: "pub fn sync_breakpoint_view (& mut self , file : & std :: path :: Path , view_owner : Option < & std :: path :: Path > , view_lines : & [usize] ,) -> ViewSync"
---

Reconciles editor breakpoints with session breakpoints.

Reconciles the editor's view of `file` with the session's breakpoints
([`Breakpoints::sync_view`]); a change made while a program is being
debugged reaches the debugger at once.

# Examples

```rust
use std::path::Path;
use crawrad_host::{HostModel, ViewSync};

let mut host = HostModel::new();
let file = Path::new("src/lib.rs");
assert_eq!(host.sync_breakpoint_view(file, Some(file), &[]), ViewSync::Same);
```
