---
id: breakpoints-breakpoints-all-sources
kind: doc
target_file: src/breakpoints.rs
line_hint: 158
author: 
created: 
title: "Breakpoints::all_sources"
tags: []
signature: "pub fn all_sources (& self) -> BTreeMap < PathBuf , Vec < crawrad_dap :: SourceBreakpoint > >"
---

Every file's [`Breakpoints::sources`] — what a debug session starts with.

# Examples

```rust
use crawrad_host::Breakpoints;

let bp = Breakpoints::default();
assert!(bp.all_sources().is_empty());
```
