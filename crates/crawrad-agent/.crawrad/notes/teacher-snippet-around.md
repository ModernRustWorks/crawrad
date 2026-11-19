---
id: teacher-snippet-around
kind: doc
target_file: src/teacher.rs
line_hint: 104
author: 
created: 
title: "snippet_around"
tags: []
signature: "pub fn snippet_around (text : & str , line0 : usize , around : usize) -> String"
---

Extracts lines of text around a given 0-based line index.

The lines of `text` from `around` before `line0` (0-based) to `around`
after it, as the lesson's code excerpt.

# Examples

```rust
use crawrad_agent::teacher::snippet_around;

let code = "line 0\nline 1\nline 2\nline 3\nline 4";
let snippet = snippet_around(code, 2, 1);
assert_eq!(snippet, "line 1\nline 2\nline 3");
```
