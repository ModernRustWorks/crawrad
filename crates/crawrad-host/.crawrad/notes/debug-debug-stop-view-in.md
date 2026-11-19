---
id: debug-debug-stop-view-in
kind: doc
target_file: src/debug.rs
line_hint: 104
author: 
created: 
title: "Debug::stop_view_in"
tags: []
signature: "pub fn stop_view_in (& mut self , file : & Path , text : & str ,) -> (Option < usize > , Vec < (usize , String) >)"
---

Computes the execution line and inline values for the given file.

What the editor showing `file` (its `text`) draws for the stop: the
execution line and the values after the lines around it.

# Examples

```rust
use std::path::Path;
use crawrad_host::Debug;

let mut debug = Debug::default();
let (line, inline) = debug.stop_view_in(Path::new("src/main.rs"), "fn main() {}\n");
assert_eq!(line, None);
assert!(inline.is_empty());
```
