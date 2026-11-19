---
id: debug-debug-inline-values-in
kind: doc
target_file: src/debug.rs
line_hint: 116
author: 
created: 
title: "Debug::inline_values_in"
tags: []
signature: "pub fn inline_values_in (& mut self , file : & Path , text : & str) -> Vec < (usize , String) >"
---

Computes inline values to show after lines in the given file.

The values to write after the lines of `file` (its `text`), when the
frame the panel shows stands in it — see [`crate::inline_values`]. The
paths are asked of the session once per stop and frame; until the
answer comes, nothing is written.

# Examples

```rust
use std::path::Path;
use crawrad_host::Debug;

let mut debug = Debug::default();
let values = debug.inline_values_in(Path::new("src/main.rs"), "let x = 1;\n");
assert!(values.is_empty());
```
