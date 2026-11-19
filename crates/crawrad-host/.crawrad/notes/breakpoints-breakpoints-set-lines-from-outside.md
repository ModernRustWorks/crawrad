---
id: breakpoints-breakpoints-set-lines-from-outside
kind: doc
target_file: src/breakpoints.rs
line_hint: 91
author: 
created: 
title: "Breakpoints::set_lines_from_outside"
tags: []
signature: "pub fn set_lines_from_outside (& mut self , file : & Path , lines : impl IntoIterator < Item = u32 >)"
---

Replaces breakpoints from outside the editor without view overwrite.

[`Breakpoints::set_lines`] from outside the editor (the MCP): the
editor's view of `file` is handed these lines at its next
[`Breakpoints::sync_view`], instead of writing its own over them.

# Examples

```rust
use std::path::Path;
use crawrad_host::Breakpoints;

let mut bp = Breakpoints::default();
bp.set_lines_from_outside(Path::new("src/lib.rs"), [10]);
assert_eq!(bp.lines(Path::new("src/lib.rs")), vec![10]);
```
