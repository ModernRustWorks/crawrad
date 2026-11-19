---
id: paging-slice-utf8-budget
kind: doc
target_file: src/paging.rs
line_hint: 4
author: 
created: 
title: "slice_utf8_budget"
tags: []
signature: "pub fn slice_utf8_budget (text : & str , offset : usize , limit : usize) -> (String , bool)"
---

Slices a string at byte boundary without splitting UTF-8 code points.

# Examples

```rust
use crawrad_mcp::paging::slice_utf8_budget;

let (slice, has_more) = slice_utf8_budget("hello world", 0, 5);
assert_eq!(slice, "hello");
assert!(has_more);
```
