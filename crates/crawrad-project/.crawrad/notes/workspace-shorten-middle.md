---
id: workspace-shorten-middle
kind: doc
target_file: src/workspace.rs
line_hint: 355
author: 
created: 
title: "shorten_middle"
tags: []
signature: "pub fn shorten_middle (text : & str , max : usize) -> String"
---

A long path cut in the middle so a Welcome row stays on one line.

# Examples

```rust
use crawrad_project::workspace::shorten_middle;

assert_eq!(shorten_middle("abcdefghij", 5), "ab…ij");
assert_eq!(shorten_middle("short", 10), "short");
```
