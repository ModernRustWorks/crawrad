---
id: breakpoints-breakpoints-set-logpoint
kind: doc
target_file: src/breakpoints.rs
line_hint: 119
author: 
created: 
title: "Breakpoints::set_logpoint"
tags: []
signature: "pub fn set_logpoint (& mut self , file : & Path , line : u32 , message : & str)"
---

Sets a logpoint message for `line` of `file`.

Makes `line` of `file` a LOGPOINT: `message` is written when the line
runs, and the program does not stop. An empty message makes it a plain
breakpoint again.

# Examples

```rust
use std::path::Path;
use crawrad_host::Breakpoints;

let mut bp = Breakpoints::default();
bp.set_logpoint(Path::new("src/lib.rs"), 10, "log here");
assert_eq!(bp.log_message(Path::new("src/lib.rs"), 10), Some("log here"));
```
