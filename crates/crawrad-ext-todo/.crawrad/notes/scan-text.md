---
id: scan-text
kind: doc
target_file: src/lib.rs
line_hint: 63
author: 
created: 
title: "scan_text"
tags: []
signature: "pub fn scan_text (text : & str) -> Vec < (u32 , u32 , u32 , & 'static str , String) >"
---

Scans text for tagged comments like TODO or FIXME.

The tagged comments of one file's text: `(line, start, end, kind, text)`.
A tag is in capitals, a word of its own, inside a comment.

# Examples

```rust
use crawrad_ext_todo::scan_text;

let todos = scan_text("// TODO: fix this\nlet x = 1;");
assert_eq!(todos.len(), 1);
assert_eq!(todos[0].3, "todo");
```
