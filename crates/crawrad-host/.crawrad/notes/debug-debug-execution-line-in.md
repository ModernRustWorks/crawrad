---
id: debug-debug-execution-line-in
kind: doc
target_file: src/debug.rs
line_hint: 295
author: 
created: 
title: "Debug::execution_line_in"
tags: []
signature: "pub fn execution_line_in (& self , file : & Path) -> Option < usize >"
---

The zero-based line `file` shows the program on when stopped.

The zero-based line `file` shows the program on, when it is stopped in
that file — the editor's yellow arrow.

# Examples

```rust
use std::path::Path;
use crawrad_host::Debug;

let debug = Debug::default();
assert_eq!(debug.execution_line_in(Path::new("src/main.rs")), None);
```
