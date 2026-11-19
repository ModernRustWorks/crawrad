---
id: debug-panel-tail
kind: doc
target_file: src/debug_panel.rs
line_hint: 208
author: 
created: 
title: "tail"
tags: []
signature: "pub fn tail (text : & str , max : usize) -> String"
---

The last `max` characters of `text`, led by `…` when something was cut.

# Examples

```rust
use crawrad_host::debug_panel::tail;

assert_eq!(tail("abcdef", 4), "…def");
```
