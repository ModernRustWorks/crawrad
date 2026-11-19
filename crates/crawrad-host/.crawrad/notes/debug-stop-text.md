---
id: debug-stop-text
kind: doc
target_file: src/debug.rs
line_hint: 47
author: 
created: 
title: "stop_text"
tags: []
signature: "pub fn stop_text (file : & Path , line : u32 , reason : & str) -> String"
---

The status bar's sentence for a stop: `stopped at form_main.rs:12 (breakpoint)`.

# Examples

```rust
use std::path::Path;
use crawrad_host::debug::stop_text;

let text = stop_text(Path::new("src/main.rs"), 12, "breakpoint");
assert_eq!(text, "stopped at main.rs:12 (breakpoint)");
```
