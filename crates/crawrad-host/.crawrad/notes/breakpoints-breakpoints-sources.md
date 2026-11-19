---
id: breakpoints-breakpoints-sources
kind: doc
target_file: src/breakpoints.rs
line_hint: 145
author: 
created: 
title: "Breakpoints::sources"
tags: []
signature: "pub fn sources (& self , file : & Path) -> Vec < crawrad_dap :: SourceBreakpoint >"
---

Returns the breakpoints of `file` for the debugger.

The breakpoints of `file` as the debugger takes them: each line, with
its message when it is a logpoint.

# Examples

```rust
use std::path::Path;
use crawrad_host::Breakpoints;

let bp = Breakpoints::default();
assert!(bp.sources(Path::new("src/lib.rs")).is_empty());
```
