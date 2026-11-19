---
id: workspace-shorten-end
kind: doc
target_file: src/workspace.rs
line_hint: 1
author: 
created: 
title: "shorten_end"
tags: []
signature: "pub fn shorten_end (text : & str , max : usize) -> String"
---

Cuts a name to `max` characters, ending in "…".

# Examples

```rust
use crawrad_project::workspace::shorten_end;
assert_eq!(shorten_end("app", 5), "app");
assert_eq!(shorten_end("a_very_long_name", 6), "a_ver…");
```
